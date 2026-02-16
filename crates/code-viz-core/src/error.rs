//! Unified error handling for code-viz
//!
//! This module provides a centralized error type hierarchy for the entire code-viz project,
//! replacing scattered unwrap() calls with proper error propagation.
//!
//! # Error Types
//!
//! The main error type is [`CodeVizError`], which wraps all specific error variants:
//!
//! - [`ParseError`] - Parsing failures (tree-sitter, malformed code)
//! - [`FileSystem`] - File I/O operations (reading, writing, permissions)
//! - [`Git`] - Git operations (repository access, commit history)
//! - [`Analysis`] - Analysis failures (metrics calculation, dependency detection)
//! - [`Coverage`] - Coverage data issues (missing data, malformed LCOV)
//! - [`Cache`] - Cache operations (serialization, storage)
//!
//! # Usage
//!
//! ## Basic error propagation with `?`
//!
//! ```rust,no_run
//! use code_viz_core::error::{CodeVizError, Result};
//! use std::path::Path;
//!
//! fn read_and_parse(path: &Path) -> Result<String> {
//!     let content = std::fs::read_to_string(path)
//!         .map_err(|e| CodeVizError::file_read(path, e))?;
//!     Ok(content)
//! }
//! ```
//!
//! ## Adding context to errors
//!
//! ```rust,no_run
//! use code_viz_core::error::{CodeVizError, Result};
//!
//! fn analyze_file(path: &str) -> Result<()> {
//!     parse_file(path)
//!         .map_err(|e| e.context(format!("analyzing file: {}", path)))?;
//!     Ok(())
//! }
//! # fn parse_file(path: &str) -> Result<()> { Ok(()) }
//! ```

use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Result type alias using [`CodeVizError`]
pub type Result<T> = std::result::Result<T, CodeVizError>;

/// Unified error type for code-viz operations
///
/// This error type consolidates all error conditions that can occur during
/// code analysis, providing rich context and proper error chaining.
#[derive(Debug, Error)]
pub enum CodeVizError {
    /// Parse error occurred while analyzing source code
    ///
    /// This variant captures parsing failures from tree-sitter or other
    /// language parsers. It includes the file path, programming language,
    /// line number (if available), and the underlying error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use code_viz_core::error::CodeVizError;
    /// use std::path::PathBuf;
    ///
    /// let error = CodeVizError::parse(
    ///     PathBuf::from("src/main.rs"),
    ///     "rust",
    ///     Some(42),
    ///     "unexpected token",
    /// );
    /// ```
    #[error("Parse error in {language} file {path:?} at line {line:?}: {message}")]
    ParseError {
        /// Path to the file that failed to parse
        path: PathBuf,
        /// Programming language being parsed
        language: String,
        /// Line number where the error occurred (if known)
        line: Option<usize>,
        /// Detailed error message
        message: String,
    },

    /// File system operation failed
    ///
    /// Covers all file I/O errors including:
    /// - File not found
    /// - Permission denied
    /// - Disk full
    /// - Invalid paths
    ///
    /// The source error from `std::io::Error` is preserved for detailed diagnostics.
    #[error("File system error at {path:?}: {message}")]
    FileSystem {
        /// Path where the error occurred
        path: PathBuf,
        /// Human-readable description
        message: String,
        /// Original I/O error
        #[source]
        source: io::Error,
    },

    /// Git operation failed
    ///
    /// Used when git operations fail, such as:
    /// - Repository not found
    /// - Invalid commit reference
    /// - Failed to read commit history
    #[error("Git error in repository {repository:?}: {message}")]
    Git {
        /// Repository path
        repository: Option<PathBuf>,
        /// Error description
        message: String,
    },

    /// Analysis operation failed
    ///
    /// Covers failures during code analysis including:
    /// - Metrics calculation errors
    /// - Dependency graph building failures
    /// - Coupling analysis errors
    /// - Dead code detection failures
    #[error("Analysis failed: {operation} - {message}")]
    Analysis {
        /// The analysis operation that failed (e.g., "coupling", "metrics")
        operation: String,
        /// Detailed error message
        message: String,
        /// Optional file path being analyzed
        path: Option<PathBuf>,
    },

    /// Coverage data error
    ///
    /// Used when coverage data is missing, malformed, or cannot be processed.
    /// This is often a non-critical error that can be handled with defaults.
    #[error("Coverage data error: {message}")]
    CoverageDataMissing {
        /// Description of the coverage issue
        message: String,
        /// Path to the coverage file (if applicable)
        path: Option<PathBuf>,
    },

    /// Cache operation failed
    ///
    /// Errors related to caching analysis results, including:
    /// - Serialization failures
    /// - Cache corruption
    /// - Storage errors
    #[error("Cache error: {message}")]
    Cache {
        /// Error description
        message: String,
        /// Underlying error if available
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Configuration error
    ///
    /// Invalid or missing configuration values
    #[error("Configuration error: {message}")]
    Config {
        /// What configuration is invalid
        message: String,
    },
}

impl CodeVizError {
    // Convenience constructors for common error patterns

    /// Create a parse error
    ///
    /// # Arguments
    ///
    /// * `path` - File path that failed to parse
    /// * `language` - Programming language
    /// * `line` - Optional line number
    /// * `message` - Error description
    pub fn parse(
        path: PathBuf,
        language: impl Into<String>,
        line: Option<usize>,
        message: impl Into<String>,
    ) -> Self {
        Self::ParseError {
            path,
            language: language.into(),
            line,
            message: message.into(),
        }
    }

    /// Create a file read error
    ///
    /// # Arguments
    ///
    /// * `path` - Path that could not be read
    /// * `source` - Original I/O error
    pub fn file_read(path: impl Into<PathBuf>, source: io::Error) -> Self {
        let path = path.into();
        Self::FileSystem {
            message: "Failed to read file".to_string(),
            path,
            source,
        }
    }

    /// Create a file write error
    ///
    /// # Arguments
    ///
    /// * `path` - Path that could not be written
    /// * `source` - Original I/O error
    pub fn file_write(path: impl Into<PathBuf>, source: io::Error) -> Self {
        let path = path.into();
        Self::FileSystem {
            message: "Failed to write file".to_string(),
            path,
            source,
        }
    }

    /// Create a git error
    ///
    /// # Arguments
    ///
    /// * `repository` - Optional repository path
    /// * `message` - Error description
    pub fn git(repository: Option<PathBuf>, message: impl Into<String>) -> Self {
        Self::Git {
            repository,
            message: message.into(),
        }
    }

    /// Create an analysis error
    ///
    /// # Arguments
    ///
    /// * `operation` - Name of the analysis operation
    /// * `message` - Error description
    pub fn analysis(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Analysis {
            operation: operation.into(),
            message: message.into(),
            path: None,
        }
    }

    /// Create an analysis error with a file path
    ///
    /// # Arguments
    ///
    /// * `operation` - Name of the analysis operation
    /// * `path` - File being analyzed
    /// * `message` - Error description
    pub fn analysis_with_path(
        operation: impl Into<String>,
        path: impl Into<PathBuf>,
        message: impl Into<String>,
    ) -> Self {
        Self::Analysis {
            operation: operation.into(),
            message: message.into(),
            path: Some(path.into()),
        }
    }

    /// Create a coverage data missing error
    ///
    /// # Arguments
    ///
    /// * `message` - Description of what's missing
    pub fn coverage_missing(message: impl Into<String>) -> Self {
        Self::CoverageDataMissing {
            message: message.into(),
            path: None,
        }
    }

    /// Create a cache error
    ///
    /// # Arguments
    ///
    /// * `message` - Error description
    pub fn cache(message: impl Into<String>) -> Self {
        Self::Cache {
            message: message.into(),
            source: None,
        }
    }

    /// Create a cache error with a source error
    ///
    /// # Arguments
    ///
    /// * `message` - Error description
    /// * `source` - Underlying error
    pub fn cache_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Cache {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Create a configuration error
    ///
    /// # Arguments
    ///
    /// * `message` - What configuration is invalid
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Add context to this error
    ///
    /// This is useful for adding higher-level context when propagating errors
    /// up the call stack.
    ///
    /// # Example
    ///
    /// ```rust
    /// use code_viz_core::error::CodeVizError;
    ///
    /// fn process_file(path: &str) -> code_viz_core::error::Result<()> {
    ///     read_file(path)
    ///         .map_err(|e| e.context(format!("processing {}", path)))?;
    ///     Ok(())
    /// }
    /// # fn read_file(path: &str) -> code_viz_core::error::Result<()> { Ok(()) }
    /// ```
    pub fn context(self, _context: impl Into<String>) -> Self {
        // For now, just return self. In the future, we could wrap the error
        // in a context variant or append to the message.
        // This provides a forward-compatible API.
        self
    }
}

// Automatic conversions from common error types

impl From<io::Error> for CodeVizError {
    fn from(err: io::Error) -> Self {
        Self::FileSystem {
            path: PathBuf::from("<unknown>"),
            message: "I/O operation failed".to_string(),
            source: err,
        }
    }
}

impl From<crate::parser::ParseError> for CodeVizError {
    fn from(err: crate::parser::ParseError) -> Self {
        Self::ParseError {
            path: PathBuf::from("<unknown>"),
            language: "unknown".to_string(),
            line: None,
            message: err.to_string(),
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_creation() {
        let error = CodeVizError::parse(
            PathBuf::from("src/main.rs"),
            "rust",
            Some(42),
            "unexpected token",
        );

        let error_msg = error.to_string();
        assert!(error_msg.contains("src/main.rs"));
        assert!(error_msg.contains("rust"));
        assert!(error_msg.contains("42"));
        assert!(error_msg.contains("unexpected token"));
    }

    #[test]
    fn test_file_read_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let error = CodeVizError::file_read(PathBuf::from("missing.txt"), io_err);

        let error_msg = error.to_string();
        assert!(error_msg.contains("missing.txt"));
        assert!(error_msg.contains("Failed to read file"));
    }

    #[test]
    fn test_analysis_error() {
        let error = CodeVizError::analysis("coupling", "failed to build graph");
        let error_msg = error.to_string();
        assert!(error_msg.contains("coupling"));
        assert!(error_msg.contains("failed to build graph"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let error: CodeVizError = io_err.into();

        match error {
            CodeVizError::FileSystem { .. } => (),
            _ => panic!("Expected FileSystem error"),
        }
    }

    #[test]
    fn test_error_implements_std_error() {
        let error = CodeVizError::analysis("test", "test error");
        // This will fail to compile if CodeVizError doesn't implement std::error::Error
        let _: &dyn std::error::Error = &error;
    }

    #[test]
    fn test_context_method() {
        let error = CodeVizError::analysis("test", "original error");
        let with_context = error.context("additional context");

        // Context should preserve the error
        assert!(with_context.to_string().contains("original error"));
    }
}
