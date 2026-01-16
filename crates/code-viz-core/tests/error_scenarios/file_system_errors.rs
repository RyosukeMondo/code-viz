//! File system error tests

use code_viz_core::error::{CodeVizError, Result};
use std::fs;
use std::io;
use std::path::PathBuf;

#[test]
fn test_file_not_found_returns_error() {
    let nonexistent = PathBuf::from("/tmp/definitely-does-not-exist-12345.txt");

    let result: Result<String> = fs::read_to_string(&nonexistent)
        .map_err(|e| CodeVizError::file_read(&nonexistent, e));

    assert!(result.is_err());
    let err = result.unwrap_err();

    match err {
        CodeVizError::FileSystem { path, message, .. } => {
            assert_eq!(path, nonexistent);
            assert!(message.contains("Failed to read file"));
        }
        _ => panic!("Expected FileSystem error, got: {:?}", err),
    }
}

#[test]
fn test_invalid_path_error_message() {
    let invalid_path = PathBuf::from("\0");
    let io_err = io::Error::new(io::ErrorKind::InvalidInput, "invalid path");

    let error = CodeVizError::file_read(&invalid_path, io_err);
    let error_msg = error.to_string();

    assert!(error_msg.contains("Failed to read file"));
}

#[test]
fn test_file_write_error() {
    let path = PathBuf::from("/root/cannot-write-here.txt");
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");

    let error = CodeVizError::file_write(&path, io_err);

    match error {
        CodeVizError::FileSystem { message, .. } => {
            assert!(message.contains("Failed to write file"));
        }
        _ => panic!("Expected FileSystem error"),
    }
}

#[test]
fn test_disk_full_error() {
    let io_err = io::Error::new(io::ErrorKind::Other, "no space left on device");
    let path = PathBuf::from("output/analysis.json");
    let error = CodeVizError::file_write(&path, io_err);

    match error {
        CodeVizError::FileSystem { message, source, .. } => {
            assert!(message.contains("Failed to write file"));
            assert!(source.to_string().contains("no space left"));
        }
        _ => panic!("Expected FileSystem error"),
    }
}

#[test]
fn test_too_many_open_files_error() {
    let io_err = io::Error::new(
        io::ErrorKind::Other,
        "too many open files"
    );
    let path = PathBuf::from("src/large_project/file_999.rs");
    let error = CodeVizError::file_read(&path, io_err);

    match error {
        CodeVizError::FileSystem { source, .. } => {
            assert!(source.to_string().contains("too many open files"));
        }
        _ => panic!("Expected FileSystem error"),
    }
}
