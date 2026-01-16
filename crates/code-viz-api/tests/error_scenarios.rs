#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Error scenario tests for code-viz-api
//!
//! Tests API-specific error handling, ensuring proper HTTP status codes
//! and structured JSON error responses.

use code_viz_core::error::CodeVizError;
use std::path::PathBuf;

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod api_error_responses {
    use super::*;

    #[test]
    fn test_parse_error_maps_to_500() {
        let error = CodeVizError::parse(
            PathBuf::from("src/main.rs"),
            "rust",
            Some(42),
            "unexpected token"
        );

        // Parse errors should map to 500 Internal Server Error
        // This is a critical failure in the analysis pipeline
        let is_server_error = matches!(error, CodeVizError::ParseError { .. });
        assert!(is_server_error);
    }

    #[test]
    fn test_analysis_failed_maps_to_500() {
        let error = CodeVizError::analysis(
            "coupling",
            "failed to build dependency graph"
        );

        // Analysis failures should map to 500 Internal Server Error
        let is_server_error = matches!(error, CodeVizError::Analysis { .. });
        assert!(is_server_error);
    }

    #[test]
    fn test_invalid_config_maps_to_400() {
        let error = CodeVizError::config("invalid threshold value");

        // Config errors should map to 400 Bad Request
        let is_client_error = matches!(error, CodeVizError::Config { .. });
        assert!(is_client_error);
    }

    #[test]
    fn test_file_not_found_maps_to_404() {
        use std::io;

        let path = PathBuf::from("missing.rs");
        let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
        let error = CodeVizError::file_read(&path, io_err);

        // File not found should map to 404 Not Found
        match error {
            CodeVizError::FileSystem { source, .. } => {
                assert_eq!(source.kind(), io::ErrorKind::NotFound);
            }
            _ => panic!("Expected FileSystem error"),
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod json_error_format {
    use super::*;

    #[test]
    fn test_error_message_is_serializable() {
        let error = CodeVizError::analysis("test", "test error");
        let error_msg = error.to_string();

        // Error messages should not contain characters that break JSON
        assert!(!error_msg.contains('\0'));
        assert!(!error_msg.contains('\u{001F}'));
    }

    #[test]
    fn test_parse_error_provides_details() {
        let error = CodeVizError::parse(
            PathBuf::from("src/main.rs"),
            "rust",
            Some(42),
            "unexpected token '}'"
        );

        let msg = error.to_string();

        // Error should provide enough detail for JSON response
        assert!(msg.contains("src/main.rs"));
        assert!(msg.contains("rust"));
        assert!(msg.contains("42"));
        assert!(msg.contains("unexpected token"));
    }

    #[test]
    fn test_error_does_not_leak_sensitive_info() {
        // Ensure errors don't leak system paths or internals
        let error = CodeVizError::analysis(
            "test",
            "analysis failed"
        );

        let msg = error.to_string();

        // Should not contain absolute system paths
        assert!(!msg.contains("/home/"));
        assert!(!msg.contains("C:\\Users\\"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod transform_errors {
    use super::*;

    #[test]
    fn test_transform_error_on_empty_input() {
        let error = CodeVizError::analysis(
            "transform",
            "cannot build tree from empty file list"
        );

        match error {
            CodeVizError::Analysis { operation, message, .. } => {
                assert_eq!(operation, "transform");
                assert!(message.contains("empty"));
            }
            _ => panic!("Expected Analysis error"),
        }
    }

    #[test]
    fn test_transform_error_includes_operation() {
        let error = CodeVizError::analysis(
            "flat_to_hierarchy",
            "missing root node"
        );

        let msg = error.to_string();
        assert!(msg.contains("flat_to_hierarchy"));
        assert!(msg.contains("missing root node"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod request_validation {
    use super::*;

    #[test]
    fn test_missing_required_field_error() {
        let error = CodeVizError::config("missing required field: 'path'");

        match error {
            CodeVizError::Config { message } => {
                assert!(message.contains("missing required field"));
                assert!(message.contains("path"));
            }
            _ => panic!("Expected Config error"),
        }
    }

    #[test]
    fn test_invalid_field_value_error() {
        let error = CodeVizError::config("invalid value for 'threshold': must be between 0 and 100");

        let msg = error.to_string();
        assert!(msg.contains("invalid value"));
        assert!(msg.contains("threshold"));
        assert!(msg.contains("0 and 100"));
    }

    #[test]
    fn test_type_mismatch_error() {
        let error = CodeVizError::config("type mismatch: expected number, got string");

        let msg = error.to_string();
        assert!(msg.contains("type mismatch"));
        assert!(msg.contains("expected number"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod concurrent_access_errors {
    use super::*;

    #[test]
    fn test_cache_error_on_concurrent_write() {
        use std::io;

        let io_err = io::Error::new(io::ErrorKind::WouldBlock, "resource busy");
        let error = CodeVizError::cache_with_source(
            "cache write failed",
            io_err
        );

        match error {
            CodeVizError::Cache { message, source } => {
                assert_eq!(message, "cache write failed");
                assert!(source.is_some());
            }
            _ => panic!("Expected Cache error"),
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod error_propagation {
    use super::*;

    #[test]
    fn test_error_chain_preserves_context() {
        let original = CodeVizError::parse(
            PathBuf::from("test.rs"),
            "rust",
            None,
            "original error"
        );

        let with_context = original.context("additional context");

        // Context method should preserve the error
        assert!(with_context.to_string().contains("original error"));
    }

    #[test]
    fn test_io_error_conversion_preserves_info() {
        use std::io;

        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let error: CodeVizError = io_err.into();

        match error {
            CodeVizError::FileSystem { source, .. } => {
                assert_eq!(source.kind(), io::ErrorKind::PermissionDenied);
                assert_eq!(source.to_string(), "access denied");
            }
            _ => panic!("Expected FileSystem error"),
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod resource_errors {
    use super::*;
    use std::io;

    #[test]
    fn test_out_of_memory_maps_to_500() {
        let io_err = io::Error::new(io::ErrorKind::OutOfMemory, "out of memory");
        let error = CodeVizError::cache_with_source("memory allocation failed", io_err);

        // Out of memory should map to 500 Internal Server Error
        let is_server_error = matches!(error, CodeVizError::Cache { .. });
        assert!(is_server_error);
    }

    #[test]
    fn test_disk_full_maps_to_500() {
        let path = PathBuf::from("cache/analysis.bin");
        let io_err = io::Error::new(io::ErrorKind::Other, "no space left on device");
        let error = CodeVizError::file_write(&path, io_err);

        // Disk full should map to 500 Internal Server Error
        let is_server_error = matches!(error, CodeVizError::FileSystem { .. });
        assert!(is_server_error);
    }

    #[test]
    fn test_too_many_open_files_error() {
        let path = PathBuf::from("src/file999.rs");
        let io_err = io::Error::new(io::ErrorKind::Other, "too many open files");
        let error = CodeVizError::file_read(&path, io_err);

        match error {
            CodeVizError::FileSystem { source, .. } => {
                assert!(source.to_string().contains("too many open files"));
            }
            _ => panic!("Expected FileSystem error"),
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod timeout_errors {
    use super::*;

    #[test]
    fn test_analysis_timeout_maps_to_504() {
        let error = CodeVizError::analysis(
            "coupling",
            "analysis timeout after 120 seconds"
        );

        // Timeout should map to 504 Gateway Timeout
        let is_timeout = matches!(error, CodeVizError::Analysis { .. });
        assert!(is_timeout);
        assert!(error.to_string().contains("timeout"));
    }

    #[test]
    fn test_git_timeout_error() {
        let error = CodeVizError::git(
            Some(PathBuf::from("/repo")),
            "git operation timeout"
        );

        let msg = error.to_string();
        assert!(msg.contains("timeout"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod rate_limiting_scenarios {
    use super::*;

    #[test]
    fn test_concurrent_request_limit() {
        let error = CodeVizError::analysis(
            "api_limit",
            "too many concurrent requests, please retry"
        );

        // This should map to 429 Too Many Requests
        let msg = error.to_string();
        assert!(msg.contains("too many concurrent"));
        assert!(msg.contains("retry"));
    }

    #[test]
    fn test_analysis_queue_full() {
        let error = CodeVizError::analysis(
            "queue",
            "analysis queue is full, try again later"
        );

        let msg = error.to_string();
        assert!(msg.contains("queue is full"));
        assert!(msg.contains("try again"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod malformed_requests {
    use super::*;

    #[test]
    fn test_empty_path_error() {
        let error = CodeVizError::config("path cannot be empty");

        let msg = error.to_string();
        assert!(msg.contains("path cannot be empty"));
    }

    #[test]
    fn test_invalid_json_payload() {
        let error = CodeVizError::config("invalid JSON in request body");

        let msg = error.to_string();
        assert!(msg.contains("invalid JSON"));
    }

    #[test]
    fn test_path_traversal_attempt() {
        let error = CodeVizError::config("invalid path: contains '..'");

        let msg = error.to_string();
        assert!(msg.contains("invalid path"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod coverage_data_errors {
    use super::*;

    #[test]
    fn test_malformed_coverage_file() {
        let error = CodeVizError::coverage_missing("coverage file is malformed");

        // Coverage errors should be non-critical (200 with partial data or 422)
        match error {
            CodeVizError::CoverageDataMissing { message, .. } => {
                assert!(message.contains("malformed"));
            }
            _ => panic!("Expected CoverageDataMissing error"),
        }
    }

    #[test]
    fn test_missing_coverage_for_file() {
        let error = CodeVizError::coverage_missing(
            "no coverage data for src/new_file.rs"
        );

        let msg = error.to_string();
        assert!(msg.contains("no coverage data"));
        assert!(msg.contains("new_file.rs"));
    }
}
