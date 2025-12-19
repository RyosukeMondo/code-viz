use anyhow::Result;
use async_trait::async_trait;
use code_viz_core::traits::AppContext;
use serde_json::Value;
use std::path::PathBuf;

/// CLI implementation of AppContext that prints to stdout/stderr.
#[derive(Clone)]
pub struct CliContext {
    verbose: bool,
}

impl CliContext {
    /// Create a new CliContext with the given verbosity.
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Create a new non-verbose CliContext.
    pub fn new_normal() -> Self {
        Self { verbose: false }
    }

    /// Create a new verbose CliContext.
    pub fn new_verbose() -> Self {
        Self { verbose: true }
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
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }

    async fn report_progress(&self, percentage: f32, message: &str) -> Result<()> {
        // Report progress to stderr to keep stdout clean for JSON output or other data.
        eprintln!("{:.0}% - {}", percentage * 100.0, message);
        Ok(())
    }
}