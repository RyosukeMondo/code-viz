use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

/// AppContext abstracts external dependencies like event emission, file system access,
/// and progress reporting. This allows business logic to be decoupled from the
/// underlying platform (Tauri, CLI, or Tests).
#[async_trait]
pub trait AppContext: Send + Sync {
    /// Emit an event to the frontend or listener.
    ///
    /// # Arguments
    /// * `event` - The name of the event.
    /// * `payload` - The JSON payload associated with the event.
    async fn emit_event(&self, event: &str, payload: Value) -> Result<()>;

    /// Get the application data directory.
    ///
    /// Returns the path to the directory where the application can store its data.
    /// In CLI mode, this might be the current directory or a specific config directory.
    fn get_app_dir(&self) -> PathBuf;

    /// Report progress of a long-running operation.
    ///
    /// # Arguments
    /// * `percentage` - Progress percentage (0.0 to 1.0).
    /// * `message` - A human-readable message describing the current progress.
    async fn report_progress(&self, percentage: f32, message: &str) -> Result<()>;
}
