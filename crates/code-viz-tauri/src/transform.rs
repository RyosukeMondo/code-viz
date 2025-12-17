//! Transformation utilities for converting flat file metrics to hierarchical trees
//!
//! This module provides functions to transform the flat Vec<FileMetrics> output
//! from code-viz-core into hierarchical TreeNode structures for visualization.

use code_viz_core::models::FileMetrics;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::models::TreeNode;

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
        };
    }

    // Map to store directory nodes by their path (for O(1) lookup)
    let mut dir_map: HashMap<PathBuf, TreeNode> = HashMap::new();

    // Root node representing the repository
    let root_path = PathBuf::from("/");
    let root_node = TreeNode {
        id: "/".to_string(),
        name: "root".to_string(),
        path: root_path.clone(),
        loc: 0,
        complexity: 0,
        node_type: "directory".to_string(),
        children: vec![],
        last_modified: std::time::SystemTime::now(),
    };
    dir_map.insert(root_path.clone(), root_node);

    // First pass: create all file nodes and ensure all parent directories exist
    let mut file_nodes = Vec::new();
    for file in files {
        // Create file node
        let file_loc = file.loc;
        let file_complexity = calculate_complexity(file_loc);
        let file_path = file.path.clone();
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let file_node = TreeNode {
            id: format!("/{}", file_path.display()),
            name: file_name,
            path: file_path.clone(),
            loc: file_loc,
            complexity: file_complexity,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: file.last_modified,
        };
        file_nodes.push((file_path.clone(), file_node));

        // Ensure all parent directories exist
        ensure_parent_directories(&file_path, &mut dir_map, &root_path);
    }

    // Second pass: attach file nodes to their parent directories
    for (file_path, file_node) in file_nodes {
        let parent_path = get_parent_path(&file_path, &root_path);
        if let Some(parent) = dir_map.get_mut(&parent_path) {
            parent.children.push(file_node);
        }
    }

    // Third pass: aggregate metrics up the tree (bottom-up)
    aggregate_directory_metrics(&mut dir_map, &root_path);

    // Extract root node
    dir_map.remove(&root_path).unwrap()
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
                id: format!("/{}", parent.display()),
                name,
                path: parent_buf.clone(),
                loc: 0,
                complexity: 0,
                node_type: "directory".to_string(),
                children: vec![],
                last_modified: std::time::SystemTime::now(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    fn create_test_file(path: &str, loc: usize) -> FileMetrics {
        FileMetrics {
            path: PathBuf::from(path),
            language: "rust".to_string(),
            loc,
            size_bytes: 2048,
            function_count: 5,
            last_modified: SystemTime::now(),
        }
    }

    #[test]
    fn test_empty_input() {
        let tree = flat_to_hierarchy(vec![]);
        assert_eq!(tree.name, "root");
        assert_eq!(tree.loc, 0);
        assert_eq!(tree.children.len(), 0);
        assert_eq!(tree.node_type, "directory");
    }

    #[test]
    fn test_single_file() {
        let files = vec![create_test_file("main.rs", 100)];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.name, "root");
        assert_eq!(tree.loc, 100);
        assert_eq!(tree.complexity, 10);
        assert_eq!(tree.children.len(), 1);

        let file = &tree.children[0];
        assert_eq!(file.name, "main.rs");
        assert_eq!(file.loc, 100);
        assert_eq!(file.node_type, "file");
        assert_eq!(file.children.len(), 0);
    }

    #[test]
    fn test_nested_structure() {
        let files = vec![
            create_test_file("src/main.rs", 100),
            create_test_file("src/lib.rs", 200),
            create_test_file("tests/test1.rs", 50),
        ];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.name, "root");
        assert_eq!(tree.loc, 350); // Sum of all files
        assert_eq!(tree.complexity, 35);
        assert_eq!(tree.children.len(), 2); // "src" and "tests" directories

        // Check src directory
        let src_dir = tree.children.iter().find(|c| c.name == "src").unwrap();
        assert_eq!(src_dir.loc, 300);
        assert_eq!(src_dir.complexity, 30);
        assert_eq!(src_dir.children.len(), 2);
        assert_eq!(src_dir.node_type, "directory");

        // Check tests directory
        let tests_dir = tree.children.iter().find(|c| c.name == "tests").unwrap();
        assert_eq!(tests_dir.loc, 50);
        assert_eq!(tests_dir.complexity, 5);
        assert_eq!(tests_dir.children.len(), 1);
    }

    #[test]
    fn test_deep_nesting() {
        let files = vec![
            create_test_file("a/b/c/d/e/file.rs", 100),
        ];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.loc, 100);
        assert_eq!(tree.children.len(), 1);

        // Traverse down the tree
        let mut current = &tree.children[0];
        assert_eq!(current.name, "a");
        assert_eq!(current.loc, 100);

        current = &current.children[0];
        assert_eq!(current.name, "b");

        current = &current.children[0];
        assert_eq!(current.name, "c");

        current = &current.children[0];
        assert_eq!(current.name, "d");

        current = &current.children[0];
        assert_eq!(current.name, "e");

        current = &current.children[0];
        assert_eq!(current.name, "file.rs");
        assert_eq!(current.node_type, "file");
        assert_eq!(current.children.len(), 0);
    }

    #[test]
    fn test_complexity_calculation() {
        assert_eq!(calculate_complexity(0), 0);
        assert_eq!(calculate_complexity(50), 5);
        assert_eq!(calculate_complexity(100), 10);
        assert_eq!(calculate_complexity(1000), 100);
        assert_eq!(calculate_complexity(2000), 100); // Capped at 100
    }

    #[test]
    fn test_multiple_files_same_directory() {
        let files = vec![
            create_test_file("src/file1.rs", 100),
            create_test_file("src/file2.rs", 200),
            create_test_file("src/file3.rs", 300),
        ];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.loc, 600);
        assert_eq!(tree.children.len(), 1);

        let src = &tree.children[0];
        assert_eq!(src.name, "src");
        assert_eq!(src.loc, 600);
        assert_eq!(src.children.len(), 3);
    }
}
