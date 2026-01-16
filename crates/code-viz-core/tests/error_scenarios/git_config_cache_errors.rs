//! Git, config, and cache error tests

use code_viz_core::error::CodeVizError;
use std::io;
use std::path::PathBuf;

// Git errors
#[test]
fn test_git_error_without_repository() {
    let error = CodeVizError::git(None, "not a git repository");

    match error {
        CodeVizError::Git { repository, message } => {
            assert_eq!(repository, None);
            assert_eq!(message, "not a git repository");
        }
        _ => panic!("Expected Git error"),
    }
}

#[test]
fn test_git_error_with_repository() {
    let repo_path = PathBuf::from("/path/to/repo");
    let error = CodeVizError::git(Some(repo_path.clone()), "invalid commit hash");

    match error {
        CodeVizError::Git { repository, message } => {
            assert_eq!(repository, Some(repo_path));
            assert_eq!(message, "invalid commit hash");
        }
        _ => panic!("Expected Git error"),
    }
}

#[test]
fn test_git_operation_timeout() {
    let error = CodeVizError::git(
        Some(PathBuf::from("/repo")),
        "git log operation timeout after 60 seconds"
    );

    let msg = error.to_string();
    assert!(msg.contains("timeout"));
    assert!(msg.contains("60 seconds"));
}

// Config errors
#[test]
fn test_config_error_creation() {
    let error = CodeVizError::config("invalid threshold: expected 0-100");

    match error {
        CodeVizError::Config { message } => {
            assert_eq!(message, "invalid threshold: expected 0-100");
        }
        _ => panic!("Expected Config error"),
    }
}

#[test]
fn test_config_error_message() {
    let error = CodeVizError::config("missing required field 'output_format'");
    let error_msg = error.to_string();

    assert!(error_msg.contains("Configuration error"));
    assert!(error_msg.contains("missing required field"));
}

#[test]
fn test_conflicting_options_error() {
    let error = CodeVizError::config(
        "conflicting options: cannot use --exclude-tests with --only-tests"
    );

    match error {
        CodeVizError::Config { message } => {
            assert!(message.contains("conflicting options"));
            assert!(message.contains("exclude-tests"));
            assert!(message.contains("only-tests"));
        }
        _ => panic!("Expected Config error"),
    }
}

#[test]
fn test_invalid_threshold_range() {
    let error = CodeVizError::config("threshold must be between 0 and 100, got -5");

    let msg = error.to_string();
    assert!(msg.contains("threshold"));
    assert!(msg.contains("0 and 100"));
    assert!(msg.contains("-5"));
}

#[test]
fn test_invalid_output_format() {
    let error = CodeVizError::config(
        "unsupported output format 'pdf', supported formats: json, csv, text"
    );

    let msg = error.to_string();
    assert!(msg.contains("unsupported output format"));
    assert!(msg.contains("pdf"));
    assert!(msg.contains("supported formats"));
}

// Cache errors
#[test]
fn test_cache_error_without_source() {
    let error = CodeVizError::cache("failed to serialize cache");

    match error {
        CodeVizError::Cache { message, source } => {
            assert_eq!(message, "failed to serialize cache");
            assert!(source.is_none());
        }
        _ => panic!("Expected Cache error"),
    }
}

#[test]
fn test_cache_error_with_source() {
    let io_err = io::Error::new(io::ErrorKind::Other, "disk full");
    let error = CodeVizError::cache_with_source("cache write failed", io_err);

    match error {
        CodeVizError::Cache { message, source } => {
            assert_eq!(message, "cache write failed");
            assert!(source.is_some());
        }
        _ => panic!("Expected Cache error"),
    }
}

#[test]
fn test_out_of_memory_error() {
    let io_err = io::Error::new(io::ErrorKind::OutOfMemory, "out of memory");
    let error = CodeVizError::cache_with_source(
        "failed to allocate memory for analysis cache",
        io_err
    );

    match error {
        CodeVizError::Cache { message, source } => {
            assert!(message.contains("memory"));
            assert!(source.is_some());
            let source_err = source.unwrap();
            assert!(source_err.to_string().contains("out of memory"));
        }
        _ => panic!("Expected Cache error"),
    }
}
