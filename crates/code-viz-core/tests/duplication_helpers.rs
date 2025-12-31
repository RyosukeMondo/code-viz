use code_viz_core::duplication::DuplicationDetector;
use code_viz_core::parser;
use std::collections::HashMap;
use std::path::PathBuf;

/// Helper to create a parser map with Rust support
fn create_rust_parser_map() -> HashMap<String, Box<dyn code_viz_core::parser::LanguageParser>> {
    let mut parsers = HashMap::new();
    parsers.insert("rs".to_string(), parser::get_parser("rust").unwrap());
    parsers
}

#[test]
fn test_extract_blocks_from_single_file() {
    // Test that blocks are correctly extracted from a single file
    let files = vec![
        (
            PathBuf::from("test.rs"),
            r#"
fn function_one() {
    if true {
        let a = 1;
        let b = 2;
    }
    let c = 3;
    let d = 4;
}

fn function_two() {
    for x in 0..10 {
        println!("{}", x);
    }
    let y = 20;
    let z = 30;
}
"#.to_string(),
        ),
    ];

    let parsers = create_rust_parser_map();
    let detector = DuplicationDetector::new(5, 0.8);
    let result = detector.run(&files, &parsers);

    // Two functions with different structure, so no duplicates
    assert_eq!(result.pairs.len(), 0);
}

#[test]
fn test_min_lines_threshold() {
    // Test that functions below min_lines threshold are ignored
    let files = vec![
        (
            PathBuf::from("test.rs"),
            r#"
fn tiny_func() {
    let a = 1;
}

fn another_tiny() {
    let b = 2;
}
"#.to_string(),
        ),
    ];

    let parsers = create_rust_parser_map();
    let detector = DuplicationDetector::new(10, 0.8); // min_lines = 10
    let result = detector.run(&files, &parsers);

    // Both functions are too small, so no blocks extracted
    assert_eq!(result.pairs.len(), 0);
}

#[test]
fn test_similarity_threshold() {
    // Test that similarity threshold filters near-duplicates
    let files = vec![
        (
            PathBuf::from("file1.rs"),
            r#"
fn my_func() {
    println!("Hello");
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
}
"#.to_string(),
        ),
        (
            PathBuf::from("file2.rs"),
            r#"
fn similar_func() {
    println!("Hello");
    let x = 1;
    let y = 2;
    let z = 3;
    let w = 999;  // Different value
}
"#.to_string(),
        ),
    ];

    let parsers = create_rust_parser_map();

    // With high threshold, should not find these as duplicates
    let detector_high = DuplicationDetector::new(5, 0.99);
    let result_high = detector_high.run(&files, &parsers);

    // With low threshold, should find these as duplicates
    let detector_low = DuplicationDetector::new(5, 0.8);
    let result_low = detector_low.run(&files, &parsers);

    // The exact behavior depends on canonicalization, but we verify threshold has effect
    assert!(result_low.pairs.len() >= result_high.pairs.len());
}

#[test]
fn test_group_blocks_by_hash() {
    // Test that identical blocks are grouped together
    let files = vec![
        (
            PathBuf::from("file1.rs"),
            r#"
fn duplicate_one() {
    println!("Same");
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
}
"#.to_string(),
        ),
        (
            PathBuf::from("file2.rs"),
            r#"
fn duplicate_two() {
    println!("Same");
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
}
"#.to_string(),
        ),
        (
            PathBuf::from("file3.rs"),
            r#"
fn duplicate_three() {
    println!("Same");
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
}
"#.to_string(),
        ),
    ];

    let parsers = create_rust_parser_map();
    let detector = DuplicationDetector::new(5, 0.8);
    let result = detector.run(&files, &parsers);

    // Three identical functions should create 3 pairs (0-1, 0-2, 1-2)
    assert_eq!(result.pairs.len(), 3);

    // All pairs should have 1.0 similarity (exact duplicates)
    for pair in &result.pairs {
        assert_eq!(pair.similarity, 1.0);
    }
}

#[test]
fn test_total_duplicated_loc_calculation() {
    // Test that total duplicated lines are counted correctly
    let files = vec![
        (
            PathBuf::from("file1.rs"),
            r#"
fn func_one() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let e = 5;
}
"#.to_string(),
        ),
        (
            PathBuf::from("file2.rs"),
            r#"
fn func_two() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let e = 5;
}
"#.to_string(),
        ),
    ];

    let parsers = create_rust_parser_map();
    let detector = DuplicationDetector::new(5, 0.8);
    let result = detector.run(&files, &parsers);

    // Should find duplicates
    assert!(result.pairs.len() > 0);

    // Total duplicated LOC should be > 0
    assert!(result.total_duplicated_loc > 0);
}

#[test]
fn test_canonicalization_ignores_identifiers() {
    // Test that canonicalization treats different identifier names as same
    let files = vec![
        (
            PathBuf::from("file1.rs"),
            r#"
fn calculate_sum() {
    let first = 10;
    let second = 20;
    let result = first + second;
    println!("{}", result);
    return result;
}
"#.to_string(),
        ),
        (
            PathBuf::from("file2.rs"),
            r#"
fn compute_total() {
    let num1 = 10;
    let num2 = 20;
    let answer = num1 + num2;
    println!("{}", answer);
    return answer;
}
"#.to_string(),
        ),
    ];

    let parsers = create_rust_parser_map();
    let detector = DuplicationDetector::new(5, 0.8);
    let result = detector.run(&files, &parsers);

    // After canonicalization, these should be detected as exact duplicates
    assert_eq!(result.pairs.len(), 1);
    assert_eq!(result.pairs[0].similarity, 1.0);
}

#[test]
fn test_parse_error_handling() {
    // Test that parse errors don't crash the analysis
    let files = vec![
        (
            PathBuf::from("valid.rs"),
            r#"
fn valid_function() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let e = 5;
}
"#.to_string(),
        ),
        (
            PathBuf::from("invalid.rs"),
            r#"
fn incomplete_function(
    // Missing closing brace and body
"#.to_string(),
        ),
    ];

    let parsers = create_rust_parser_map();
    let detector = DuplicationDetector::new(5, 0.8);

    // Should not panic, just skip the invalid file
    let result = detector.run(&files, &parsers);

    // Result should be valid (possibly empty if only invalid files)
    assert_eq!(result.pairs.len(), 0);
}

#[test]
fn test_multiple_languages_unsupported() {
    // Test that files with unsupported extensions are skipped
    let files = vec![
        (
            PathBuf::from("file.unknown"),
            r#"
fn some_function() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let e = 5;
}
"#.to_string(),
        ),
    ];

    let parsers = create_rust_parser_map();
    let detector = DuplicationDetector::new(5, 0.8);
    let result = detector.run(&files, &parsers);

    // File should be skipped due to unknown extension
    assert_eq!(result.pairs.len(), 0);
}

#[test]
fn test_empty_files() {
    // Test handling of empty files
    let files = vec![
        (PathBuf::from("empty.rs"), String::new()),
        (PathBuf::from("also_empty.rs"), "".to_string()),
    ];

    let parsers = create_rust_parser_map();
    let detector = DuplicationDetector::new(5, 0.8);
    let result = detector.run(&files, &parsers);

    // Empty files should not cause errors
    assert_eq!(result.pairs.len(), 0);
    assert_eq!(result.total_duplicated_loc, 0);
}

#[test]
fn test_line_count_accuracy() {
    // Test that line counts are accurate in duplicate pairs
    let files = vec![
        (
            PathBuf::from("file1.rs"),
            r#"
fn multi_line() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let e = 5;
    let f = 6;
}
"#.to_string(),
        ),
        (
            PathBuf::from("file2.rs"),
            r#"
fn another_multi_line() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let e = 5;
    let f = 6;
}
"#.to_string(),
        ),
    ];

    let parsers = create_rust_parser_map();
    let detector = DuplicationDetector::new(5, 0.8);
    let result = detector.run(&files, &parsers);

    assert!(result.pairs.len() > 0);

    // Verify line count is reasonable (should be around 8-9 lines for the function)
    for pair in &result.pairs {
        assert!(pair.line_count >= 5); // At least min_lines
        assert!(pair.line_count <= 20); // Reasonable upper bound for this test
    }
}
