//! Transformation utilities for converting flat file metrics to hierarchical trees
//!
//! This module provides functions to transform the flat Vec<FileMetrics> output
//! from code-viz-core into hierarchical TreeNode structures for visualization.

use code_viz_core::models::FileMetrics;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::models::TreeNode;

/// Finds the common root directory from a list of file paths
///
/// This function identifies the deepest common directory that contains all files.
/// Used to convert absolute filesystem paths to project-relative paths.
///
/// # Arguments
/// * `files` - Vector of file metrics with absolute paths
///
/// # Returns
/// The common root directory path
///
/// # Examples
/// ```
/// // Files: /home/user/project/src/main.rs, /home/user/project/tests/test.rs
/// // Returns: /home/user/project
/// ```
fn find_common_root(files: &[FileMetrics]) -> PathBuf {
    if files.is_empty() {
        return PathBuf::from("/");
    }

    // Get the parent directories of all files
    let parents: Vec<PathBuf> = files
        .iter()
        .filter_map(|f| f.path.parent())
        .map(|p| p.to_path_buf())
        .collect();

    if parents.is_empty() {
        return PathBuf::from("/");
    }

    // Start with the first parent as the candidate
    let mut common = parents[0].clone();

    // Find the common ancestor of all parent directories
    for parent in parents.iter().skip(1) {
        // Walk up the tree until we find a common ancestor
        while !parent.starts_with(&common) && common != Path::new("/") {
            common = common
                .parent()
                .unwrap_or(Path::new("/"))
                .to_path_buf();
        }
    }

    // If all files have the same parent directory AND they're in a subdirectory
    // (not at root level), go up one more level to get the project root.
    // For example, if all files are in "src/", we want the parent of "src/" as root.
    let all_same_parent = parents.iter().all(|p| p == &common);
    if all_same_parent && parents.len() > 1 {
        // Multiple files in the same directory suggests it's a subdirectory
        common = common
            .parent()
            .unwrap_or(&common)
            .to_path_buf();
    }

    common
}

/// Strips a prefix from a path, returning a relative path
///
/// If the path doesn't start with the prefix, returns the path as-is.
///
/// # Arguments
/// * `path` - The absolute path to strip
/// * `prefix` - The prefix to remove
///
/// # Returns
/// A relative path with the prefix removed
fn strip_prefix(path: &Path, prefix: &Path) -> PathBuf {
    path.strip_prefix(prefix)
        .unwrap_or(path)
        .to_path_buf()
}

/// Converts a flat list of file metrics into a hierarchical tree structure
///
/// This function builds a directory tree from flat file paths, creating intermediate
/// directory nodes as needed and aggregating metrics up the tree.
///
/// # Arguments
/// * `files` - Flat vector of file metrics from code-viz-core analysis
///
/// # Returns
/// A single root TreeNode containing the entire directory hierarchy
///
/// # Complexity
/// O(n) where n is the number of files
///
/// # Examples
/// ```
/// use code_viz_tauri::transform::flat_to_hierarchy;
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
///     },
/// ];
///
/// let tree = flat_to_hierarchy(files);
/// assert_eq!(tree.name, "root");
/// assert_eq!(tree.children.len(), 1);
/// ```
pub fn flat_to_hierarchy(files: Vec<FileMetrics>) -> TreeNode {
    // Handle empty input - return empty root node
    if files.is_empty() {
        return TreeNode {
            id: "/".to_string(),
            name: "root".to_string(),
            path: PathBuf::from("/"),
            loc: 0,
            complexity: 0,
            node_type: "directory".to_string(),
            children: vec![],
            last_modified: std::time::SystemTime::now(),
            dead_code_ratio: None,
        };
    }

    // Check if paths are absolute (start with "/") or relative
    let has_absolute_paths = files.iter().any(|f| f.path.is_absolute());

    let (root_path, project_name) = if has_absolute_paths {
        // Find common root path from all files and use project name
        let common_root = find_common_root(&files);
        let proj_name = common_root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("root")
            .to_string();
        (common_root, proj_name)
    } else {
        // For relative paths, use generic root
        (PathBuf::from("/"), "root".to_string())
    };

    // Map to store directory nodes by their path (for O(1) lookup)
    let mut dir_map: HashMap<PathBuf, TreeNode> = HashMap::new();

    // Root node representing the repository
    let root_node_path = if has_absolute_paths {
        PathBuf::from("")
    } else {
        root_path.clone()
    };

    let root_node = TreeNode {
        id: "/".to_string(),
        name: project_name,
        path: root_node_path.clone(),
        loc: 0,
        complexity: 0,
        node_type: "directory".to_string(),
        children: vec![],
        last_modified: std::time::SystemTime::now(),
        dead_code_ratio: None,
    };
    dir_map.insert(root_node_path.clone(), root_node);

    // First pass: create all file nodes and ensure all parent directories exist
    let mut file_nodes = Vec::new();
    for file in files {
        // Create file node
        let file_loc = file.loc;
        let file_complexity = calculate_complexity(file_loc);

        // Convert absolute path to relative path by stripping common root
        let file_path = if has_absolute_paths {
            strip_prefix(&file.path, &root_path)
        } else {
            file.path.clone()
        };
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let file_node = TreeNode {
            id: file_path.to_string_lossy().to_string(),
            name: file_name,
            path: file_path.clone(),
            loc: file_loc,
            complexity: file_complexity,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: file.last_modified,
            dead_code_ratio: None,
        };
        file_nodes.push((file_path.clone(), file_node));

        // Ensure all parent directories exist
        ensure_parent_directories(&file_path, &mut dir_map, &root_node_path);
    }

    // Second pass: attach file nodes to their parent directories
    for (file_path, file_node) in file_nodes {
        let parent_path = get_parent_path(&file_path, &root_node_path);
        if let Some(parent) = dir_map.get_mut(&parent_path) {
            parent.children.push(file_node);
        }
    }

    // Third pass: aggregate metrics up the tree (bottom-up)
    aggregate_directory_metrics(&mut dir_map, &root_node_path);

    // Extract root node
    dir_map.remove(&root_node_path).unwrap()
}

/// Ensures all parent directories exist in the directory map
fn ensure_parent_directories(
    file_path: &Path,
    dir_map: &mut HashMap<PathBuf, TreeNode>,
    root_path: &Path,
) {
    let mut current = file_path.to_path_buf();

    // Walk up the directory tree, creating missing directories
    while let Some(parent) = current.parent() {
        if parent.as_os_str().is_empty() || parent == root_path {
            break;
        }

        let parent_buf = parent.to_path_buf();
        if !dir_map.contains_key(&parent_buf) {
            let name = parent
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let dir_node = TreeNode {
                id: parent_buf.to_string_lossy().to_string(),
                name,
                path: parent_buf.clone(),
                loc: 0,
                complexity: 0,
                node_type: "directory".to_string(),
                children: vec![],
                last_modified: std::time::SystemTime::now(),
                dead_code_ratio: None,
            };
            dir_map.insert(parent_buf.clone(), dir_node);

            // Ensure this directory's parent exists
            ensure_parent_in_tree(&parent_buf, dir_map, root_path);
        }
        current = parent_buf;
    }
}

/// Ensures a directory node is attached to its parent
fn ensure_parent_in_tree(
    dir_path: &Path,
    dir_map: &mut HashMap<PathBuf, TreeNode>,
    root_path: &Path,
) {
    let parent_path = get_parent_path(dir_path, root_path);

    // Don't try to attach root to itself
    if dir_path == root_path {
        return;
    }

    // Ensure parent directory exists
    if !dir_map.contains_key(&parent_path) {
        ensure_parent_directories(dir_path, dir_map, root_path);
    }
}

/// Gets the parent path of a given path, defaulting to root if no parent
fn get_parent_path(path: &Path, root_path: &Path) -> PathBuf {
    path.parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| root_path.to_path_buf())
}

/// Aggregates metrics (LOC, complexity) from children up to parent directories
fn aggregate_directory_metrics(
    dir_map: &mut HashMap<PathBuf, TreeNode>,
    root_path: &Path,
) {
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
            let total_loc: usize = dir_node.children.iter().map(|c| c.loc).sum();
            let max_modified = dir_node
                .children
                .iter()
                .map(|c| c.last_modified)
                .max()
                .unwrap_or(std::time::SystemTime::now());

            // Store calculated values
            let complexity = calculate_complexity(total_loc);

            // Update the directory node
            if let Some(dir_node_mut) = dir_map.get_mut(&path) {
                dir_node_mut.loc = total_loc;
                dir_node_mut.complexity = complexity;
                dir_node_mut.last_modified = max_modified;
            }

            // Now attach this directory to its parent
            let parent_path = get_parent_path(&path, root_path);
            if parent_path != path {
                // Clone the updated node
                if let Some(updated_node) = dir_map.get(&path).cloned() {
                    if let Some(parent) = dir_map.get_mut(&parent_path) {
                        // Check if this child already exists in parent
                        if !parent.children.iter().any(|c| c.path == path) {
                            parent.children.push(updated_node);
                        }
                    }
                }
            }
        }
    }

    // Finally, aggregate root node metrics
    if let Some(root) = dir_map.get_mut(root_path) {
        let total_loc: usize = root.children.iter().map(|c| c.loc).sum();
        let max_modified = root
            .children
            .iter()
            .map(|c| c.last_modified)
            .max()
            .unwrap_or(std::time::SystemTime::now());

        root.loc = total_loc;
        root.complexity = calculate_complexity(total_loc);
        root.last_modified = max_modified;
    }
}

/// Calculate complexity score from LOC (placeholder: loc/10, capped at 100)
fn calculate_complexity(loc: usize) -> u32 {
    ((loc / 10) as u32).min(100)
}

