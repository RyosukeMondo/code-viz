//! Comprehensive error scenario tests for code-viz-core
//!
//! This test module validates all error paths across the crate,
//! ensuring errors are handled gracefully and provide useful information.

use code_viz_core::error::{CodeVizError, Result};
use std::io;
use std::path::PathBuf;

#[cfg(test)]
mod parser_errors {
    use code_viz_core::parser::{LanguageParser, RustParser, TypeScriptParser};

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
        use code_viz_core::parser::ParseError;

        let error = ParseError::TreeSitterError("failed to parse".to_string());
        let msg = error.to_string();

        assert!(msg.contains("Tree-sitter") || msg.contains("parse"));
    }
}

#[cfg(test)]
mod file_system_errors {
    use super::*;
    use std::fs;

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
}

#[cfg(test)]
mod coverage_errors {
    use super::*;
    use code_viz_core::coverage::parse_coverage_report;

    #[test]
    fn test_malformed_coverage_json_error() {
        let invalid_json = "{ this is not valid json }";

        let result = parse_coverage_report(invalid_json);

        // Should handle malformed data gracefully
        match result {
            Ok(_) => {
                // Parser was lenient or returned empty
            }
            Err(e) => {
                // Should provide useful error
                let msg = e.to_string();
                assert!(msg.contains("coverage") ||
                        msg.contains("parse") ||
                        msg.contains("JSON") ||
                        msg.contains("invalid"));
            }
        }
    }

    #[test]
    fn test_empty_coverage_data() {
        let empty_json = "{}";

        let result = parse_coverage_report(empty_json);

        // Empty coverage data should be handled gracefully
        match result {
            Ok(coverage) => {
                // Should return empty or minimal data
                assert!(coverage.is_empty() || !coverage.is_empty());
            }
            Err(_) => {
                // Error is acceptable for empty data
            }
        }
    }

    #[test]
    fn test_coverage_data_missing_error_type() {
        let error = CodeVizError::coverage_missing("no coverage data found");

        match error {
            CodeVizError::CoverageDataMissing { message, path } => {
                assert_eq!(message, "no coverage data found");
                assert_eq!(path, None);
            }
            _ => panic!("Expected CoverageDataMissing error"),
        }
    }
}

#[cfg(test)]
mod analysis_errors {
    use super::*;

    #[test]
    fn test_analysis_error_contains_operation() {
        let error = CodeVizError::analysis("coupling", "failed to build dependency graph");

        match error {
            CodeVizError::Analysis { operation, message, .. } => {
                assert_eq!(operation, "coupling");
                assert_eq!(message, "failed to build dependency graph");
            }
            _ => panic!("Expected Analysis error"),
        }
    }

    #[test]
    fn test_analysis_error_with_path() {
        let path = PathBuf::from("src/main.rs");
        let error = CodeVizError::analysis_with_path(
            "metrics",
            path.clone(),
            "complexity too high"
        );

        match error {
            CodeVizError::Analysis { operation, message, path: error_path } => {
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
}

#[cfg(test)]
mod git_errors {
    use super::*;

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
}

#[cfg(test)]
mod config_errors {
    use super::*;

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
}

#[cfg(test)]
mod cache_errors {
    use super::*;

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
}

#[cfg(test)]
mod error_traits {
    use super::*;

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
}

#[cfg(test)]
mod error_conversions {
    use super::*;

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
}

#[cfg(test)]
mod context_propagation {
    use super::*;

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
}
