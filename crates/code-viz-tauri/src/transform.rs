//! Transformation utilities for converting flat file metrics to hierarchical trees
//!
//! This module provides a thin wrapper around the core transform module,
//! converting between Tauri-specific types and core types.

use code_viz_api::error::ApiError;
use code_viz_core::models::FileMetrics;

use crate::models::TreeNode;

/// Converts a flat list of file metrics into a hierarchical tree structure
///
/// This function delegates to the core transform module and converts the result
/// to Tauri-specific types for TypeScript bindings.
///
/// # Arguments
/// * `files` - Flat vector of file metrics from code-viz-core analysis
///
/// # Returns
/// A Result containing the root TreeNode or an ApiError
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
    // Use core transform module
    let core_tree = code_viz_core::transform::flat_to_hierarchy(files)
        .map_err(|e| ApiError::TransformError(e.to_string()))?;

    // Convert core TreeNode to API TreeNode, then to Tauri TreeNode
    let api_tree = convert_core_to_api_tree(core_tree);
    Ok(api_tree.into())
}

/// Converts a core TreeNode to an API TreeNode
fn convert_core_to_api_tree(core_node: code_viz_core::models::TreeNode) -> code_viz_api::TreeNode {
    code_viz_api::TreeNode {
        id: core_node.id,
        name: core_node.name,
        path: core_node.path,
        loc: core_node.loc,
        complexity: core_node.complexity,
        node_type: core_node.node_type,
        children: core_node.children.into_iter().map(convert_core_to_api_tree).collect(),
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

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::SystemTime;

    fn create_test_file(path: &str, loc: usize) -> FileMetrics {
        FileMetrics {
            path: PathBuf::from(path),
            language: "rust".to_string(),
            loc,
            size_bytes: 2048,
            function_count: 5,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            code_churn: None,
            coupling: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        }
    }

    #[test]
    fn test_empty_input() {
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(vec![]).unwrap();
        assert_eq!(tree.name, "root");
        assert_eq!(tree.loc, 0);
        assert_eq!(tree.children.len(), 0);
        assert_eq!(tree.node_type, "directory");
    }

    #[test]
    fn test_single_file() {
        let files = vec![create_test_file("main.rs", 100)];
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

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
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

        assert_eq!(tree.name, "root");
        assert_eq!(tree.loc, 350); // Sum of all files
        assert_eq!(tree.complexity, 35);
        assert_eq!(tree.children.len(), 2); // "src" and "tests" directories

        // Check src directory
        // Test-only unwrap: Test fixtures guarantee these directories exist
        let src_dir = tree.children.iter().find(|c| c.name == "src").unwrap();
        assert_eq!(src_dir.loc, 300);
        assert_eq!(src_dir.complexity, 30);
        assert_eq!(src_dir.children.len(), 2);
        assert_eq!(src_dir.node_type, "directory");

        // Check tests directory
        // Test-only unwrap: Test fixtures guarantee these directories exist
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
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

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
    fn test_multiple_files_same_directory() {
        let files = vec![
            create_test_file("src/file1.rs", 100),
            create_test_file("src/file2.rs", 200),
            create_test_file("src/file3.rs", 300),
        ];
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

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
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

        assert_eq!(tree.loc, 230);
        assert_eq!(tree.children.len(), 3); // README.md, src, tests

        // Verify root level file
        // Test-only unwrap: Test fixtures guarantee these directories exist
        let readme = tree.children.iter().find(|c| c.name == "README.md").unwrap();
        assert_eq!(readme.node_type, "file");
        assert_eq!(readme.loc, 10);

        // Verify nested directories aggregate correctly
        // Test-only unwrap: Test fixtures guarantee these directories exist
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
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

        assert_eq!(tree.loc, 350);

        // Test-only unwrap: Test fixtures guarantee these directories exist
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
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

        assert_eq!(tree.loc, 600);
        assert_eq!(tree.children.len(), 3);

        // Each directory should have its own main.rs with correct LOC
        // Test-only unwrap: Test fixtures guarantee these directories exist
        let src = tree.children.iter().find(|c| c.name == "src").unwrap();
        let src_main = &src.children[0];
        assert_eq!(src_main.name, "main.rs");
        assert_eq!(src_main.loc, 100);

        // Test-only unwrap: Test fixtures guarantee these directories exist
        let tests = tree.children.iter().find(|c| c.name == "tests").unwrap();
        let tests_main = &tests.children[0];
        assert_eq!(tests_main.name, "main.rs");
        assert_eq!(tests_main.loc, 200);

        // Test-only unwrap: Test fixtures guarantee these directories exist
        let examples = tree.children.iter().find(|c| c.name == "examples").unwrap();
        let examples_main = &examples.children[0];
        assert_eq!(examples_main.name, "main.rs");
        assert_eq!(examples_main.loc, 300);
    }

    #[test]
    fn test_very_long_path() {
        let long_path = "a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/file.rs";
        let files = vec![create_test_file(long_path, 50)];
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

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
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

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
                dead_function_count: None,
                dead_code_loc: None,
                dead_code_ratio: None,
                code_churn: None,
                coupling: None,
                ai_bloat_index: None,
                cognitive_complexity: None,
                test_coverage: None,
            },
            FileMetrics {
                path: PathBuf::from("src/older.rs"),
                language: "rust".to_string(),
                loc: 100,
                size_bytes: 2048,
                function_count: 5,
                last_modified: older,
                dead_function_count: None,
                dead_code_loc: None,
                dead_code_ratio: None,
                code_churn: None,
                coupling: None,
                ai_bloat_index: None,
                cognitive_complexity: None,
                test_coverage: None,
            },
            FileMetrics {
                path: PathBuf::from("src/newest.rs"),
                language: "rust".to_string(),
                loc: 100,
                size_bytes: 2048,
                function_count: 5,
                last_modified: now,
                dead_function_count: None,
                dead_code_loc: None,
                dead_code_ratio: None,
                code_churn: None,
                coupling: None,
                ai_bloat_index: None,
                cognitive_complexity: None,
                test_coverage: None,
            },
        ];

        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

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
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

        assert_eq!(tree.loc, 530);
        assert_eq!(tree.children.len(), 3); // frontend, backend, shared

        // Test-only unwrap: Test fixtures guarantee these directories exist
        let frontend = tree.children.iter().find(|c| c.name == "frontend").unwrap();
        assert_eq!(frontend.loc, 150);

        // Test-only unwrap: Test fixtures guarantee these directories exist
        let backend = tree.children.iter().find(|c| c.name == "backend").unwrap();
        assert_eq!(backend.loc, 350);

        // Test-only unwrap: Test fixtures guarantee these directories exist
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
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();
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
        // Test-only unwrap: Test fixtures are valid by construction
        let tree = flat_to_hierarchy(files).unwrap();

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

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
#[path = "transform.test.rs"]
mod transform_test;
