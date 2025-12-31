//! Transformation utilities for converting flat file metrics to hierarchical trees
//!
//! This module provides a thin wrapper around the core transform module,
//! adapting its output for the API layer.

use code_viz_core::models::FileMetrics;
use code_viz_core::transform::flat_to_hierarchy as core_flat_to_hierarchy;

use crate::models::TreeNode;
use crate::error::ApiError;

/// Converts a flat list of file metrics into a hierarchical tree structure
///
/// This function delegates to the core transform module and converts any
/// errors to API-specific error types.
///
/// # Arguments
/// * `files` - Flat vector of file metrics from code-viz-core analysis
///
/// # Returns
/// A Result containing the root TreeNode or an ApiError
///
/// # Errors
/// Returns `ApiError::TransformError` if the transformation fails
///
/// # Examples
/// ```
/// use code_viz_api::transform::flat_to_hierarchy;
/// use code_viz_core::models::FileMetrics;
/// use std::path::PathBuf;
/// use std::time::SystemTime;
///
/// let files = vec![
///     FileMetrics {
///         path: PathBuf::from("src/main.rs"),
///         language: "rust".to_string(),
///         loc: 100,
///         size_bytes: 2048,
///         function_count: 5,
///         last_modified: SystemTime::now(),
///         dead_function_count: None,
///         dead_code_loc: None,
///         dead_code_ratio: None,
///         coupling: None,
///         code_churn: None,
///         ai_bloat_index: None,
///         cognitive_complexity: None,
///         test_coverage: None,
///     },
/// ];
///
/// let tree = flat_to_hierarchy(files).unwrap();
/// assert_eq!(tree.name, "root");
/// assert_eq!(tree.children.len(), 1);
/// ```
pub fn flat_to_hierarchy(files: Vec<FileMetrics>) -> Result<TreeNode, ApiError> {
    // Delegate to core transform module
    let core_tree = core_flat_to_hierarchy(files)
        .map_err(|e| ApiError::TransformError(e.to_string()))?;

    // Convert core TreeNode to API TreeNode
    // Since the types are identical, we can use Into trait
    Ok(convert_tree_node(core_tree))
}

/// Converts a core TreeNode to an API TreeNode
///
/// Both types have identical structure, so this is a direct field-by-field copy.
/// This function exists to maintain type separation between layers and allow
/// future divergence if needed.
fn convert_tree_node(core_node: code_viz_core::models::TreeNode) -> TreeNode {
    TreeNode {
        id: core_node.id,
        name: core_node.name,
        path: core_node.path,
        loc: core_node.loc,
        complexity: core_node.complexity,
        node_type: core_node.node_type,
        children: core_node.children.into_iter().map(convert_tree_node).collect(),
        last_modified: core_node.last_modified,
        dead_code_ratio: core_node.dead_code_ratio,
        language: core_node.language,
        size_bytes: core_node.size_bytes,
        function_count: core_node.function_count,
        coupling: core_node.coupling,
        code_churn: core_node.code_churn,
        ai_bloat_index: core_node.ai_bloat_index,
        cognitive_complexity: core_node.cognitive_complexity,
        test_coverage: core_node.test_coverage,
    }
}
