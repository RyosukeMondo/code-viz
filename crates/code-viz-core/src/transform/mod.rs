//! Transform module for converting flat file lists to hierarchical trees
//!
//! This module provides the core transformation logic used by both the API
//! and Tauri layers to convert flat file metrics into hierarchical tree
//! structures for visualization.

mod path_utils;
mod tree_builder;
mod metric_aggregator;

use crate::error::CodeVizError;
use crate::models::{FileMetrics, TreeNode};
use path_utils::find_common_root;
use tree_builder::{build_directory_map, build_file_nodes, attach_file_nodes_to_parents, create_empty_root};
use metric_aggregator::aggregate_directory_metrics;
use std::path::PathBuf;

/// Converts a flat list of file metrics into a hierarchical tree structure
///
/// This is the main entry point for transforming flat file data into a tree
/// hierarchy. It handles both absolute and relative paths, creates intermediate
/// directory nodes, and aggregates metrics up the tree.
///
/// # Arguments
/// * `files` - Flat vector of file metrics from code-viz-core analysis
///
/// # Returns
/// A Result containing the root TreeNode or a CodeVizError
///
/// # Errors
/// Returns `CodeVizError::TransformError` if the root node cannot be extracted
///
/// # Examples
/// ```
/// use code_viz_core::transform::flat_to_hierarchy;
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
/// ```
pub fn flat_to_hierarchy(files: Vec<FileMetrics>) -> Result<TreeNode, CodeVizError> {
    if files.is_empty() {
        return Ok(create_empty_root());
    }

    let has_absolute_paths = files.iter().any(|f| f.path.is_absolute());
    let (root_path, project_name) = determine_root_path(&files, has_absolute_paths);
    let root_node_path = if has_absolute_paths {
        PathBuf::from("")
    } else {
        root_path.clone()
    };

    let mut dir_map = build_directory_map(&project_name, &root_node_path);
    let file_nodes = build_file_nodes(files, has_absolute_paths, &root_path, &mut dir_map, &root_node_path);
    attach_file_nodes_to_parents(file_nodes, &mut dir_map, &root_node_path);
    aggregate_directory_metrics(&mut dir_map, &root_node_path);

    dir_map.remove(&root_node_path).ok_or_else(|| {
        CodeVizError::analysis(
            "tree_transform",
            "Root node missing from directory map after tree construction"
        )
    })
}

/// Determines the root path and project name from file list
fn determine_root_path(files: &[FileMetrics], has_absolute_paths: bool) -> (PathBuf, String) {
    if has_absolute_paths {
        let common_root = find_common_root(files);
        let proj_name = common_root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("root")
            .to_string();
        (common_root, proj_name)
    } else {
        (PathBuf::from("/"), "root".to_string())
    }
}
