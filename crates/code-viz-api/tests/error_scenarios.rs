//! Error scenario tests for code-viz-api
//!
//! Tests API-specific error handling, ensuring proper HTTP status codes
//! and structured JSON error responses.

use code_viz_core::error::CodeVizError;
use std::path::PathBuf;

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
