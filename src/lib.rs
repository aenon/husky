use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub input_dir: String,
    pub output_dir: String,
    pub templates_dir: String,
    pub assets_dir: String,
    pub default_template: String,
    pub site_name: String,
    pub base_url: String,
}

#[derive(Debug)]
pub struct Page {
    pub content: String,
    pub frontmatter: PageMeta,
    pub route: String,
    pub template: String,
}

#[derive(Debug, Deserialize)]
pub struct PageMeta {
    pub title: String,
    pub template: Option<String>,
    pub date: Option<String>,
    // Add other metadata fields
}

#[async_trait]
pub trait Generator {
    async fn run(&self) -> Result<()>;
}

pub struct StaticSiteGenerator {
    config: Config,
    ts_runtime: TypescriptRuntime,
}

impl StaticSiteGenerator {
    pub fn new(config_path: &str) -> Result<Self> {
        let config = Self::load_config(config_path)?;
        let ts_runtime = TypescriptRuntime::new()?;
        Ok(Self { config, ts_runtime })
    }

    fn load_config(path: &str) -> Result<Config> {
        let config_str = std::fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&config_str)?)
    }

    fn discover_content(&self) -> Result<Vec<PathBuf>> {
        let mut content_files = Vec::new();
        for entry in walkdir::WalkDir::new(&self.config.input_dir) {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "md") {
                content_files.push(entry.path().to_owned());
            }
        }
        Ok(content_files)
    }

    fn parse_markdown(&self, content: &str) -> Result<(PageMeta, String)> {
        // Split frontmatter and content
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() != 3 {
            anyhow::bail!("Invalid markdown format: missing frontmatter");
        }

        let frontmatter: PageMeta = serde_yaml::from_str(parts[1])?;
        let markdown = parts[2];

        // Parse markdown
        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);
        options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
        options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);

        let parser = pulldown_cmark::Parser::new_ext(markdown, options);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        Ok((frontmatter, html))
    }
}

#[async_trait]
impl Generator for StaticSiteGenerator {
    async fn run(&self) -> Result<()> {
        // 1. Discover content
        let content_files = self.discover_content()?;

        // 2. Process each file
        for file in content_files {
            let content = std::fs::read_to_string(&file)?;
            let (meta, html) = self.parse_markdown(&content)?;

            // 3. Generate route from file path
            let route = self.generate_route(&file)?;

            // 4. Process with TypeScript (if needed)
            let processed_content = self.ts_runtime.process_content(&html, &meta).await?;

            // 5. Apply template
            let page = Page {
                content: processed_content,
                frontmatter: meta,
                route,
                template: meta.template.unwrap_or(self.config.default_template.clone()),
            };
            self.render_page(&page)?;
        }

        // 6. Copy assets
        self.copy_assets()?;

        Ok(())
    }
}
