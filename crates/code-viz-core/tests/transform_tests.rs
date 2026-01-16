#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for the transform module

use code_viz_core::models::FileMetrics;
use code_viz_core::transform::flat_to_hierarchy;
use std::path::PathBuf;
use std::time::SystemTime;

#[test]
fn test_empty_file_list_returns_empty_root() {
    let files = vec![];
    let tree = flat_to_hierarchy(files).expect("Should handle empty file list");

    assert_eq!(tree.name, "root");
    assert_eq!(tree.loc, 0);
    assert_eq!(tree.node_type, "directory");
    assert!(tree.children.is_empty());
}

#[test]
fn test_single_file_creates_tree() {
    let files = vec![
        FileMetrics {
            path: PathBuf::from("src/main.rs"),
            language: "rust".to_string(),
            loc: 100,
            size_bytes: 2048,
            function_count: 5,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            coupling: None,
            code_churn: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        },
    ];

    let tree = flat_to_hierarchy(files).expect("Should transform single file");

    assert_eq!(tree.name, "root");
    assert_eq!(tree.node_type, "directory");
    assert!(!tree.children.is_empty());
}

#[test]
fn test_multiple_files_creates_hierarchy() {
    let files = vec![
        FileMetrics {
            path: PathBuf::from("src/main.rs"),
            language: "rust".to_string(),
            loc: 100,
            size_bytes: 2048,
            function_count: 5,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            coupling: None,
            code_churn: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        },
        FileMetrics {
            path: PathBuf::from("src/lib.rs"),
            language: "rust".to_string(),
            loc: 50,
            size_bytes: 1024,
            function_count: 3,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            coupling: None,
            code_churn: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        },
    ];

    let tree = flat_to_hierarchy(files).expect("Should transform multiple files");

    assert_eq!(tree.name, "root");
    assert_eq!(tree.node_type, "directory");
    assert_eq!(tree.loc, 150); // Sum of both files
    assert!(!tree.children.is_empty());
}

#[test]
fn test_nested_directories_create_proper_hierarchy() {
    let files = vec![
        FileMetrics {
            path: PathBuf::from("src/main.rs"),
            language: "rust".to_string(),
            loc: 100,
            size_bytes: 2048,
            function_count: 5,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            coupling: None,
            code_churn: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        },
        FileMetrics {
            path: PathBuf::from("src/utils/helpers.rs"),
            language: "rust".to_string(),
            loc: 50,
            size_bytes: 1024,
            function_count: 3,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            coupling: None,
            code_churn: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        },
    ];

    let tree = flat_to_hierarchy(files).expect("Should transform nested files");

    assert_eq!(tree.name, "root");
    assert_eq!(tree.node_type, "directory");
    assert_eq!(tree.loc, 150); // Sum of all files
}

#[test]
fn test_complexity_calculation() {
    let files = vec![
        FileMetrics {
            path: PathBuf::from("large.rs"),
            language: "rust".to_string(),
            loc: 1000, // Should result in complexity of 100
            size_bytes: 20480,
            function_count: 50,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            coupling: None,
            code_churn: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        },
    ];

    let tree = flat_to_hierarchy(files).expect("Should transform file");

    assert_eq!(tree.complexity, 100); // loc/10, capped at 100
}
