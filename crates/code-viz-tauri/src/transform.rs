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

    #[test]
    fn test_mixed_depth_structure() {
        let files = vec![
            create_test_file("README.md", 10),
            create_test_file("src/main.rs", 100),
            create_test_file("src/utils/helper.rs", 50),
            create_test_file("src/utils/config.rs", 30),
            create_test_file("tests/integration/test1.rs", 40),
        ];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.loc, 230);
        assert_eq!(tree.children.len(), 3); // README.md, src, tests

        // Verify root level file
        let readme = tree.children.iter().find(|c| c.name == "README.md").unwrap();
        assert_eq!(readme.node_type, "file");
        assert_eq!(readme.loc, 10);

        // Verify nested directories aggregate correctly
        let src = tree.children.iter().find(|c| c.name == "src").unwrap();
        assert_eq!(src.loc, 180);
        assert_eq!(src.children.len(), 2); // main.rs and utils/

        let utils = src.children.iter().find(|c| c.name == "utils").unwrap();
        assert_eq!(utils.loc, 80);
        assert_eq!(utils.children.len(), 2);
    }

    #[test]
    fn test_special_characters_in_path() {
        let files = vec![
            create_test_file("src/my-file.rs", 100),
            create_test_file("src/file_with_underscore.rs", 200),
            create_test_file("tests/test-1.rs", 50),
        ];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.loc, 350);

        let src = tree.children.iter().find(|c| c.name == "src").unwrap();
        assert_eq!(src.children.len(), 2);

        let file1 = src.children.iter().find(|c| c.name == "my-file.rs");
        let file2 = src.children.iter().find(|c| c.name == "file_with_underscore.rs");

        assert!(file1.is_some());
        assert!(file2.is_some());
    }

    #[test]
    fn test_files_with_same_name_different_dirs() {
        let files = vec![
            create_test_file("src/main.rs", 100),
            create_test_file("tests/main.rs", 200),
            create_test_file("examples/main.rs", 300),
        ];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.loc, 600);
        assert_eq!(tree.children.len(), 3);

        // Each directory should have its own main.rs with correct LOC
        let src = tree.children.iter().find(|c| c.name == "src").unwrap();
        let src_main = &src.children[0];
        assert_eq!(src_main.name, "main.rs");
        assert_eq!(src_main.loc, 100);

        let tests = tree.children.iter().find(|c| c.name == "tests").unwrap();
        let tests_main = &tests.children[0];
        assert_eq!(tests_main.name, "main.rs");
        assert_eq!(tests_main.loc, 200);

        let examples = tree.children.iter().find(|c| c.name == "examples").unwrap();
        let examples_main = &examples.children[0];
        assert_eq!(examples_main.name, "main.rs");
        assert_eq!(examples_main.loc, 300);
    }

    #[test]
    fn test_very_long_path() {
        let long_path = "a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/file.rs";
        let files = vec![create_test_file(long_path, 50)];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.loc, 50);
        assert_eq!(tree.children.len(), 1);

        // Verify we can traverse the entire depth
        let mut current = &tree.children[0];
        let mut depth = 0;
        while !current.children.is_empty() && current.node_type != "file" {
            current = &current.children[0];
            depth += 1;
        }

        // Should have traversed through all intermediate directories
        assert!(depth > 20);
        assert_eq!(current.name, "file.rs");
        assert_eq!(current.node_type, "file");
    }

    #[test]
    fn test_complexity_capping() {
        // Test that complexity is properly capped at 100
        let files = vec![
            create_test_file("huge_file.rs", 10000), // Should cap at 100
        ];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.loc, 10000);
        assert_eq!(tree.complexity, 100); // Should be capped

        let file = &tree.children[0];
        assert_eq!(file.complexity, 100); // Should be capped
    }

    #[test]
    fn test_last_modified_aggregation() {
        use std::time::Duration;

        let now = SystemTime::now();
        let old = now - Duration::from_secs(86400); // 1 day ago
        let older = now - Duration::from_secs(172800); // 2 days ago

        let files = vec![
            FileMetrics {
                path: PathBuf::from("src/old.rs"),
                language: "rust".to_string(),
                loc: 100,
                size_bytes: 2048,
                function_count: 5,
                last_modified: old,
            },
            FileMetrics {
                path: PathBuf::from("src/older.rs"),
                language: "rust".to_string(),
                loc: 100,
                size_bytes: 2048,
                function_count: 5,
                last_modified: older,
            },
            FileMetrics {
                path: PathBuf::from("src/newest.rs"),
                language: "rust".to_string(),
                loc: 100,
                size_bytes: 2048,
                function_count: 5,
                last_modified: now,
            },
        ];

        let tree = flat_to_hierarchy(files);

        // Root should have the most recent timestamp
        let src = &tree.children[0];
        assert!(src.last_modified >= now - Duration::from_secs(1)); // Allow for small time differences
    }

    #[test]
    fn test_parallel_directory_trees() {
        let files = vec![
            create_test_file("frontend/src/main.ts", 100),
            create_test_file("frontend/src/utils.ts", 50),
            create_test_file("backend/src/main.rs", 200),
            create_test_file("backend/src/handler.rs", 150),
            create_test_file("shared/types.ts", 30),
        ];
        let tree = flat_to_hierarchy(files);

        assert_eq!(tree.loc, 530);
        assert_eq!(tree.children.len(), 3); // frontend, backend, shared

        let frontend = tree.children.iter().find(|c| c.name == "frontend").unwrap();
        assert_eq!(frontend.loc, 150);

        let backend = tree.children.iter().find(|c| c.name == "backend").unwrap();
        assert_eq!(backend.loc, 350);

        let shared = tree.children.iter().find(|c| c.name == "shared").unwrap();
        assert_eq!(shared.loc, 30);
    }

    #[test]
    fn test_performance_large_dataset() {
        use std::time::Instant;

        // Generate 10,000 files to test O(n) complexity
        let mut files = Vec::new();
        for i in 0..10_000 {
            let path = format!("src/module_{}/submodule_{}/file_{}.rs", i / 100, i / 10, i);
            files.push(create_test_file(&path, 100));
        }

        let start = Instant::now();
        let tree = flat_to_hierarchy(files);
        let duration = start.elapsed();

        // Verify correctness
        assert_eq!(tree.loc, 1_000_000); // 10,000 files * 100 LOC

        // Performance check: should complete in reasonable time (< 1 second for 10K files)
        assert!(duration.as_secs() < 1, "Performance test failed: took {:?} for 10K files", duration);

        println!("Performance test: 10,000 files processed in {:?}", duration);
    }

    #[test]
    fn test_no_duplicate_children() {
        // Ensure that the same directory isn't added multiple times as a child
        let files = vec![
            create_test_file("src/a.rs", 100),
            create_test_file("src/b.rs", 200),
            create_test_file("src/c.rs", 300),
        ];
        let tree = flat_to_hierarchy(files);

        // Root should only have one "src" directory
        assert_eq!(tree.children.len(), 1);
        assert_eq!(tree.children[0].name, "src");

        // Src should have exactly 3 file children
        let src = &tree.children[0];
        assert_eq!(src.children.len(), 3);

        // Verify no duplicate names
        let names: Vec<&str> = src.children.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"a.rs"));
        assert!(names.contains(&"b.rs"));
        assert!(names.contains(&"c.rs"));
    }
}

#[cfg(test)]
#[path = "transform.test.rs"]
mod transform_test;
