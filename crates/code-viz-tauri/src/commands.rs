//! Tauri command definitions - thin wrappers around shared API
//!
//! This module contains Tauri IPC commands that wrap the shared code-viz-api handlers.
//! All business logic lives in code-viz-api (SSOT), these are just transport adapters.

use crate::models::TreeNode;
use crate::context::{TauriContext, RealFileSystem, RealGit};
use code_viz_dead_code::DeadCodeResult;

/// Analyze a repository - Tauri IPC wrapper
///
/// This command is a thin wrapper around code_viz_api::analyze_repository_handler.
/// It provides Tauri-specific context and converts the result to Tauri's TreeNode type.
#[tauri::command]
#[specta::specta]
pub async fn analyze_repository(
    app: tauri::AppHandle,
    path: String,
    request_id: Option<String>,
) -> Result<TreeNode, String> {
    let ctx = TauriContext::new(app);
    let fs = RealFileSystem::new();

    // Call the shared SSOT handler
    let api_tree = code_viz_api::analyze_repository_handler(ctx, fs, path, request_id)
        .await
        .map_err(|e| e.to_user_message())?;

    // Convert API TreeNode to Tauri TreeNode (adds specta Type for TS generation)
    Ok(api_tree.into())
}

/// Analyze dead code - Tauri IPC wrapper
///
/// This command is a thin wrapper around code_viz_api::analyze_dead_code_handler.
#[tauri::command]
#[specta::specta]
pub async fn analyze_dead_code_command(
    app: tauri::AppHandle,
    path: String,
    min_confidence: u8,
    request_id: Option<String>,
) -> Result<DeadCodeResult, String> {
    let ctx = TauriContext::new(app);
    let fs = RealFileSystem::new();
    let git = RealGit::new();

    // Call the shared SSOT handler
    code_viz_api::analyze_dead_code_handler(ctx, fs, git, path, min_confidence, request_id)
        .await
        .map_err(|e| e.to_user_message())
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use serde_json;
    use std::env;

    /// Verify Tauri TreeNode serialization matches code-viz-api contract
    #[test]
    fn test_tree_node_contract_consistency() {
        use std::time::{SystemTime, UNIX_EPOCH};
        use std::path::PathBuf;

        // Create both API and Tauri tree nodes with same data
        let api_node = code_viz_api::TreeNode {
            id: "test.rs".to_string(),
            name: "test.rs".to_string(),
            path: PathBuf::from("test.rs"),
            loc: 100,
            complexity: 10,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1234567890),
            dead_code_ratio: None,
        };

        let tauri_node: TreeNode = api_node.clone().into();

        // Both should serialize to identical JSON
        let api_json = serde_json::to_value(&api_node).unwrap();
        let tauri_json = serde_json::to_value(&tauri_node).unwrap();

        assert_eq!(api_json["id"], tauri_json["id"]);
        assert_eq!(api_json["loc"], tauri_json["loc"]);
        assert_eq!(api_json["lastModified"], tauri_json["lastModified"]);

        // CRITICAL: Both must serialize lastModified as ISO 8601 string
        assert!(api_json["lastModified"].is_string());
        assert!(tauri_json["lastModified"].is_string());
    }
}
