use anyhow::Result;
use async_trait::async_trait;
use code_viz_core::traits::AppContext;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Mock implementation of AppContext for unit testing.
/// Captures emitted events in a thread-safe vector for later verification.
#[derive(Clone)]
pub struct MockContext {
    events: Arc<Mutex<Vec<(String, Value)>>>,
    app_dir: PathBuf,
}

impl MockContext {
    /// Create a new MockContext with default temp directory.
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            app_dir: std::env::temp_dir(),
        }
    }

    /// Create a new MockContext with a specific app directory.
    pub fn with_app_dir(path: PathBuf) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            app_dir: path,
        }
    }

    /// Get a clone of all captured events.
    pub fn get_events(&self) -> Vec<(String, Value)> {
        self.events.lock().unwrap().clone()
    }

    /// Assert that an event with the given name was emitted.
    /// Panics with a descriptive message if the event was not found.
    pub fn assert_event_emitted(&self, event_name: &str) {
        let events = self.get_events();
        assert!(
            events.iter().any(|(name, _)| name == event_name),
            "Expected event '{}' to be emitted, but it was not. Captured events: {:?}",
            event_name,
            events
        );
    }

    /// Clear all captured events.
    pub fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }
}

impl Default for MockContext {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AppContext for MockContext {
    async fn emit_event(&self, event: &str, payload: Value) -> Result<()> {
        self.events.lock().unwrap().push((event.to_string(), payload));
        Ok(())
    }

    fn get_app_dir(&self) -> PathBuf {
        self.app_dir.clone()
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
