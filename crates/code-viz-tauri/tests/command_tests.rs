//! Integration tests for Tauri commands using MockContext
//!
//! These tests verify that the shared code-viz-api handlers work correctly
//! when called from Tauri commands.

use code_viz_tauri::context::RealFileSystem;
use code_viz_core::mocks::MockContext;
use std::env;

/// Unit test for analyze_repository using shared API handler with MockContext.
/// This test verifies that the command logic correctly reports progress
/// and returns the analysis results without requiring a full Tauri runtime.
#[tokio::test]
async fn test_analyze_repository_with_mock_context() {
    // 1. Setup MockContext and RealFileSystem
    let ctx = MockContext::new();
    let fs = RealFileSystem::new();

    // 2. Prepare arguments (use current directory for analysis)
    let current_dir = env::current_dir()
        .expect("Failed to get current directory")
        .to_string_lossy()
        .to_string();

    // 3. Execute the shared API handler (SSOT)
    let result = code_viz_api::analyze_repository_handler(
        ctx.clone(),
        fs,
        current_dir,
        Some("test-request-id".to_string())
    ).await;

    // 4. Verify results
    assert!(result.is_ok(), "Command failed: {:?}", result.err());
    let tree = result.unwrap();
    assert!(!tree.name.is_empty());

    // 5. Verify event emissions via MockContext
    let events = ctx.get_events();

    // Check for progress events
    let progress_events: Vec<_> = events.iter()
        .filter(|(name, _)| name == "progress")
        .collect();

    // We expect at least 5 progress reports based on the implementation
    assert!(progress_events.len() >= 5, "Expected at least 5 progress events, got {}", progress_events.len());

    // Verify the first progress report
    let first_progress = progress_events.first().unwrap().1.as_object().unwrap();
    let first_percentage = first_progress["percentage"].as_f64().unwrap();
    assert!((first_percentage - 0.1).abs() < 1e-6);
    assert_eq!(first_progress["message"].as_str().unwrap(), "Scanning directory...");

    // Verify the final progress report
    let last_progress = progress_events.last().unwrap().1.as_object().unwrap();
    let last_percentage = last_progress["percentage"].as_f64().unwrap();
    assert!((last_percentage - 1.0).abs() < 1e-6);
    assert_eq!(last_progress["message"].as_str().unwrap(), "Analysis complete");

    // Use the utility method for a simple assertion
    ctx.assert_event_emitted("progress");
}

/// Verify MockContext handles custom app directory correctly
#[tokio::test]
async fn test_mock_context_app_dir() {
    let custom_path = std::env::temp_dir().join("code-viz-test-dir");
    let ctx = MockContext::new().with_app_dir(custom_path.clone());

    use code_viz_core::traits::AppContext;
    assert_eq!(ctx.get_app_dir(), custom_path);
}

/// SSOT Verification Test: Ensures Tauri and API layers produce identical results
#[tokio::test]
async fn test_ssot_contract_consistency() {
    use code_viz_tauri::models::TreeNode as TauriTreeNode;
    use code_viz_api::TreeNode as ApiTreeNode;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Create an API TreeNode
    let api_node = ApiTreeNode {
        id: "test.rs".to_string(),
        name: "test.rs".to_string(),
        path: PathBuf::from("test.rs"),
        loc: 100,
        complexity: 10,
        node_type: "file".to_string(),
        children: vec![],
        last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1234567890),
        dead_code_ratio: Some(0.25),
    };

    // Convert to Tauri TreeNode
    let tauri_node: TauriTreeNode = api_node.clone().into();

    // Serialize both
    let api_json_str = serde_json::to_string(&api_node).unwrap();
    let tauri_json_str = serde_json::to_string(&tauri_node).unwrap();

    // Parse back to compare
    let api_json: serde_json::Value = serde_json::from_str(&api_json_str).unwrap();
    let tauri_json: serde_json::Value = serde_json::from_str(&tauri_json_str).unwrap();

    // CRITICAL SSOT ASSERTION: JSON must be identical
    assert_eq!(api_json["id"], tauri_json["id"]);
    assert_eq!(api_json["name"], tauri_json["name"]);
    assert_eq!(api_json["loc"], tauri_json["loc"]);
    assert_eq!(api_json["complexity"], tauri_json["complexity"]);
    assert_eq!(api_json["type"], tauri_json["type"]);
    assert_eq!(api_json["lastModified"], tauri_json["lastModified"]);
    assert_eq!(api_json["deadCodeRatio"], tauri_json["deadCodeRatio"]);

    // Verify lastModified is ISO 8601 string in BOTH
    assert!(api_json["lastModified"].is_string());
    assert!(tauri_json["lastModified"].is_string());

    // Verify no raw SystemTime leakage
    assert!(!api_json_str.contains("secs_since_epoch"));
    assert!(!tauri_json_str.contains("secs_since_epoch"));
}
