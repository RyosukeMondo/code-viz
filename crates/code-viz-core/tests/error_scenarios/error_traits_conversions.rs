//! Error trait and conversion tests

use code_viz_core::error::CodeVizError;
use std::io;
use std::path::PathBuf;

#[test]
fn test_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CodeVizError>();
}

#[test]
fn test_error_implements_std_error() {
    let error = CodeVizError::analysis("test", "test error");
    let _: &dyn std::error::Error = &error;
}

#[test]
fn test_error_implements_debug() {
    let error = CodeVizError::parse(
        PathBuf::from("test.rs"),
        "rust",
        Some(10),
        "test error"
    );

    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("ParseError"));
}

#[test]
fn test_error_implements_display() {
    let error = CodeVizError::analysis("test", "test message");
    let display_str = format!("{}", error);

    assert!(display_str.contains("test"));
    assert!(display_str.contains("test message"));
}

#[test]
fn test_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
    let error: CodeVizError = io_err.into();

    match error {
        CodeVizError::FileSystem { .. } => {
            // Correct variant
        }
        _ => panic!("Expected FileSystem error from io::Error"),
    }
}

#[test]
fn test_io_error_preserves_kind() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let error: CodeVizError = io_err.into();

    match error {
        CodeVizError::FileSystem { source, .. } => {
            assert_eq!(source.kind(), io::ErrorKind::PermissionDenied);
        }
        _ => panic!("Expected FileSystem error"),
    }
}

#[test]
fn test_context_method_preserves_error() {
    let original = CodeVizError::analysis("test", "original message");
    let with_context = original.context("additional context");

    // Context should preserve the original error
    let msg = with_context.to_string();
    assert!(msg.contains("original message"));
}

#[test]
fn test_context_can_be_chained() {
    let error = CodeVizError::parse(
        PathBuf::from("test.rs"),
        "rust",
        None,
        "parse failed"
    );

    let contextualized = error
        .context("step 1")
        .context("step 2");

    // Should still be valid error
    assert!(contextualized.to_string().contains("parse failed"));
}
