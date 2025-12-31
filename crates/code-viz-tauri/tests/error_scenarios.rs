//! Error scenario tests for code-viz-tauri
//!
//! Tests Tauri-specific error handling, ensuring errors propagate correctly
//! to the frontend via IPC and provide actionable information.

use code_viz_core::error::CodeVizError;
use std::path::PathBuf;

#[cfg(test)]
mod ipc_error_propagation {
    use super::*;

    #[test]
    fn test_error_is_serializable_to_frontend() {
        let error = CodeVizError::analysis("test", "analysis failed");
        let error_msg = error.to_string();

        // Error messages sent to frontend must be valid strings
        assert!(!error_msg.is_empty());
        assert!(error_msg.len() < 10000); // Reasonable size limit
    }

    #[test]
    fn test_parse_error_includes_file_info() {
        let error = CodeVizError::parse(
            PathBuf::from("src/components/App.tsx"),
            "tsx",
            Some(24),
            "unexpected token"
        );

        let msg = error.to_string();

        // Frontend needs file path and line number for navigation
        assert!(msg.contains("App.tsx"));
        assert!(msg.contains("24"));
    }

    #[test]
    fn test_analysis_error_is_actionable() {
        let error = CodeVizError::analysis_with_path(
            "metrics",
            PathBuf::from("src/large.rs"),
            "file too large to analyze"
        );

        let msg = error.to_string();

        // Error should explain what went wrong
        assert!(msg.contains("metrics"));
        assert!(msg.contains("too large"));
    }
}

#[cfg(test)]
mod tauri_command_errors {
    use super::*;

    #[test]
    fn test_file_system_error_for_frontend() {
        use std::io;

        let path = PathBuf::from("/inaccessible/file.rs");
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let error = CodeVizError::file_read(&path, io_err);

        let msg = error.to_string();

        // Error should be understandable by frontend users
        assert!(msg.contains("file.rs"));
        assert!(msg.contains("Failed to read file"));
    }

    #[test]
    fn test_git_error_for_frontend() {
        let error = CodeVizError::git(
            Some(PathBuf::from("/workspace/project")),
            "not a git repository"
        );

        let msg = error.to_string();

        // Frontend should know what repository had the issue
        assert!(msg.contains("project"));
        assert!(msg.contains("not a git repository"));
    }

    #[test]
    fn test_config_error_for_frontend() {
        let error = CodeVizError::config("invalid analysis configuration");

        let msg = error.to_string();

        assert!(msg.contains("Configuration error"));
        assert!(msg.contains("invalid analysis configuration"));
    }
}

#[cfg(test)]
mod transform_errors {
    use super::*;

    #[test]
    fn test_transform_error_on_invalid_data() {
        let error = CodeVizError::analysis(
            "transform",
            "cannot build hierarchy: missing root"
        );

        match error {
            CodeVizError::Analysis { operation, message, .. } => {
                assert_eq!(operation, "transform");
                assert!(message.contains("missing root"));
            }
            _ => panic!("Expected Analysis error"),
        }
    }

    #[test]
    fn test_empty_file_list_error() {
        let error = CodeVizError::analysis(
            "flat_to_hierarchy",
            "empty file list provided"
        );

        let msg = error.to_string();
        assert!(msg.contains("flat_to_hierarchy"));
        assert!(msg.contains("empty"));
    }
}

#[cfg(test)]
mod coverage_errors {
    use super::*;

    #[test]
    fn test_missing_coverage_data_error() {
        let error = CodeVizError::coverage_missing("no coverage report found");

        match error {
            CodeVizError::CoverageDataMissing { message, path } => {
                assert_eq!(message, "no coverage report found");
                assert_eq!(path, None);
            }
            _ => panic!("Expected CoverageDataMissing error"),
        }
    }

    #[test]
    fn test_coverage_error_with_path() {
        let path = PathBuf::from("coverage/lcov.info");
        let mut error = CodeVizError::coverage_missing("invalid format");

        // Manually set path for testing
        if let CodeVizError::CoverageDataMissing { path: ref mut p, .. } = error {
            *p = Some(path.clone());
        }

        match error {
            CodeVizError::CoverageDataMissing { path: Some(p), .. } => {
                assert_eq!(p, path);
            }
            _ => panic!("Expected coverage error with path"),
        }
    }
}

#[cfg(test)]
mod cache_errors {
    use super::*;

    #[test]
    fn test_cache_serialization_error() {
        let error = CodeVizError::cache("failed to serialize analysis results");

        match error {
            CodeVizError::Cache { message, source } => {
                assert!(message.contains("serialize"));
                assert!(source.is_none());
            }
            _ => panic!("Expected Cache error"),
        }
    }

    #[test]
    fn test_cache_write_error_with_source() {
        use std::io;

        let io_err = io::Error::new(io::ErrorKind::OutOfMemory, "out of memory");
        let error = CodeVizError::cache_with_source("cache allocation failed", io_err);

        match error {
            CodeVizError::Cache { message, source } => {
                assert_eq!(message, "cache allocation failed");
                assert!(source.is_some());
            }
            _ => panic!("Expected Cache error"),
        }
    }
}

#[cfg(test)]
mod error_context {
    use super::*;

    #[test]
    fn test_context_method_works() {
        let error = CodeVizError::analysis("test", "test error");
        let with_context = error.context("additional context");

        // Should still be the same error type
        assert!(matches!(with_context, CodeVizError::Analysis { .. }));
    }

    #[test]
    fn test_error_chain_for_debugging() {
        let error = CodeVizError::parse(
            PathBuf::from("debug.rs"),
            "rust",
            Some(10),
            "syntax error"
        );

        let contextualized = error.context("during hot reload");

        // Error should still contain original information
        let msg = contextualized.to_string();
        assert!(msg.contains("debug.rs"));
        assert!(msg.contains("syntax error"));
    }
}

#[cfg(test)]
mod error_traits {
    use super::*;

    #[test]
    fn test_error_is_send_sync() {
        // Tauri requires errors to be Send + Sync for IPC
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CodeVizError>();
    }

    #[test]
    fn test_error_display_trait() {
        let error = CodeVizError::analysis("test", "display test");
        let display = format!("{}", error);

        assert!(!display.is_empty());
        assert!(display.contains("test"));
    }

    #[test]
    fn test_error_debug_trait() {
        let error = CodeVizError::config("debug test");
        let debug = format!("{:?}", error);

        assert!(debug.contains("Config"));
        assert!(debug.contains("debug test"));
    }
}
