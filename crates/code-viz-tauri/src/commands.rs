//! Tauri command definitions for frontend IPC communication
//!
//! This module contains Tauri commands that expose the code-viz-core
//! analysis functionality to the frontend.

use crate::models::TreeNode;
use crate::transform::flat_to_hierarchy;
use code_viz_core::{analyze, AnalysisConfig};
use std::path::PathBuf;

/// Analyzes a repository and returns hierarchical tree structure for visualization
///
/// This command wraps the code-viz-core analysis engine, converting the flat
/// file metrics output into a hierarchical TreeNode structure suitable for
/// rendering as a treemap.
///
/// # Arguments
/// * `path` - Absolute path to the repository root directory
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
/// const tree = await invoke<TreeNode>('analyze_repository', {
///   path: '/home/user/my-project'
/// });
/// ```
#[tauri::command]
#[specta::specta]
pub async fn analyze_repository(path: String) -> Result<TreeNode, String> {
    // Validate path exists and is a directory
    let repo_path = PathBuf::from(&path);
    if !repo_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    if !repo_path.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }

    // Use default configuration (excludes node_modules, target, .git, etc.)
    let config = AnalysisConfig::default();

    // Call code-viz-core analysis engine
    let analysis_result = analyze(&repo_path, &config)
        .map_err(|e| format!("Analysis failed: {}", e))?;

    // Transform flat file metrics to hierarchical tree
    let tree = flat_to_hierarchy(analysis_result.files);

    Ok(tree)
}
