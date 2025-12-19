//! Tauri command definitions for frontend IPC communication
//!
//! This module contains Tauri commands that expose the code-viz-core
//! analysis functionality to the frontend.

use crate::models::TreeNode;
use crate::transform::flat_to_hierarchy;
use crate::context::TauriContext;
use code_viz_core::traits::AppContext;
use code_viz_core::analyze;
use code_viz_core::AnalysisConfig as CoreAnalysisConfig;
use code_viz_dead_code::{analyze_dead_code, DeadCodeResult};
use std::path::PathBuf;
use uuid::Uuid;

/// Analyzes a repository and returns hierarchical tree structure for visualization
///
/// This command wraps the code-viz-core analysis engine, converting the flat
/// file metrics output into a hierarchical TreeNode structure suitable for
/// rendering as a treemap.
///
/// # Arguments
/// * `path` - Absolute path to the repository root directory
/// * `request_id` - Optional UUID for correlating frontend and backend logs
///
/// # Returns
/// * `Result<TreeNode, String>` - Root TreeNode on success, error message on failure
///
/// # Errors
/// Returns error if:
/// - Path does not exist or is not accessible
/// - Analysis engine encounters critical failure
/// - Path is not a valid directory
///
/// # Examples
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const requestId = crypto.randomUUID();
/// const tree = await invoke<TreeNode>('analyze_repository', {
///   path: '/home/user/my-project',
///   requestId: requestId
/// });
/// ```
#[tauri::command]
#[specta::specta]
pub async fn analyze_repository(
    app: tauri::AppHandle,
    path: String,
    request_id: Option<String>,
) -> Result<TreeNode, String> {
    let ctx = TauriContext::new(app);
    analyze_repository_inner(ctx, path, request_id).await
}

/// Inner implementation of analyze_repository that uses AppContext
pub async fn analyze_repository_inner(
    ctx: impl AppContext,
    path: String,
    request_id: Option<String>,
) -> Result<TreeNode, String> {
    // Parse request ID or generate a new one
    let req_id = request_id
        .and_then(|id| Uuid::parse_str(&id).ok())
        .unwrap_or_else(Uuid::new_v4);

    tracing::info!(
        request_id = %req_id,
        path = %path,
        "Starting repository analysis"
    );

    ctx.report_progress(0.1, "Validating path").await.ok();

    // Validate path exists and is a directory
    let repo_path = PathBuf::from(&path);
    if !repo_path.exists() {
        tracing::error!(
            request_id = %req_id,
            path = %path,
            "Path does not exist"
        );
        return Err(format!("Path does not exist: {}", path));
    }
    if !repo_path.is_dir() {
        tracing::error!(
            request_id = %req_id,
            path = %path,
            "Path is not a directory"
        );
        return Err(format!("Path is not a directory: {}", path));
    }

    tracing::debug!(
        request_id = %req_id,
        "Using default analysis configuration"
    );

    ctx.report_progress(0.3, "Initial analysis").await.ok();

    // Use default configuration (excludes node_modules, target, .git, etc.)
    let config = CoreAnalysisConfig::default();

    tracing::info!(
        request_id = %req_id,
        "Calling code-viz-core analysis engine"
    );

    ctx.report_progress(0.5, "Scanning files").await.ok();

    // Call code-viz-core analysis engine
    let analysis_result = analyze(&repo_path, &config).map_err(|e| {
        tracing::error!(
            request_id = %req_id,
            error = %e,
            "Analysis engine failed"
        );
        format!("Analysis failed: {}", e)
    })?;

    ctx.report_progress(0.8, "Processing results").await.ok();

    tracing::info!(
        request_id = %req_id,
        file_count = analysis_result.files.len(),
        "Analysis complete, transforming to hierarchy"
    );

    // Transform flat file metrics to hierarchical tree
    let tree = flat_to_hierarchy(analysis_result.files);

    tracing::info!(
        request_id = %req_id,
        "Repository analysis completed successfully"
    );

    ctx.report_progress(1.0, "Done").await.ok();

    Ok(tree)
}

/// Analyzes dead code in a repository and returns filtered results
///
/// This command wraps the code-viz-dead-code analysis engine, performing
/// reachability analysis to identify unreachable (dead) code symbols.
/// Results are filtered by confidence score to reduce false positives.
///
/// # Arguments
/// * `path` - Absolute path to the repository root directory
/// * `min_confidence` - Minimum confidence score (0-100) for dead code inclusion
/// * `request_id` - Optional UUID for correlating frontend and backend logs
///
/// # Returns
/// * `Result<DeadCodeResult, String>` - Dead code analysis result on success, error message on failure
///
/// # Errors
/// Returns error if:
/// - Path does not exist or is not accessible
/// - Dead code analysis engine encounters critical failure
/// - Path is not a valid directory
///
/// # Examples
/// ```typescript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const requestId = crypto.randomUUID();
/// const result = await invoke<DeadCodeResult>('analyze_dead_code_command', {
///   path: '/home/user/my-project',
///   minConfidence: 80,
///   requestId: requestId
/// });
/// ```
#[tauri::command]
#[specta::specta]
pub async fn analyze_dead_code_command(
    path: String,
    min_confidence: u8,
    request_id: Option<String>,
) -> Result<DeadCodeResult, String> {
    // Parse request ID or generate a new one
    let req_id = request_id
        .and_then(|id| Uuid::parse_str(&id).ok())
        .unwrap_or_else(Uuid::new_v4);

    tracing::info!(
        request_id = %req_id,
        path = %path,
        min_confidence = min_confidence,
        "Starting dead code analysis"
    );

    // Validate path exists and is a directory
    let repo_path = PathBuf::from(&path);
    if !repo_path.exists() {
        tracing::error!(
            request_id = %req_id,
            path = %path,
            "Path does not exist"
        );
        return Err(format!("Path does not exist: {}", path));
    }
    if !repo_path.is_dir() {
        tracing::error!(
            request_id = %req_id,
            path = %path,
            "Path is not a directory"
        );
        return Err(format!("Path is not a directory: {}", path));
    }

    tracing::info!(
        request_id = %req_id,
        "Calling code-viz-dead-code analysis engine with default configuration"
    );

    // Call dead code analysis engine with default config
    // None uses default configuration (excludes node_modules, dist, build, .git)
    let analysis_result = analyze_dead_code(&repo_path, None).map_err(|e| {
        tracing::error!(
            request_id = %req_id,
            error = %e,
            "Dead code analysis engine failed"
        );
        format!("Dead code analysis failed: {}", e)
    })?;

    tracing::info!(
        request_id = %req_id,
        total_files = analysis_result.summary.total_files,
        files_with_dead_code = analysis_result.summary.files_with_dead_code,
        dead_functions = analysis_result.summary.dead_functions,
        "Dead code analysis complete, filtering by confidence"
    );

    // Filter by confidence score
    let filtered_result = analysis_result.filter_by_confidence(min_confidence);

    tracing::info!(
        request_id = %req_id,
        filtered_files_with_dead_code = filtered_result.summary.files_with_dead_code,
        filtered_dead_functions = filtered_result.summary.dead_functions,
        "Dead code analysis completed successfully"
    );

    Ok(filtered_result)
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use serde_json;
    use std::env;

    /// Integration test: Verify analyze_repository returns valid serializable TreeNode
    ///
    /// This test ensures that the entire command pipeline (analysis → transformation → serialization)
    /// produces JSON that matches the TypeScript contract expected by the frontend.
    #[tokio::test]
    async fn test_analyze_repository_serialization_contract() {
        use code_viz_core::mocks::MockContext;
        let ctx = MockContext::new();

        // Use current directory as test subject (code-viz itself)
        let current_dir = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        // Execute the command
        let result = analyze_repository_inner(ctx, current_dir, Some("test-request-id".to_string())).await;

        // Command should succeed
        assert!(result.is_ok(), "analyze_repository failed: {:?}", result);

        let tree = result.unwrap();

        // Verify basic structure
        assert!(!tree.name.is_empty(), "TreeNode name should not be empty");
        assert!(tree.loc > 0, "TreeNode LOC should be positive");
        assert!(tree.complexity > 0, "TreeNode complexity should be positive");

        // CRITICAL: Serialize to JSON and verify structure
        let json_value = serde_json::to_value(&tree).expect("Failed to serialize TreeNode to JSON");

        // Verify required fields exist and have correct types
        assert!(
            json_value["id"].is_string(),
            "id must be a string, got: {:?}",
            json_value["id"]
        );
        assert!(
            json_value["name"].is_string(),
            "name must be a string, got: {:?}",
            json_value["name"]
        );
        assert!(
            json_value["path"].is_string(),
            "path must be a string, got: {:?}",
            json_value["path"]
        );
        assert!(
            json_value["loc"].is_number(),
            "loc must be a number, got: {:?}",
            json_value["loc"]
        );
        assert!(
            json_value["complexity"].is_number(),
            "complexity must be a number, got: {:?}",
            json_value["complexity"]
        );
        assert!(
            json_value["type"].is_string(),
            "type must be a string, got: {:?}",
            json_value["type"]
        );

        // CRITICAL: Verify lastModified is serialized as ISO 8601 string, NOT raw object
        assert!(
            json_value["lastModified"].is_string(),
            "lastModified MUST be a string (ISO 8601), not an object. Got: {:?}",
            json_value["lastModified"]
        );

        // Verify ISO 8601 format
        let timestamp_str = json_value["lastModified"]
            .as_str()
            .expect("lastModified should be a string");
        assert!(
            timestamp_str.contains('T'),
            "lastModified must be ISO 8601 format (contain T): {}",
            timestamp_str
        );
        assert!(
            timestamp_str.ends_with('Z'),
            "lastModified must use Z for UTC (not +00:00): {}",
            timestamp_str
        );

        // Verify children array exists (even if empty for files)
        assert!(
            json_value["children"].is_array(),
            "children must be an array, got: {:?}",
            json_value["children"]
        );

        // If there are children, verify they also have proper serialization
        if let Some(children) = json_value["children"].as_array() {
            if !children.is_empty() {
                let first_child = &children[0];
                assert!(
                    first_child["lastModified"].is_string(),
                    "Child node lastModified must also be a string, got: {:?}",
                    first_child["lastModified"]
                );
            }
        }
    }

    /// Integration test: Verify serialized JSON contains no raw SystemTime objects
    ///
    /// This test catches regressions where SystemTime might serialize as
    /// {secs_since_epoch: ..., nanos_since_epoch: ...} instead of ISO 8601 string.
    #[tokio::test]
    async fn test_no_raw_systemtime_in_json() {
        use code_viz_core::mocks::MockContext;
        let ctx = MockContext::new();

        let current_dir = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        let result = analyze_repository_inner(ctx, current_dir, Some("test-systemtime".to_string())).await;
        assert!(result.is_ok(), "analyze_repository failed");

        let tree = result.unwrap();
        let json_str = serde_json::to_string(&tree).expect("Failed to serialize to JSON string");

        // CRITICAL: JSON should never contain raw SystemTime fields
        assert!(
            !json_str.contains("secs_since_epoch"),
            "JSON contains raw SystemTime field 'secs_since_epoch'. This breaks TypeScript contract!"
        );
        assert!(
            !json_str.contains("nanos_since_epoch"),
            "JSON contains raw SystemTime field 'nanos_since_epoch'. This breaks TypeScript contract!"
        );

        // Verify it does contain the expected field name
        assert!(
            json_str.contains("lastModified"),
            "JSON must contain 'lastModified' field"
        );
    }

    /// Integration test: Verify dead code command serialization
    ///
    /// Tests that dead code results serialize correctly and include proper timestamps
    #[tokio::test]
    async fn test_analyze_dead_code_serialization() {
        let current_dir = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        // Call with min_confidence of 70
        let result =
            analyze_dead_code_command(current_dir, 70, Some("test-dead-code".to_string())).await;

        // Command should succeed
        assert!(
            result.is_ok(),
            "analyze_dead_code_command failed: {:?}",
            result
        );

        let dead_code_result = result.unwrap();

        // Verify serialization works
        let json_value =
            serde_json::to_value(&dead_code_result).expect("Failed to serialize DeadCodeResult");

        // Verify basic structure
        assert!(
            json_value["summary"].is_object(),
            "summary must be an object"
        );
        assert!(
            json_value["files"].is_array(),
            "files must be an array"
        );

        // Verify no raw SystemTime objects in the entire result
        let json_str = serde_json::to_string(&dead_code_result).expect("Failed to serialize");
        assert!(
            !json_str.contains("secs_since_epoch"),
            "DeadCodeResult JSON should not contain raw SystemTime fields"
        );
    }
}
