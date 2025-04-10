use anyhow::Result;
use deno_core::error::AnyError;
use deno_runtime::deno_core::{self, op};
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use std::path::Path;

pub struct TypescriptRuntime {
    runtime: MainWorker,
}

impl TypescriptRuntime {
    pub fn new() -> Result<Self> {
        let options = deno_runtime::worker::WorkerOptions {
            module_loader: std::sync::Arc::new(deno_runtime::SourceLoader),
            fs: std::sync::Arc::new(deno_runtime::FsInner),
            permissions: Permissions::allow_all(),
            ..Default::default()
        };

        let runtime = MainWorker::bootstrap_from_options(options);
        Ok(Self { runtime })
    }

    pub async fn process_content(&mut self, content: &str, meta: &PageMeta) -> Result<String> {
        // Initialize the runtime with your TypeScript processing code
        let js_code = format!(
            r#"
            const content = `{}`;
            const meta = {};
            // Your TypeScript processing logic here
            "#,
            content,
            serde_json::to_string(meta)?
        );

        let result = self.runtime.execute_script("[native]", &js_code)?;
        // Handle the result and return processed content
        Ok(result.to_string())
    }
}
