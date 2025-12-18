//! Additional tests for path handling in transform module
//!
//! These tests verify that absolute filesystem paths are correctly
//! converted to relative paths within the project.

#[cfg(test)]
mod path_handling_tests {
    use super::super::*;
    use code_viz_core::models::FileMetrics;
    use std::path::PathBuf;
    use std::time::SystemTime;

    fn create_file_with_abs_path(abs_path: &str, loc: usize) -> FileMetrics {
        FileMetrics {
            path: PathBuf::from(abs_path),
            language: "rust".to_string(),
            loc,
            size_bytes: 2048,
            function_count: 5,
            last_modified: SystemTime::now(),
        }
    }

    #[test]
    fn test_absolute_paths_converted_to_relative() {
        // Simulate files with absolute paths like what the analysis engine returns
        let files = vec![
            create_file_with_abs_path("/home/user/project/src/main.rs", 100),
            create_file_with_abs_path("/home/user/project/src/lib.rs", 200),
        ];

        let tree = flat_to_hierarchy(files);

        // The root should contain only relative paths, not the full filesystem path
        assert_eq!(tree.children.len(), 1);
        let src_dir = &tree.children[0];
        assert_eq!(src_dir.name, "src");

        // Paths should be relative to project root
        assert!(
            !src_dir.path.to_string_lossy().starts_with("/home/"),
            "Paths should be relative, not absolute. Got: {:?}",
            src_dir.path
        );
    }

    #[test]
    fn test_root_node_name_matches_project() {
        let files = vec![
            create_file_with_abs_path("/home/user/my-project/README.md", 50),
        ];

        let tree = flat_to_hierarchy(files);

        // Root name should be the project name, not generic "root"
        assert_eq!(tree.name, "my-project");
    }

    #[test]
    fn test_drill_down_path_with_relative_paths() {
        // When analyzing /home/user/project, all paths should be relative
        let files = vec![
            create_file_with_abs_path("/home/user/project/src/main.rs", 100),
            create_file_with_abs_path("/home/user/project/tests/test1.rs", 50),
        ];

        let tree = flat_to_hierarchy(files);

        // Verify that we can drill down using just the directory name
        // not the full absolute path
        assert_eq!(tree.children.len(), 2);

        let src_dir = tree.children.iter().find(|c| c.name == "src").unwrap();
        assert_eq!(src_dir.name, "src");

        // The path should be "src", not "/home/user/project/src"
        let path_str = src_dir.path.to_string_lossy();
        assert!(
            path_str == "src" || path_str == "./src",
            "Expected relative path 'src' or './src', got: {}",
            path_str
        );
    }
}
