//! Analysis error tests

use code_viz_core::error::CodeVizError;
use std::path::PathBuf;

#[test]
fn test_analysis_error_contains_operation() {
    let error = CodeVizError::analysis("coupling", "failed to build dependency graph");

    match error {
        CodeVizError::Analysis {
            operation, message, ..
        } => {
            assert_eq!(operation, "coupling");
            assert_eq!(message, "failed to build dependency graph");
        }
        _ => panic!("Expected Analysis error"),
    }
}

#[test]
fn test_analysis_error_with_path() {
    let path = PathBuf::from("src/main.rs");
    let error = CodeVizError::analysis_with_path("metrics", path.clone(), "complexity too high");

    match error {
        CodeVizError::Analysis {
            operation,
            message,
            path: error_path,
        } => {
            assert_eq!(operation, "metrics");
            assert_eq!(message, "complexity too high");
            assert_eq!(error_path, Some(path));
        }
        _ => panic!("Expected Analysis error"),
    }
}

#[test]
fn test_analysis_error_message_formatting() {
    let error = CodeVizError::analysis("duplication", "timeout exceeded");
    let error_msg = error.to_string();

    assert!(error_msg.contains("Analysis failed"));
    assert!(error_msg.contains("duplication"));
    assert!(error_msg.contains("timeout exceeded"));
}

#[test]
fn test_resource_exhaustion_during_analysis() {
    let error = CodeVizError::analysis("metrics", "resource exhaustion: too many files to process");

    match error {
        CodeVizError::Analysis {
            operation, message, ..
        } => {
            assert_eq!(operation, "metrics");
            assert!(message.contains("resource exhaustion"));
            assert!(message.contains("too many files"));
        }
        _ => panic!("Expected Analysis error"),
    }
}

#[test]
fn test_analysis_timeout_with_large_codebase() {
    let error = CodeVizError::analysis(
        "coupling",
        "analysis timeout after 300 seconds (codebase too large)",
    );

    let msg = error.to_string();
    assert!(msg.contains("coupling"));
    assert!(msg.contains("timeout"));
    assert!(msg.contains("300 seconds"));
}
