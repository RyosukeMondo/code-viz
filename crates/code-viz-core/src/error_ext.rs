//! Error conversion utilities and extensions
//!
//! This module provides ergonomic helpers to reduce error mapping boilerplate
//! throughout the codebase. It includes extension traits for `Result` types
//! and convenience macros for common error conversions.
//!
//! # Examples
//!
//! ## Using ResultExt for context
//!
//! ```rust
//! use code_viz_core::error_ext::ResultExt;
//! use std::fs;
//!
//! fn read_config() -> code_viz_core::error::Result<String> {
//!     fs::read_to_string("config.toml")
//!         .with_context(|| "reading configuration file".to_string())?;
//!     Ok("config".to_string())
//! }
//! ```
//!
//! ## Using macros for error conversion
//!
//! ```rust
//! use code_viz_core::{io_error, parse_error};
//! use std::path::PathBuf;
//!
//! # fn example() -> code_viz_core::error::Result<()> {
//! // Convert I/O errors with file context
//! let content = std::fs::read_to_string("file.txt")
//!     .map_err(|e| io_error!("file.txt", e))?;
//!
//! // Create parse errors with language context
//! let error = parse_error!("src/main.rs", "rust", Some(42), "unexpected token");
//! # Ok(())
//! # }
//! ```

use crate::error::{CodeVizError, Result};

/// Extension trait for `Result` types to add context to errors
///
/// This trait provides a convenient way to add contextual information
/// to errors as they propagate up the call stack, without needing
/// verbose `.map_err()` calls.
///
/// # Examples
///
/// ```rust
/// use code_viz_core::error_ext::ResultExt;
/// use code_viz_core::error::Result;
///
/// fn parse_file(path: &str) -> Result<String> {
///     read_file(path)
///         .with_context(|| format!("parsing file: {}", path))?;
///     Ok("parsed".to_string())
/// }
/// # fn read_file(path: &str) -> Result<String> { Ok("content".to_string()) }
/// ```
pub trait ResultExt<T> {
    /// Add contextual information to an error
    ///
    /// The context closure is only evaluated if the result is an error,
    /// making this efficient for the success path.
    ///
    /// # Arguments
    ///
    /// * `context` - A closure that returns contextual information as a String
    fn with_context<F>(self, context: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: Into<CodeVizError>,
{
    #[inline]
    fn with_context<F>(self, context: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let err: CodeVizError = e.into();
            err.context(context())
        })
    }
}

/// Convert an I/O error with file path context
///
/// # Examples
///
/// ```rust
/// use code_viz_core::io_error;
///
/// # fn example() -> code_viz_core::error::Result<()> {
/// let content = std::fs::read_to_string("config.toml")
///     .map_err(|e| io_error!("config.toml", e))?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! io_error {
    ($path:expr, $source:expr) => {
        $crate::error::CodeVizError::file_read($path, $source)
    };
}

/// Create a parse error with full context
///
/// # Arguments
///
/// * `path` - File path
/// * `language` - Programming language
/// * `line` - Line number (or `None`)
/// * `message` - Error message
///
/// # Examples
///
/// ```rust
/// use code_viz_core::parse_error;
///
/// # fn example() -> code_viz_core::error::Result<()> {
/// let error = parse_error!(
///     "src/main.rs",
///     "rust",
///     Some(42),
///     "unexpected token"
/// );
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! parse_error {
    ($path:expr, $language:expr, $line:expr, $message:expr) => {
        $crate::error::CodeVizError::parse(
            std::path::PathBuf::from($path),
            $language,
            $line,
            $message,
        )
    };
}

/// Create an analysis error with operation context
///
/// # Examples
///
/// ```rust
/// use code_viz_core::analysis_error;
///
/// # fn example() -> code_viz_core::error::Result<()> {
/// return Err(analysis_error!("coupling", "failed to build dependency graph"));
/// # }
/// ```
#[macro_export]
macro_rules! analysis_error {
    ($operation:expr, $message:expr) => {
        $crate::error::CodeVizError::analysis($operation, $message)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::path::PathBuf;

    // NOTE: Test code uses unwrap() for test fixtures and assertions.
    // This is acceptable because:
    // 1. Test data is controlled and known to be valid
    // 2. Test failures (panics) are the desired outcome when setup fails
    // 3. Panics in tests provide clear failure points for debugging

    #[test]
    fn test_result_ext_with_context() {
        fn failing_function() -> std::result::Result<(), io::Error> {
            Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
        }

        let result: Result<()> = failing_function().with_context(|| "test context".to_string());

        assert!(result.is_err());
        let err = result.unwrap_err();
        // The error should be converted to CodeVizError
        match err {
            CodeVizError::FileSystem { .. } => (),
            _ => panic!("Expected FileSystem error"),
        }
    }

    #[test]
    fn test_result_ext_success() {
        fn success_function() -> std::result::Result<i32, io::Error> {
            Ok(42)
        }

        let result: Result<i32> = success_function().with_context(|| "test context".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_io_error_macro() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let error = io_error!("test.txt", io_err);

        let error_msg = error.to_string();
        assert!(error_msg.contains("test.txt"));
    }

    #[test]
    fn test_parse_error_macro() {
        let error = parse_error!("src/main.rs", "rust", Some(42), "syntax error");

        match error {
            CodeVizError::ParseError {
                path,
                language,
                line,
                message,
            } => {
                assert_eq!(path, PathBuf::from("src/main.rs"));
                assert_eq!(language, "rust");
                assert_eq!(line, Some(42));
                assert_eq!(message, "syntax error");
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_analysis_error_macro() {
        let error = analysis_error!("coupling", "graph build failed");

        match error {
            CodeVizError::Analysis {
                operation, message, ..
            } => {
                assert_eq!(operation, "coupling");
                assert_eq!(message, "graph build failed");
            }
            _ => panic!("Expected Analysis error"),
        }
    }

    #[test]
    fn test_macro_hygiene() {
        // Test that macros work with module paths
        let error = crate::io_error!(
            "test.txt",
            io::Error::new(io::ErrorKind::NotFound, "not found")
        );

        assert!(matches!(error, CodeVizError::FileSystem { .. }));
    }
}
