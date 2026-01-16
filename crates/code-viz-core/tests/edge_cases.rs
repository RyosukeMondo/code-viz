#![allow(clippy::unwrap_used, clippy::expect_used)]
use code_viz_core::parser::get_parser;
use code_viz_core::scanner::scan_directory;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

// ============================================================================
// 1. EMPTY INPUT TESTS
// ============================================================================

#[test]
fn test_empty_file_parsing() {
    let parser = get_parser("rust").expect("Failed to get parser");
    let empty_code = "";
    let result = parser.parse(empty_code);
    assert!(result.is_ok(), "Parser should handle empty files without panicking");
    let tree = result.unwrap();
    assert_eq!(parser.count_functions(&tree), 0, "Empty file should have 0 functions");
}

#[test]
fn test_empty_repository() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should handle empty directories");
    let files = result.unwrap();
    assert!(files.is_empty(), "Empty directory should return no files");
}

#[test]
fn test_whitespace_only_file() {
    let parser = get_parser("typescript").expect("Failed to get parser");
    let whitespace_code = "    \n\n\t\t  \n  ";
    let result = parser.parse(whitespace_code);
    assert!(result.is_ok(), "Parser should handle whitespace-only files");
    let tree = result.unwrap();
    assert_eq!(parser.count_functions(&tree), 0, "Whitespace-only file should have 0 functions");
}

#[test]
fn test_comments_only_file() {
    let parser = get_parser("rust").expect("Failed to get parser");
    let comments_only = r#"
        // This is a comment
        /* Multi-line
           comment
        */
        // Another comment
    "#;
    let result = parser.parse(comments_only);
    assert!(result.is_ok(), "Parser should handle comments-only files");
}

// ============================================================================
// 2. EXTREME VALUES TESTS
// ============================================================================

#[test]
fn test_very_large_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let large_file_path = temp_dir.path().join("large.rs");

    let mut file = File::create(&large_file_path).expect("Failed to create file");

    // Create a file with 50K+ lines (should still parse, but >10MB files should be skipped by scanner)
    for i in 0..50000 {
        writeln!(file, "fn function_{}() {{ println!(\"test\"); }}", i)
            .expect("Failed to write to file");
    }
    drop(file);

    let parser = get_parser("rust").expect("Failed to get parser");
    let content = fs::read_to_string(&large_file_path).expect("Failed to read file");
    let result = parser.parse(&content);
    assert!(result.is_ok(), "Parser should handle large files without panicking");
}

#[test]
fn test_very_deep_directory_nesting() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let mut current_path = temp_dir.path().to_path_buf();

    // Create deeply nested directory structure (100 levels)
    for i in 0..100 {
        current_path = current_path.join(format!("level_{}", i));
        fs::create_dir(&current_path).expect("Failed to create nested dir");
    }

    // Create a file at the deepest level
    let file_path = current_path.join("deep.rs");
    fs::write(&file_path, "fn test() {}").expect("Failed to write file");

    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should handle deep nesting");
    let files = result.unwrap();
    assert_eq!(files.len(), 1, "Should find the deeply nested file");
}

#[test]
fn test_many_small_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create 1000 small files
    for i in 0..1000 {
        let file_path = temp_dir.path().join(format!("file_{}.rs", i));
        fs::write(&file_path, format!("fn func_{}() {{}}", i))
            .expect("Failed to write file");
    }

    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should handle many files");
    let files = result.unwrap();
    assert_eq!(files.len(), 1000, "Should find all 1000 files");
}

#[test]
fn test_extremely_long_line() {
    let parser = get_parser("javascript").expect("Failed to get parser");

    // Create a line with 10K characters
    let long_string = "a".repeat(10000);
    let code = format!("const x = \"{}\";", long_string);

    let result = parser.parse(&code);
    assert!(result.is_ok(), "Parser should handle extremely long lines");
}

#[test]
fn test_deeply_nested_code_blocks() {
    let parser = get_parser("python").expect("Failed to get parser");

    // Create deeply nested if statements (50 levels)
    let mut code = String::new();
    for i in 0..50 {
        code.push_str(&format!("{}if True:\n", "    ".repeat(i)));
    }
    code.push_str(&format!("{}pass\n", "    ".repeat(50)));

    let result = parser.parse(&code);
    assert!(result.is_ok(), "Parser should handle deeply nested code");
}

// ============================================================================
// 3. SPECIAL CHARACTERS TESTS
// ============================================================================

#[test]
fn test_unicode_in_code() {
    let parser = get_parser("typescript").expect("Failed to get parser");
    let unicode_code = r#"
        const greeting = "Hello 世界 🌍";
        const emoji = "🚀 💻 ✨";
        function greet(name: string = "用户") {
            console.log(`${greeting} ${name}`);
        }
    "#;

    let result = parser.parse(unicode_code);
    assert!(result.is_ok(), "Parser should handle Unicode characters");
    let tree = result.unwrap();
    assert!(parser.count_functions(&tree) >= 1, "Should count functions with Unicode");
}

#[test]
fn test_special_characters_in_strings() {
    let parser = get_parser("rust").expect("Failed to get parser");
    let special_chars = r#"
        fn test() {
            let s1 = "tab\there";
            let s2 = "newline\nhere";
            let s3 = "quote\"here";
            let s4 = "backslash\\here";
            let s5 = "null\0byte";
        }
    "#;

    let result = parser.parse(special_chars);
    assert!(result.is_ok(), "Parser should handle special characters in strings");
}

#[test]
fn test_unicode_filename_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create files with Unicode names
    let unicode_filenames = vec![
        "日本語.rs",
        "français.ts",
        "español.py",
        "emoji_🚀.js",
        "中文文件.go",
    ];

    for filename in &unicode_filenames {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, "fn test() {}").expect("Failed to write Unicode filename");
    }

    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should handle Unicode filenames");
    let files = result.unwrap();
    assert_eq!(files.len(), unicode_filenames.len(), "Should find all Unicode-named files");
}

#[test]
fn test_spaces_in_paths() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create directory with spaces
    let space_dir = temp_dir.path().join("my folder with spaces");
    fs::create_dir(&space_dir).expect("Failed to create dir with spaces");

    let file_path = space_dir.join("my file.rs");
    fs::write(&file_path, "fn test() {}").expect("Failed to write file");

    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should handle spaces in paths");
    let files = result.unwrap();
    assert_eq!(files.len(), 1, "Should find file in directory with spaces");
}

#[test]
fn test_regex_special_chars_in_code() {
    let parser = get_parser("javascript").expect("Failed to get parser");
    let regex_code = r#"
        const pattern1 = /[a-z]+/gi;
        const pattern2 = /\d{3}-\d{3}-\d{4}/;
        const pattern3 = /(foo|bar)+/;
        const pattern4 = /^.*$/m;
    "#;

    let result = parser.parse(regex_code);
    assert!(result.is_ok(), "Parser should handle regex special characters");
}

// ============================================================================
// 4. CIRCULAR DEPENDENCY TESTS
// ============================================================================

#[test]
fn test_circular_import_structure() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create module A that imports B
    let module_a = temp_dir.path().join("module_a.ts");
    fs::write(&module_a, r#"
        import { b } from './module_b';
        export const a = "A";
    "#).expect("Failed to write module A");

    // Create module B that imports A (circular)
    let module_b = temp_dir.path().join("module_b.ts");
    fs::write(&module_b, r#"
        import { a } from './module_a';
        export const b = "B";
    "#).expect("Failed to write module B");

    let parser = get_parser("typescript").expect("Failed to get parser");

    // Parse both files individually (they should parse fine)
    let content_a = fs::read_to_string(&module_a).expect("Failed to read A");
    let content_b = fs::read_to_string(&module_b).expect("Failed to read B");

    let result_a = parser.parse(&content_a);
    let result_b = parser.parse(&content_b);

    assert!(result_a.is_ok(), "Module A should parse despite circular dependency");
    assert!(result_b.is_ok(), "Module B should parse despite circular dependency");
}

#[test]
fn test_transitive_circular_dependencies() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create A -> B -> C -> A circular chain
    fs::write(temp_dir.path().join("a.py"), "from b import func_b\ndef func_a(): pass")
        .expect("Failed to write a.py");
    fs::write(temp_dir.path().join("b.py"), "from c import func_c\ndef func_b(): pass")
        .expect("Failed to write b.py");
    fs::write(temp_dir.path().join("c.py"), "from a import func_a\ndef func_c(): pass")
        .expect("Failed to write c.py");

    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should handle transitive circular dependencies");
    assert_eq!(result.unwrap().len(), 3, "Should find all files in circular chain");
}

// ============================================================================
// 5. PLATFORM-SPECIFIC TESTS
// ============================================================================

#[cfg(unix)]
#[test]
fn test_symlink_handling() {
    use std::os::unix::fs as unix_fs;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let target_file = temp_dir.path().join("target.rs");
    fs::write(&target_file, "fn test() {}").expect("Failed to write target");

    let symlink = temp_dir.path().join("link.rs");
    unix_fs::symlink(&target_file, &symlink).expect("Failed to create symlink");

    // Scanner should not follow symlinks by default
    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should handle symlinks");
}

#[test]
fn test_windows_path_separators() {
    // Test that we can handle Windows-style paths even on non-Windows systems
    let parser = get_parser("typescript").expect("Failed to get parser");
    let code_with_windows_path = r#"
        const path = "C:\\Users\\Name\\file.ts";
        const another = "D:\\folder\\subfolder\\test.js";
    "#;

    let result = parser.parse(code_with_windows_path);
    assert!(result.is_ok(), "Parser should handle Windows path separators in strings");
}

#[test]
fn test_case_sensitivity_patterns() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create files with different casing
    fs::write(temp_dir.path().join("Test.rs"), "fn test() {}").expect("Failed to write Test.rs");
    fs::write(temp_dir.path().join("TEST.rs"), "fn test() {}").expect("Failed to write TEST.rs");
    fs::write(temp_dir.path().join("test.rs"), "fn test() {}").expect("Failed to write test.rs");

    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should handle case-sensitive filenames");

    #[cfg(unix)]
    {
        // On Unix, all three files should be found
        assert_eq!(result.unwrap().len(), 3, "Unix should find all case-variant files");
    }
}

#[test]
fn test_dot_files_and_hidden_directories() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create hidden directory and file
    let hidden_dir = temp_dir.path().join(".hidden");
    fs::create_dir(&hidden_dir).expect("Failed to create hidden dir");
    fs::write(hidden_dir.join("test.rs"), "fn test() {}").expect("Failed to write hidden file");

    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should handle hidden files/directories");
}

// ============================================================================
// 6. MALFORMED AND BOUNDARY TESTS
// ============================================================================

#[test]
fn test_malformed_code_does_not_panic() {
    let parser = get_parser("rust").expect("Failed to get parser");

    let malformed_cases = vec![
        "fn { } incomplete",
        "struct NoBody;;;;;",
        "impl { { { }",
        "fn test( incomplete_params",
        "let x = ",
        "if true { no closing brace",
        "match x { ",
        "fn test() { return }}}}}",
    ];

    for code in malformed_cases {
        let result = parser.parse(code);
        assert!(result.is_ok(), "Parser should not panic on malformed code: {}", code);
    }
}

#[test]
fn test_mixed_line_endings() {
    let parser = get_parser("javascript").expect("Failed to get parser");

    // Mix Unix (LF), Windows (CRLF), and old Mac (CR) line endings
    let mixed_code = "function test1() {\n  return 1;\n}\r\nfunction test2() {\r\n  return 2;\r\n}\rfunction test3() {\r  return 3;\r}";

    let result = parser.parse(mixed_code);
    assert!(result.is_ok(), "Parser should handle mixed line endings");
}

#[test]
fn test_null_bytes_in_input() {
    let parser = get_parser("python").expect("Failed to get parser");

    let code_with_null = "def test():\0    pass";

    let result = parser.parse(code_with_null);
    // Parser may or may not succeed, but it should not panic
    let _ = result;
}

#[test]
fn test_binary_file_handling() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a binary file with .rs extension
    let binary_file = temp_dir.path().join("binary.rs");
    let binary_data: Vec<u8> = (0..=255).collect();
    fs::write(&binary_file, binary_data).expect("Failed to write binary file");

    let parser = get_parser("rust").expect("Failed to get parser");
    let content = fs::read_to_string(&binary_file);

    if let Ok(text) = content {
        let result = parser.parse(&text);
        assert!(result.is_ok(), "Parser should not panic on binary data interpreted as text");
    }
}

#[test]
fn test_zero_byte_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let empty_file = temp_dir.path().join("empty.rs");
    File::create(&empty_file).expect("Failed to create empty file");

    let parser = get_parser("rust").expect("Failed to get parser");
    let content = fs::read_to_string(&empty_file).expect("Failed to read empty file");

    let result = parser.parse(&content);
    assert!(result.is_ok(), "Parser should handle zero-byte files");
}

#[test]
fn test_unsupported_file_extensions_filtered() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create files with unsupported extensions
    fs::write(temp_dir.path().join("test.txt"), "text file").expect("Failed to write txt");
    fs::write(temp_dir.path().join("test.md"), "markdown").expect("Failed to write md");
    fs::write(temp_dir.path().join("test.json"), "{}").expect("Failed to write json");
    fs::write(temp_dir.path().join("test.rs"), "fn test() {}").expect("Failed to write rs");

    let result = scan_directory(temp_dir.path(), &[]);
    assert!(result.is_ok(), "Scanner should filter unsupported extensions");
    let files = result.unwrap();
    assert_eq!(files.len(), 1, "Should only find the .rs file");
}

// ============================================================================
// 7. PROPERTY-BASED TESTS (using proptest)
// ============================================================================

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn parser_never_panics_on_random_rust_input(s in "\\PC*") {
            let parser = get_parser("rust").expect("Failed to get parser");
            let _ = parser.parse(&s);
            // Test passes if no panic occurs
        }

        #[test]
        fn parser_never_panics_on_random_typescript_input(s in "\\PC*") {
            let parser = get_parser("typescript").expect("Failed to get parser");
            let _ = parser.parse(&s);
            // Test passes if no panic occurs
        }

        #[test]
        fn parser_never_panics_on_random_python_input(s in "\\PC*") {
            let parser = get_parser("python").expect("Failed to get parser");
            let _ = parser.parse(&s);
            // Test passes if no panic occurs
        }

        #[test]
        fn function_count_always_valid(s in "fn [a-z_][a-z0-9_]* ?\\(\\) ?\\{ ?\\}") {
            let parser = get_parser("rust").expect("Failed to get parser");
            if let Ok(tree) = parser.parse(&s) {
                let count = parser.count_functions(&tree);
                // Function count is usize, so it's always non-negative, but verify it's reasonable
                assert!(count < 1000, "Function count should be reasonable for short input");
            }
        }

        #[test]
        fn empty_string_variants_always_parse(whitespace in "[ \t\n\r]*") {
            let parser = get_parser("javascript").expect("Failed to get parser");
            let result = parser.parse(&whitespace);
            assert!(result.is_ok(), "Whitespace-only input should always parse");
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        #[test]
        fn scanner_never_panics_on_exclude_patterns(pattern in "[a-z*/._-]+") {
            let temp_dir = TempDir::new().expect("Failed to create temp dir");
            let _ = scan_directory(temp_dir.path(), &[pattern]);
            // Test passes if no panic occurs
        }

        #[test]
        fn deeply_nested_structures_parse(depth in 1usize..30) {
            let parser = get_parser("python").expect("Failed to get parser");
            let mut code = String::new();
            for i in 0..depth {
                code.push_str(&format!("{}if True:\n", "    ".repeat(i)));
            }
            code.push_str(&format!("{}pass\n", "    ".repeat(depth)));

            let result = parser.parse(&code);
            assert!(result.is_ok(), "Should parse nested depth: {}", depth);
        }

        #[test]
        fn repeated_code_patterns_parse(repetitions in 1usize..100) {
            let parser = get_parser("rust").expect("Failed to get parser");
            let mut code = String::new();
            for i in 0..repetitions {
                code.push_str(&format!("fn func_{}() {{}}\n", i));
            }

            let result = parser.parse(&code);
            assert!(result.is_ok(), "Should parse {} repetitions", repetitions);
        }
    }
}

// ============================================================================
// 8. INVARIANT TESTS
// ============================================================================

#[test]
fn invariant_tree_structure_always_valid() {
    let parser = get_parser("rust").expect("Failed to get parser");
    let code = "fn test() { fn nested() {} }";

    let result = parser.parse(code);
    assert!(result.is_ok(), "Should produce valid tree");

    let tree = result.unwrap();
    let root = tree.root_node();

    // Invariant: root node always exists and has a kind
    assert!(!root.kind().is_empty(), "Root should have a kind");

    // Invariant: tree is properly formed (start <= end)
    assert!(root.start_byte() <= root.end_byte(), "Start should be before or at end");
}

#[test]
fn invariant_function_count_consistent() {
    let parser = get_parser("typescript").expect("Failed to get parser");

    let codes = vec![
        ("", 0),
        ("const x = 1;", 0),
        ("function test() {}", 1),
        ("function a() {} function b() {}", 2),
        ("const fn = () => {}", 1),
        ("class Test { method() {} }", 1),
    ];

    for (code, expected_count) in codes {
        let tree = parser.parse(code).expect("Parse should succeed");
        let count = parser.count_functions(&tree);
        assert_eq!(count, expected_count, "Function count should match for: {}", code);
    }
}

#[test]
fn invariant_scanner_returns_sorted_paths() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    fs::write(temp_dir.path().join("z.rs"), "").expect("Write z.rs");
    fs::write(temp_dir.path().join("a.rs"), "").expect("Write a.rs");
    fs::write(temp_dir.path().join("m.rs"), "").expect("Write m.rs");

    let files = scan_directory(temp_dir.path(), &[]).expect("Scan should succeed");

    // Verify deterministic ordering
    assert_eq!(files.len(), 3);
    for window in files.windows(2) {
        assert!(window[0] <= window[1], "Files should be in deterministic order");
    }
}
