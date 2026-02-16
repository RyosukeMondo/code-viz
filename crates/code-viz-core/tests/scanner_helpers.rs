#![allow(clippy::unwrap_used, clippy::expect_used)]
use code_viz_core::scanner::scan_directory;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a test directory structure
fn create_test_directory() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create some test files
    fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
    fs::write(base_path.join("test.ts"), "function main() {}").unwrap();
    fs::write(base_path.join("test.py"), "def main(): pass").unwrap();
    fs::write(base_path.join("README.md"), "# Test").unwrap(); // Unsupported extension

    // Create a subdirectory
    fs::create_dir(base_path.join("subdir")).unwrap();
    fs::write(base_path.join("subdir/nested.rs"), "fn nested() {}").unwrap();

    temp_dir
}

#[test]
fn test_scan_directory_basic() {
    let temp_dir = create_test_directory();
    let result = scan_directory(temp_dir.path(), &[]);

    assert!(result.is_ok());
    let files = result.unwrap();

    // Should find .rs, .ts, and .py files
    assert!(files.len() >= 3);

    // Verify supported extensions are included
    assert!(files
        .iter()
        .any(|f| f.extension().and_then(|e| e.to_str()) == Some("rs")));
    assert!(files
        .iter()
        .any(|f| f.extension().and_then(|e| e.to_str()) == Some("ts")));
    assert!(files
        .iter()
        .any(|f| f.extension().and_then(|e| e.to_str()) == Some("py")));

    // Verify unsupported extensions are excluded
    assert!(!files
        .iter()
        .any(|f| f.extension().and_then(|e| e.to_str()) == Some("md")));
}

#[test]
fn test_validate_path_nonexistent() {
    let nonexistent = PathBuf::from("/this/path/does/not/exist");
    let result = scan_directory(&nonexistent, &[]);

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("not found") || error.to_string().contains("Not found"));
}

#[test]
fn test_validate_path_not_directory() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "test content").unwrap();

    let result = scan_directory(&file_path, &[]);

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        error.to_string().contains("not a directory")
            || error.to_string().contains("Not a directory")
    );
}

#[test]
fn test_exclude_patterns() {
    let temp_dir = create_test_directory();
    let base_path = temp_dir.path();

    // Create files in node_modules (commonly excluded)
    fs::create_dir(base_path.join("node_modules")).unwrap();
    fs::write(base_path.join("node_modules/lib.js"), "export {}").unwrap();

    // Scan with exclusion pattern
    let exclude_patterns = vec!["**/node_modules/**".to_string()];
    let result = scan_directory(base_path, &exclude_patterns);

    assert!(result.is_ok());
    let files = result.unwrap();

    // Should not include files in node_modules
    assert!(!files
        .iter()
        .any(|f| f.to_str().unwrap().contains("node_modules")));
}

#[test]
fn test_supported_extensions() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create files with various extensions
    let extensions = vec![
        ("test.ts", true),
        ("test.tsx", true),
        ("test.js", true),
        ("test.jsx", true),
        ("test.rs", true),
        ("test.py", true),
        ("test.go", true),
        ("test.cpp", true),
        ("test.cc", true),
        ("test.cxx", true),
        ("test.hpp", true),
        ("test.h", true),
        ("test.txt", false),  // Not supported
        ("test.md", false),   // Not supported
        ("test.json", false), // Not supported
    ];

    for (filename, _) in &extensions {
        fs::write(base_path.join(filename), "test content").unwrap();
    }

    let result = scan_directory(base_path, &[]);
    assert!(result.is_ok());
    let files = result.unwrap();

    // Verify supported extensions are included
    for (filename, should_include) in extensions {
        let found = files
            .iter()
            .any(|f| f.file_name().and_then(|n| n.to_str()) == Some(filename));
        if should_include {
            assert!(found, "Expected {} to be included", filename);
        } else {
            assert!(!found, "Expected {} to be excluded", filename);
        }
    }
}

#[test]
fn test_glob_set_building() {
    let temp_dir = create_test_directory();

    // Test with valid glob patterns
    let valid_patterns = vec![
        "*.log".to_string(),
        "**/test/**".to_string(),
        "build/*".to_string(),
    ];

    let result = scan_directory(temp_dir.path(), &valid_patterns);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_glob_pattern() {
    let temp_dir = create_test_directory();

    // Test with invalid glob pattern
    let invalid_patterns = vec!["[invalid".to_string()]; // Unclosed bracket

    let result = scan_directory(temp_dir.path(), &invalid_patterns);

    // Should return error for invalid pattern
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("pattern") || error.to_string().contains("glob"));
}

#[test]
fn test_gitignore_respected() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create .gitignore file
    fs::write(base_path.join(".gitignore"), "ignored.rs\n").unwrap();

    // Create files
    fs::write(base_path.join("included.rs"), "fn main() {}").unwrap();
    fs::write(base_path.join("ignored.rs"), "fn ignored() {}").unwrap();

    let result = scan_directory(base_path, &[]);
    assert!(result.is_ok());
    let files = result.unwrap();

    // Should include included.rs
    assert!(files
        .iter()
        .any(|f| f.file_name().and_then(|n| n.to_str()) == Some("included.rs")));

    // Should respect .gitignore and exclude ignored.rs
    assert!(!files
        .iter()
        .any(|f| f.file_name().and_then(|n| n.to_str()) == Some("ignored.rs")));
}

#[test]
fn test_nested_directories() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create nested directory structure
    fs::create_dir_all(base_path.join("level1/level2/level3")).unwrap();
    fs::write(base_path.join("level1/file1.rs"), "fn level1() {}").unwrap();
    fs::write(base_path.join("level1/level2/file2.rs"), "fn level2() {}").unwrap();
    fs::write(
        base_path.join("level1/level2/level3/file3.rs"),
        "fn level3() {}",
    )
    .unwrap();

    let result = scan_directory(base_path, &[]);
    assert!(result.is_ok());
    let files = result.unwrap();

    // Should find files at all nesting levels
    assert!(files
        .iter()
        .any(|f| f.to_str().unwrap().contains("file1.rs")));
    assert!(files
        .iter()
        .any(|f| f.to_str().unwrap().contains("file2.rs")));
    assert!(files
        .iter()
        .any(|f| f.to_str().unwrap().contains("file3.rs")));
}

#[test]
fn test_large_file_skipping() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create a small file (should be included)
    fs::write(base_path.join("small.rs"), "fn small() {}").unwrap();

    // We can't easily test large file handling without creating actual large files,
    // but we can verify small files are included
    let result = scan_directory(base_path, &[]);
    assert!(result.is_ok());
    let files = result.unwrap();

    assert!(files
        .iter()
        .any(|f| f.file_name().and_then(|n| n.to_str()) == Some("small.rs")));
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok());
    let files = result.unwrap();

    // Empty directory should return empty list
    assert_eq!(files.len(), 0);
}

#[test]
fn test_hidden_files() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create regular and hidden files
    fs::write(base_path.join("visible.rs"), "fn visible() {}").unwrap();
    fs::write(base_path.join(".hidden.rs"), "fn hidden() {}").unwrap();

    let result = scan_directory(base_path, &[]);
    assert!(result.is_ok());
    let files = result.unwrap();

    // Should include visible file
    assert!(files
        .iter()
        .any(|f| f.file_name().and_then(|n| n.to_str()) == Some("visible.rs")));

    // Hidden files handling depends on configuration
    // The scanner uses hidden(true) which means it processes hidden files,
    // but .gitignore might still exclude them
}

#[test]
fn test_multiple_exclude_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create various files
    fs::create_dir(base_path.join("build")).unwrap();
    fs::create_dir(base_path.join("dist")).unwrap();
    fs::create_dir(base_path.join("src")).unwrap();

    fs::write(base_path.join("build/output.rs"), "fn build() {}").unwrap();
    fs::write(base_path.join("dist/bundle.rs"), "fn dist() {}").unwrap();
    fs::write(base_path.join("src/main.rs"), "fn main() {}").unwrap();

    // Exclude build and dist directories
    let exclude_patterns = vec!["**/build/**".to_string(), "**/dist/**".to_string()];

    let result = scan_directory(base_path, &exclude_patterns);
    assert!(result.is_ok());
    let files = result.unwrap();

    // Should include src/main.rs
    assert!(files
        .iter()
        .any(|f| f.to_str().unwrap().contains("src/main.rs")));

    // Should exclude build and dist
    assert!(!files.iter().any(|f| f.to_str().unwrap().contains("build")));
    assert!(!files.iter().any(|f| f.to_str().unwrap().contains("dist")));
}
