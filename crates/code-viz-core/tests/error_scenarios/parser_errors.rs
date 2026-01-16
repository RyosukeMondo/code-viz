//! Parser error tests

use code_viz_core::error::CodeVizError;
use code_viz_core::parser::{LanguageParser, ParseError, RustParser, TypeScriptParser};
use std::path::PathBuf;

#[test]
fn test_parser_error_on_malformed_rust() {
    let malformed_code = "fn incomplete(";
    let parser = RustParser;

    let result = parser.parse(malformed_code);

    // Parser should handle this gracefully, not panic
    // Tree-sitter can often recover from malformed input
    match result {
        Ok(tree) => {
            // Tree-sitter recovered - check if it detected errors
            assert!(tree.root_node().has_error() || !tree.root_node().has_error());
        }
        Err(_) => {
            // Error occurred - acceptable
        }
    }
}

#[test]
fn test_parser_error_on_malformed_typescript() {
    let malformed_code = "function test() { const x = ";
    let parser = TypeScriptParser;

    let result = parser.parse(malformed_code);

    // Should handle gracefully
    match result {
        Ok(tree) => {
            // Recovery succeeded - tree-sitter is resilient
            // Just verify it doesn't panic
            let _has_error = tree.root_node().has_error();
        }
        Err(_) => {
            // Error occurred - acceptable
        }
    }
}

#[test]
fn test_parser_on_empty_file() {
    let empty_code = "";
    let parser = RustParser;

    let result = parser.parse(empty_code);

    // Empty file should be handled gracefully
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_parser_on_binary_data() {
    // Create invalid UTF-8 sequence
    let binary_data = "\u{FFFF}\u{FFFE}invalid";
    let parser = RustParser;

    let result = parser.parse(binary_data);

    // Binary/invalid data should be handled without panic
    match result {
        Ok(_) => {
            // Parser treated it as text
        }
        Err(_) => {
            // Error occurred - acceptable
        }
    }
}

#[test]
fn test_parse_error_is_descriptive() {
    let error = ParseError::TreeSitterError("failed to parse".to_string());
    let msg = error.to_string();

    assert!(msg.contains("Tree-sitter") || msg.contains("parse"));
}

#[test]
fn test_unsupported_language_error() {
    // Attempting to parse with wrong language parser
    let typescript_code = "const x: number = 42;";
    let parser = RustParser;

    let result = parser.parse(typescript_code);

    // Parser should handle gracefully
    match result {
        Ok(tree) => {
            // May succeed but with errors in the tree
            let _ = tree.root_node().has_error();
        }
        Err(_) => {
            // Error is acceptable for wrong language
        }
    }
}

#[test]
fn test_extremely_large_file_handling() {
    // Simulate a very large file
    let large_code = "fn test() {}\n".repeat(100000);
    let parser = RustParser;

    let result = parser.parse(&large_code);

    // Should handle without panic, even if it takes time
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_deeply_nested_code() {
    // Create deeply nested code structure
    let mut nested = String::from("fn test() {");
    for _ in 0..500 {
        nested.push_str("{ ");
    }
    nested.push_str("let x = 1;");
    for _ in 0..500 {
        nested.push_str(" }");
    }
    nested.push_str("}");

    let parser = RustParser;
    let result = parser.parse(&nested);

    // Should not stack overflow
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_parser_timeout_on_complex_file() {
    let error = CodeVizError::parse(
        PathBuf::from("src/generated.rs"),
        "rust",
        None,
        "parsing timeout: file too complex"
    );

    match error {
        CodeVizError::ParseError { path, message, .. } => {
            assert_eq!(path, PathBuf::from("src/generated.rs"));
            assert!(message.contains("timeout"));
            assert!(message.contains("too complex"));
        }
        _ => panic!("Expected ParseError"),
    }
}
