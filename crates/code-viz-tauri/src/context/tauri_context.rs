use anyhow::Result;
use async_trait::async_trait;
use code_viz_core::traits::AppContext;
use serde_json::{json, Value};
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};

/// Tauri implementation of AppContext that wraps a Tauri AppHandle.
/// This context delegates event emission to Tauri's IPC system.
pub struct TauriContext {
    app: AppHandle,
}

impl TauriContext {
    /// Create a new TauriContext with the given AppHandle.
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

#[async_trait]
impl AppContext for TauriContext {
    async fn emit_event(&self, event: &str, payload: Value) -> Result<()> {
        self.app.emit(event, payload)?;
        Ok(())
    }

    fn get_app_dir(&self) -> PathBuf {
        self.app
            .path()
            .app_data_dir()
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default())
    }

    async fn report_progress(&self, percentage: f32, message: &str) -> Result<()> {
        self.emit_event(
            "progress",
            json!({
                "percentage": percentage,
                "message": message
            }),
        )
        .await
    }
}
