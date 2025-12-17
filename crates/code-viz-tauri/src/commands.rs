//! Tauri command definitions for frontend IPC communication
//!
//! This module contains Tauri commands that expose the code-viz-core
//! analysis functionality to the frontend.

use crate::models::TreeNode;
use crate::transform::flat_to_hierarchy;
use code_viz_core::{analyze, AnalysisConfig};
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

    // Use default configuration (excludes node_modules, target, .git, etc.)
    let config = AnalysisConfig::default();

    tracing::info!(
        request_id = %req_id,
        "Calling code-viz-core analysis engine"
    );

    // Call code-viz-core analysis engine
    let analysis_result = analyze(&repo_path, &config).map_err(|e| {
        tracing::error!(
            request_id = %req_id,
            error = %e,
            "Analysis engine failed"
        );
        format!("Analysis failed: {}", e)
    })?;

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

    Ok(tree)
}
