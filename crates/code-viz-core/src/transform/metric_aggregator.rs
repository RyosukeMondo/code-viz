//! Metric aggregation utilities for directory tree nodes
//!
//! This module provides functions for aggregating metrics (LOC, complexity, etc.)
//! from child nodes up to parent directories.

use super::path_utils::get_parent_path;
use super::tree_builder::calculate_complexity;
use crate::models::TreeNode;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Aggregates metrics (LOC, complexity) from children up to parent directories
///
/// This function performs a bottom-up traversal of the directory tree, aggregating
/// metrics from leaf nodes (files) up through parent directories to the root.
///
/// # Arguments
/// * `dir_map` - Mutable HashMap of path to TreeNode
/// * `root_path` - The root path of the tree
pub fn aggregate_directory_metrics(dir_map: &mut HashMap<PathBuf, TreeNode>, root_path: &Path) {
    // Collect all paths and sort by depth (deepest first) for bottom-up aggregation
    let mut paths: Vec<PathBuf> = dir_map.keys().cloned().collect();
    paths.sort_by(|a, b| {
        let depth_a = a.components().count();
        let depth_b = b.components().count();
        depth_b.cmp(&depth_a) // Sort descending by depth
    });

    // Process directories from deepest to shallowest
    for path in paths {
        if path == *root_path {
            continue; // Skip root in this loop, handle it last
        }

        // Calculate this directory's metrics from its children
        if let Some(dir_node) = dir_map.get(&path) {
            let (total_loc, max_modified, complexity) = aggregate_child_metrics(dir_node);

            // Update the directory node
            if let Some(dir_node_mut) = dir_map.get_mut(&path) {
                dir_node_mut.loc = total_loc;
                dir_node_mut.complexity = complexity;
                dir_node_mut.last_modified = max_modified;
            }

            // Now attach this directory to its parent
            attach_to_parent(dir_map, &path, root_path);
        }
    }

    // Finally, aggregate root node metrics
    aggregate_root_metrics(dir_map, root_path);
}

/// Aggregates metrics from a node's children
fn aggregate_child_metrics(node: &TreeNode) -> (usize, SystemTime, u32) {
    let total_loc: usize = node.children.iter().map(|c| c.loc).sum();
    let max_modified = node
        .children
        .iter()
        .map(|c| c.last_modified)
        .max()
        .unwrap_or(SystemTime::now());
    let complexity = calculate_complexity(total_loc);

    (total_loc, max_modified, complexity)
}

/// Attaches a directory node to its parent
fn attach_to_parent(dir_map: &mut HashMap<PathBuf, TreeNode>, path: &Path, root_path: &Path) {
    let parent_path = get_parent_path(path, root_path);
    if parent_path == *path {
        return; // Don't attach to self
    }

    // Clone the updated node
    if let Some(updated_node) = dir_map.get(path).cloned() {
        if let Some(parent) = dir_map.get_mut(&parent_path) {
            // Check if this child already exists in parent
            if !parent.children.iter().any(|c| c.path == *path) {
                parent.children.push(updated_node);
            }
        }
    }
}

/// Aggregates metrics for the root node
fn aggregate_root_metrics(dir_map: &mut HashMap<PathBuf, TreeNode>, root_path: &Path) {
    if let Some(root) = dir_map.get_mut(root_path) {
        let total_loc: usize = root.children.iter().map(|c| c.loc).sum();
        let max_modified = root
            .children
            .iter()
            .map(|c| c.last_modified)
            .max()
            .unwrap_or(SystemTime::now());

        root.loc = total_loc;
        root.complexity = calculate_complexity(total_loc);
        root.last_modified = max_modified;
    }
}
