use code_viz_dead_code::models::SymbolKind;
use code_viz_dead_code::{DeadCodeResult, DeadCodeSummary, DeadSymbol, FileDeadCode};
use std::path::PathBuf;

// Import the formatters (these are in src/output/dead_code.rs)
// Since CLI is a binary crate, we need to access the functions directly
// We'll compile them into this test file for validation

use serde_json;

/// Format dead code result as pretty-printed JSON
fn format_json(result: &DeadCodeResult) -> Result<String, String> {
    serde_json::to_string_pretty(result).map_err(|_| "JSON serialization failed".to_string())
}

fn create_sample_result() -> DeadCodeResult {
    DeadCodeResult {
        summary: DeadCodeSummary {
            total_files: 10,
            files_with_dead_code: 2,
            dead_functions: 5,
            dead_classes: 1,
            total_dead_loc: 150,
            dead_code_ratio: 0.15,
        },
        files: vec![
            FileDeadCode {
                path: PathBuf::from("src/utils.ts"),
                dead_code: vec![
                    DeadSymbol {
                        symbol: "unusedFunction".to_string(),
                        kind: SymbolKind::Function,
                        line_start: 10,
                        line_end: 20,
                        loc: 10,
                        confidence: 95,
                        reason: "Not imported or called anywhere".to_string(),
                        last_modified: None,
                    },
                    DeadSymbol {
                        symbol: "oldHelper".to_string(),
                        kind: SymbolKind::ArrowFunction,
                        line_start: 25,
                        line_end: 30,
                        loc: 5,
                        confidence: 85,
                        reason: "Exported but never used".to_string(),
                        last_modified: None,
                    },
                ],
            },
            FileDeadCode {
                path: PathBuf::from("src/legacy.ts"),
                dead_code: vec![DeadSymbol {
                    symbol: "LegacyClass".to_string(),
                    kind: SymbolKind::Class,
                    line_start: 1,
                    line_end: 100,
                    loc: 100,
                    confidence: 65,
                    reason: "Exported and recently modified".to_string(),
                    last_modified: None,
                }],
            },
        ],
    }
}

#[test]
fn test_format_json() {
    let result = create_sample_result();
    let json = format_json(&result).unwrap();

    // Verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["summary"]["totalFiles"], 10);
    assert_eq!(parsed["summary"]["deadFunctions"], 5);
    assert_eq!(parsed["files"].as_array().unwrap().len(), 2);
}

#[test]
fn test_json_roundtrip() {
    let result = create_sample_result();
    let json = format_json(&result).unwrap();

    // Verify we can deserialize it back
    let deserialized: DeadCodeResult = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.summary.total_files, result.summary.total_files);
    assert_eq!(
        deserialized.summary.dead_functions,
        result.summary.dead_functions
    );
    assert_eq!(deserialized.files.len(), result.files.len());
}

#[test]
fn test_json_schema_compliance() {
    let result = create_sample_result();
    let json = format_json(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Verify required fields exist
    assert!(parsed["summary"].is_object());
    assert!(parsed["summary"]["totalFiles"].is_number());
    assert!(parsed["summary"]["filesWithDeadCode"].is_number());
    assert!(parsed["summary"]["deadFunctions"].is_number());
    assert!(parsed["summary"]["deadClasses"].is_number());
    assert!(parsed["summary"]["totalDeadLoc"].is_number());
    assert!(parsed["summary"]["deadCodeRatio"].is_number());

    assert!(parsed["files"].is_array());
    let files = parsed["files"].as_array().unwrap();
    assert!(!files.is_empty());

    // Check first file structure
    let first_file = &files[0];
    assert!(first_file["path"].is_string());
    assert!(first_file["deadCode"].is_array());

    let dead_symbols = first_file["deadCode"].as_array().unwrap();
    if !dead_symbols.is_empty() {
        let symbol = &dead_symbols[0];
        assert!(symbol["symbol"].is_string());
        assert!(symbol["kind"].is_string());
        assert!(symbol["lineStart"].is_number());
        assert!(symbol["lineEnd"].is_number());
        assert!(symbol["loc"].is_number());
        assert!(symbol["confidence"].is_number());
        assert!(symbol["reason"].is_string());
    }
}
