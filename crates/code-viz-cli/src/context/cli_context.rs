use anyhow::Result;
use async_trait::async_trait;
use code_viz_core::traits::AppContext;
use serde_json::Value;
use std::path::PathBuf;

/// CLI implementation of AppContext that prints to stdout.
pub struct CliContext {
    verbose: bool,
}

impl CliContext {
    /// Create a new CliContext.
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

#[async_trait]
impl AppContext for CliContext {
    async fn emit_event(&self, event: &str, payload: Value) -> Result<()> {
        if self.verbose {
            println!("[{}] {}", event, payload);
        }
        Ok(())
    }

    fn get_app_dir(&self) -> PathBuf {
        std::env::current_dir().unwrap_or_default()
    }

    async fn report_progress(&self, percentage: f32, message: &str) -> Result<()> {
        // For CLI, we just print the progress to stdout.
        // In a more advanced implementation, this could use a progress bar crate like 'indicatif'.
        println!("{}: {:.1}%", message, percentage * 100.0);
        Ok(())
    }
}
