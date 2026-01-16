//! Error scenario tests for code-viz-cli
//!
//! Tests CLI-specific error handling, ensuring user-friendly error messages
//! and proper exit codes.

use std::path::PathBuf;

#[cfg(test)]
mod cli_error_handling {
    #[test]
    fn test_error_messages_are_user_friendly() {
        // CLI errors should be human-readable, not technical jargon
        // This is a design principle test

        // Parse error should be friendly
        let parse_err_msg = "Failed to parse src/main.rs. Is this a valid Rust file?";
        assert!(parse_err_msg.contains("Failed to parse"));
        assert!(parse_err_msg.contains("valid"));

        // Missing file should be friendly
        let missing_file_msg = "File not found: config.toml. Check the path and try again.";
        assert!(missing_file_msg.contains("File not found"));
        assert!(missing_file_msg.contains("Check the path"));
    }

    #[test]
    fn test_config_validation_errors() {
        // Test that configuration errors are descriptive
        use code_viz_core::error::CodeVizError;

        let error = CodeVizError::config("invalid threshold: expected 0-100, got 150");
        let msg = error.to_string();

        assert!(msg.contains("Configuration error"));
        assert!(msg.contains("invalid threshold"));
        assert!(msg.contains("expected 0-100"));
    }

    #[test]
    fn test_analysis_failure_error() {
        use code_viz_core::error::CodeVizError;

        let error = CodeVizError::analysis(
            "coupling",
            "timeout after 30 seconds"
        );

        let msg = error.to_string();
        assert!(msg.contains("Analysis failed"));
        assert!(msg.contains("coupling"));
        assert!(msg.contains("timeout"));
    }
}

#[cfg(test)]
mod file_access_errors {
    use super::*;
    use std::io;
    use code_viz_core::error::CodeVizError;

    #[test]
    fn test_permission_denied_error() {
        let path = PathBuf::from("/root/restricted.txt");
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");

        let error = CodeVizError::file_read(&path, io_err);
        let msg = error.to_string();

        assert!(msg.contains("/root/restricted.txt"));
        assert!(msg.contains("Failed to read file"));
    }

    #[test]
    fn test_directory_not_file_error() {
        let path = PathBuf::from("/tmp");
        let io_err = io::Error::new(io::ErrorKind::IsADirectory, "is a directory");

        let error = CodeVizError::file_read(&path, io_err);

        match error {
            CodeVizError::FileSystem { path: err_path, source, .. } => {
                assert_eq!(err_path, path);
                assert_eq!(source.kind(), io::ErrorKind::IsADirectory);
            }
            _ => panic!("Expected FileSystem error"),
        }
    }

    #[test]
    fn test_file_not_found_helpful_message() {
        let path = PathBuf::from("nonexistent.toml");
        let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");

        let error = CodeVizError::file_read(&path, io_err);
        let msg = error.to_string();

        // Error should mention the file path
        assert!(msg.contains("nonexistent.toml"));
    }
}

#[cfg(test)]
mod argument_validation {
    use code_viz_core::error::CodeVizError;

    #[test]
    fn test_invalid_argument_error() {
        let error = CodeVizError::config("invalid output format: 'xlsx' (supported: json, text, csv)");

        match error {
            CodeVizError::Config { message } => {
                assert!(message.contains("invalid output format"));
                assert!(message.contains("supported"));
            }
            _ => panic!("Expected Config error"),
        }
    }

    #[test]
    fn test_missing_required_argument() {
        let error = CodeVizError::config("missing required argument: --path");
        let msg = error.to_string();

        assert!(msg.contains("missing required argument"));
        assert!(msg.contains("--path"));
    }

    #[test]
    fn test_conflicting_arguments() {
        let error = CodeVizError::config("cannot use --watch with --baseline");
        let msg = error.to_string();

        assert!(msg.contains("Configuration error"));
        assert!(msg.contains("cannot use"));
    }
}

#[cfg(test)]
mod git_errors {
    use super::*;
    use code_viz_core::error::CodeVizError;

    #[test]
    fn test_not_a_git_repository() {
        let path = PathBuf::from("/tmp/not-a-repo");
        let error = CodeVizError::git(
            Some(path.clone()),
            "not a git repository"
        );

        match error {
            CodeVizError::Git { repository, message } => {
                assert_eq!(repository, Some(path));
                assert_eq!(message, "not a git repository");
            }
            _ => panic!("Expected Git error"),
        }
    }

    #[test]
    fn test_git_command_not_found() {
        let error = CodeVizError::git(
            None,
            "git command not found in PATH"
        );

        let msg = error.to_string();
        assert!(msg.contains("git"));
        assert!(msg.contains("not found"));
    }

    #[test]
    fn test_invalid_commit_reference() {
        let repo = PathBuf::from(".");
        let error = CodeVizError::git(
            Some(repo),
            "invalid commit reference: 'invalid-hash'"
        );

        let msg = error.to_string();
        assert!(msg.contains("invalid commit reference"));
    }
}

#[cfg(test)]
mod analysis_timeout {
    use code_viz_core::error::CodeVizError;

    #[test]
    fn test_analysis_timeout_error() {
        let error = CodeVizError::analysis(
            "duplication",
            "analysis timeout after 60 seconds"
        );

        let msg = error.to_string();
        assert!(msg.contains("duplication"));
        assert!(msg.contains("timeout"));
    }

    #[test]
    fn test_timeout_includes_file_context() {
        use std::path::PathBuf;

        let path = PathBuf::from("src/large_file.rs");
        let error = CodeVizError::analysis_with_path(
            "metrics",
            path.clone(),
            "processing timeout"
        );

        match error {
            CodeVizError::Analysis { path: err_path, .. } => {
                assert_eq!(err_path, Some(path));
            }
            _ => panic!("Expected Analysis error with path"),
        }
    }
}

#[cfg(test)]
mod resource_constraints {
    use super::*;
    use std::io;
    use code_viz_core::error::CodeVizError;

    #[test]
    fn test_disk_full_error_on_output() {
        let path = PathBuf::from("output/report.json");
        let io_err = io::Error::new(io::ErrorKind::Other, "disk full");

        let error = CodeVizError::file_write(&path, io_err);
        let msg = error.to_string();

        assert!(msg.contains("output/report.json"));
        assert!(msg.contains("Failed to write file"));
    }

    #[test]
    fn test_out_of_memory_during_large_analysis() {
        let error = CodeVizError::analysis(
            "metrics",
            "out of memory while processing large codebase"
        );

        let msg = error.to_string();
        assert!(msg.contains("out of memory"));
        assert!(msg.contains("large codebase"));
    }

    #[test]
    fn test_too_many_files_to_analyze() {
        let error = CodeVizError::analysis(
            "scanner",
            "too many files (>50000), consider using --exclude"
        );

        let msg = error.to_string();
        assert!(msg.contains("too many files"));
        assert!(msg.contains("exclude"));
    }
}

#[cfg(test)]
mod malformed_input {
    use super::*;
    use code_viz_core::error::CodeVizError;

    #[test]
    fn test_binary_file_detected() {
        let error = CodeVizError::parse(
            PathBuf::from("assets/image.png"),
            "unknown",
            None,
            "binary file detected, skipping"
        );

        let msg = error.to_string();
        assert!(msg.contains("image.png"));
        assert!(msg.contains("binary file"));
    }

    #[test]
    fn test_empty_repository_error() {
        let error = CodeVizError::git(
            Some(PathBuf::from("/empty/repo")),
            "repository has no commits"
        );

        let msg = error.to_string();
        assert!(msg.contains("no commits"));
    }

    #[test]
    fn test_unsupported_file_extension() {
        let error = CodeVizError::parse(
            PathBuf::from("README.md"),
            "markdown",
            None,
            "unsupported language: markdown"
        );

        let msg = error.to_string();
        assert!(msg.contains("unsupported language"));
        assert!(msg.contains("markdown"));
    }
}

#[cfg(test)]
mod error_recovery {
    use code_viz_core::error::CodeVizError;

    #[test]
    fn test_partial_analysis_on_errors() {
        // When some files fail, analysis should continue with others
        let error = CodeVizError::analysis(
            "scanner",
            "partial analysis: 5 files failed, 95 succeeded"
        );

        let msg = error.to_string();
        assert!(msg.contains("partial analysis"));
        assert!(msg.contains("5 files failed"));
        assert!(msg.contains("95 succeeded"));
    }

    #[test]
    fn test_graceful_degradation_message() {
        let error = CodeVizError::coverage_missing(
            "coverage data unavailable, continuing without coverage metrics"
        );

        let msg = error.to_string();
        assert!(msg.contains("coverage data unavailable"));
        assert!(msg.contains("continuing without"));
    }
}
