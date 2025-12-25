use code_viz_core::duplication::DuplicationDetector;
use code_viz_core::parser;
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_exact_duplicates() {
    let files = vec![
        (
            PathBuf::from("file1.rs"),
            "fn my_func() {\n    println!(\"Hello\");\n    println!(\"World\");\n    let x = 1;\n    let y = 2;\n    let z = 3;\n}".to_string(),
        ),
        (
            PathBuf::from("file2.rs"),
            // This is the same as above but with different variable names
            "fn another_func() {\n    println!(\"Hello\");\n    println!(\"World\");\n    let x = 1;\n    let y = 2;\n    let z = 3;\n}".to_string(),
        ),
    ];

    let mut parsers = HashMap::new();
    parsers.insert("rs".to_string(), parser::get_parser("rust").unwrap());

    let detector = DuplicationDetector::new(5, 0.8);
    let result = detector.run(&files, &parsers);

    assert_eq!(result.pairs.len(), 1);
    assert_eq!(result.pairs[0].similarity, 1.0);
    assert_eq!(result.pairs[0].original.path, PathBuf::from("file1.rs"));
    assert_eq!(result.pairs[0].duplicate.path, PathBuf::from("file2.rs"));
}

#[test]
fn test_similar_duplicates() {
    let files = vec![
        (
            PathBuf::from("file1.rs"),
            "fn my_func() {\n    println!(\"Hello\");\n    println!(\"World\");\n    let a = 1;\n    let b = 2;\n    let c = 3;\n}".to_string(),
        ),
        (
            PathBuf::from("file2.rs"),
            "fn another_func() {\n    println!(\"Hello\");\n    println!(\"World\");\n    let x = 1;\n    let y = 2;\n    let z = 3;\n}".to_string(),
        ),
    ];

    let mut parsers = HashMap::new();
    parsers.insert("rs".to_string(), parser::get_parser("rust").unwrap());

    let detector = DuplicationDetector::new(5, 0.8);
    let result = detector.run(&files, &parsers);

    // After canonicalization (removing identifiers), these should be exact duplicates.
    assert_eq!(result.pairs.len(), 1);
    assert_eq!(result.pairs[0].similarity, 1.0);
}

#[test]
fn test_no_duplicates() {
    let files = vec![
        (
            PathBuf::from("file1.rs"),
            "fn my_func() {\n    println!(\"One\");\n    if true {\n        let a = 1;\n    }\n    let b = 2;\n    let c = 3;\n}".to_string(),
        ),
        (
            PathBuf::from("file2.rs"),
            "fn another_func() {\n    for i in 0..5 {\n        println!(\"{}\", i);\n    }\n    let x = 4;\n    let y = 5;\n}".to_string(),
        ),
    ];

    let mut parsers = HashMap::new();
    parsers.insert("rs".to_string(), parser::get_parser("rust").unwrap());

    let detector = DuplicationDetector::new(5, 0.9); // Increased threshold for safety
    let result = detector.run(&files, &parsers);

    assert_eq!(result.pairs.len(), 0);
}
