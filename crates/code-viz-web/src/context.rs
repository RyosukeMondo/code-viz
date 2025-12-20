//! Web-specific context implementations
//!
//! Implements the trait-based dependency injection for the web server.

use async_trait::async_trait;
use code_viz_core::traits::AppContext;
use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;

// Re-export shared implementations from code-viz-core
pub use code_viz_core::context::{RealFileSystem, RealGit};

/// Web application context
#[derive(Clone)]
pub struct WebContext;

impl WebContext {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AppContext for WebContext {
    async fn emit_event(&self, event: &str, payload: Value) -> Result<()> {
        // For web, we could:
        // - Log events
        // - Send to WebSocket clients
        // - Store in database
        tracing::debug!(event = %event, payload = %payload, "Event emitted");
        Ok(())
    }

    fn get_app_dir(&self) -> PathBuf {
        // For web, use system temp dir or config dir
        std::env::temp_dir().join("code-viz-web")
    }

    async fn report_progress(&self, percentage: f32, message: &str) -> Result<()> {
        tracing::info!(percentage = %percentage, message = %message, "Progress update");
        // Could emit SSE (Server-Sent Events) for real-time progress
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use code_viz_core::traits::FileSystem;

    #[tokio::test]
    async fn test_web_context() {
        let ctx = WebContext::new();
        let result = ctx.emit_event("test", serde_json::json!({"key": "value"})).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_real_filesystem() {
        let fs = RealFileSystem::new();
        let temp_dir = std::env::temp_dir();
        assert!(fs.exists(&temp_dir));
    }
}
