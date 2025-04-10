use anyhow::Result;
use tera::{Context, Tera};

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(templates_dir: &str) -> Result<Self> {
        let tera = Tera::new(&format!("{}/**/*.html", templates_dir))?;
        Ok(Self { tera })
    }

    pub fn render(&self, template: &str, context: &Context) -> Result<String> {
        Ok(self.tera.render(template, context)?)
    }
}
