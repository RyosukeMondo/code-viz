//! Tauri command definitions for frontend IPC communication
//!
//! This module contains Tauri commands that expose the code-viz-core
//! analysis functionality to the frontend.

use crate::models::TreeNode;
use crate::transform::flat_to_hierarchy;
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
    let config = CoreAnalysisConfig::default();

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
