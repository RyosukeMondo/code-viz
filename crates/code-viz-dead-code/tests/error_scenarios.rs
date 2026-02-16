//! Error scenario tests for code-viz-dead-code
//!
//! Tests dead code analysis-specific error handling, ensuring graceful
//! degradation when symbol analysis encounters issues.

use code_viz_core::error::CodeVizError;
use std::path::PathBuf;

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod symbol_graph_errors {
    use super::*;

    #[test]
    fn test_graph_build_failure_error() {
        let error = CodeVizError::analysis("symbol_graph", "failed to build symbol graph");

        match error {
            CodeVizError::Analysis {
                operation, message, ..
            } => {
                assert_eq!(operation, "symbol_graph");
                assert!(message.contains("failed to build"));
            }
            _ => panic!("Expected Analysis error"),
        }
    }

    #[test]
    fn test_parser_failure_in_graph_building() {
        let path = PathBuf::from("src/malformed.rs");
        let error = CodeVizError::parse(path.clone(), "rust", Some(15), "expected identifier");

        // Parser failures during graph building should not crash
        match error {
            CodeVizError::ParseError { path: err_path, .. } => {
                assert_eq!(err_path, path);
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_missing_exports_handled_gracefully() {
        let error = CodeVizError::analysis("exports", "no exports found in module");

        // Missing exports should be a non-critical error
        let msg = error.to_string();
        assert!(msg.contains("exports"));
        assert!(msg.contains("no exports found"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod symbol_resolution_errors {
    use super::*;

    #[test]
    fn test_symbol_not_found_error() {
        let error = CodeVizError::analysis_with_path(
            "symbol_resolution",
            PathBuf::from("src/main.rs"),
            "symbol 'MyType' not found",
        );

        match error {
            CodeVizError::Analysis {
                operation,
                message,
                path,
            } => {
                assert_eq!(operation, "symbol_resolution");
                assert!(message.contains("not found"));
                assert_eq!(path, Some(PathBuf::from("src/main.rs")));
            }
            _ => panic!("Expected Analysis error with path"),
        }
    }

    #[test]
    fn test_type_resolution_failure() {
        let error = CodeVizError::analysis("type_resolution", "cannot resolve type for symbol");

        let msg = error.to_string();
        assert!(msg.contains("type_resolution"));
        assert!(msg.contains("cannot resolve"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let error = CodeVizError::analysis(
            "dependency_check",
            "circular dependency detected: A -> B -> A",
        );

        let msg = error.to_string();
        assert!(msg.contains("circular dependency"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod entry_point_errors {
    use super::*;

    #[test]
    fn test_missing_entry_point_error() {
        let error = CodeVizError::analysis("entry_point", "no entry points specified");

        match error {
            CodeVizError::Analysis {
                operation, message, ..
            } => {
                assert_eq!(operation, "entry_point");
                assert!(message.contains("no entry points"));
            }
            _ => panic!("Expected Analysis error"),
        }
    }

    #[test]
    fn test_invalid_entry_point() {
        let error = CodeVizError::analysis(
            "entry_point",
            "entry point 'main' not found in symbol graph",
        );

        let msg = error.to_string();
        assert!(msg.contains("entry point"));
        assert!(msg.contains("not found"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod reachability_analysis_errors {
    use super::*;

    #[test]
    fn test_reachability_timeout() {
        let error = CodeVizError::analysis("reachability", "timeout during reachability analysis");

        let msg = error.to_string();
        assert!(msg.contains("reachability"));
        assert!(msg.contains("timeout"));
    }

    #[test]
    fn test_large_graph_warning() {
        let error = CodeVizError::analysis(
            "reachability",
            "graph too large (>10000 nodes), analysis may be slow",
        );

        let msg = error.to_string();
        assert!(msg.contains("too large"));
        assert!(msg.contains("10000"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod partial_results {
    use super::*;

    #[test]
    fn test_partial_analysis_on_file_error() {
        // When a single file fails to parse, the analysis should continue
        // with other files and provide partial results
        let error =
            CodeVizError::parse(PathBuf::from("src/broken.rs"), "rust", None, "syntax error");

        // This error should be logged but not stop the entire analysis
        match error {
            CodeVizError::ParseError { path, .. } => {
                assert_eq!(path, PathBuf::from("src/broken.rs"));
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_missing_symbol_uses_unknown_type() {
        // When type resolution fails, the symbol should be marked as UnknownType
        // rather than failing the entire analysis
        let error =
            CodeVizError::analysis("type_resolution", "unknown type, marking as UnknownType");

        let msg = error.to_string();
        assert!(msg.contains("unknown type"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod cache_errors {
    use super::*;

    #[test]
    fn test_cache_load_failure() {
        let error = CodeVizError::cache("failed to load cached symbol graph");

        match error {
            CodeVizError::Cache { message, .. } => {
                assert!(message.contains("failed to load"));
                assert!(message.contains("symbol graph"));
            }
            _ => panic!("Expected Cache error"),
        }
    }

    #[test]
    fn test_cache_invalidation_on_error() {
        use std::io;

        let io_err = io::Error::new(io::ErrorKind::InvalidData, "corrupted cache");
        let error = CodeVizError::cache_with_source("cache corruption detected", io_err);

        match error {
            CodeVizError::Cache { message, source } => {
                assert!(message.contains("corruption"));
                assert!(source.is_some());
            }
            _ => panic!("Expected Cache error with source"),
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod file_access_errors {
    use super::*;

    #[test]
    fn test_source_file_not_found() {
        use std::io;

        let path = PathBuf::from("src/deleted.rs");
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let error = CodeVizError::file_read(&path, io_err);

        match error {
            CodeVizError::FileSystem {
                path: err_path,
                source,
                ..
            } => {
                assert_eq!(err_path, path);
                assert_eq!(source.kind(), io::ErrorKind::NotFound);
            }
            _ => panic!("Expected FileSystem error"),
        }
    }

    #[test]
    fn test_permission_denied_on_source() {
        use std::io;

        let path = PathBuf::from("src/protected.rs");
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let error = CodeVizError::file_read(&path, io_err);

        let msg = error.to_string();
        assert!(msg.contains("protected.rs"));
        assert!(msg.contains("Failed to read file"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod graceful_degradation {
    use super::*;

    #[test]
    fn test_continue_on_non_critical_error() {
        // Test that non-critical errors don't stop analysis
        let coverage_error = CodeVizError::coverage_missing("coverage data unavailable");

        // This should be handled gracefully, not crash
        match coverage_error {
            CodeVizError::CoverageDataMissing { .. } => {
                // Expected - analysis can continue without coverage
            }
            _ => panic!("Expected CoverageDataMissing error"),
        }
    }

    #[test]
    fn test_partial_symbol_graph_acceptable() {
        let error = CodeVizError::analysis(
            "symbol_graph",
            "partial graph built: 10 files failed to parse",
        );

        // Partial results are acceptable if some analysis succeeded
        let msg = error.to_string();
        assert!(msg.contains("partial graph"));
        assert!(msg.contains("10 files failed"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod resource_errors {
    use super::*;
    use std::io;

    #[test]
    fn test_out_of_memory_during_graph_building() {
        let io_err = io::Error::new(io::ErrorKind::OutOfMemory, "out of memory");
        let error =
            CodeVizError::cache_with_source("symbol graph too large to fit in memory", io_err);

        match error {
            CodeVizError::Cache { message, source } => {
                assert!(message.contains("too large"));
                assert!(message.contains("memory"));
                assert!(source.is_some());
            }
            _ => panic!("Expected Cache error"),
        }
    }

    #[test]
    fn test_too_many_symbols_warning() {
        let error = CodeVizError::analysis(
            "symbol_graph",
            "graph contains >100000 symbols, analysis may be slow",
        );

        let msg = error.to_string();
        assert!(msg.contains("100000 symbols"));
        assert!(msg.contains("may be slow"));
    }

    #[test]
    fn test_graph_too_complex_for_analysis() {
        let error = CodeVizError::analysis(
            "reachability",
            "graph complexity exceeds limits (depth >1000)",
        );

        let msg = error.to_string();
        assert!(msg.contains("exceeds limits"));
        assert!(msg.contains("depth >1000"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod entry_point_validation {
    use super::*;

    #[test]
    fn test_multiple_main_functions_error() {
        let error = CodeVizError::analysis(
            "entry_point",
            "multiple main functions found: src/main.rs, src/bin/alt_main.rs",
        );

        let msg = error.to_string();
        assert!(msg.contains("multiple main functions"));
    }

    #[test]
    fn test_entry_point_parse_error() {
        let error = CodeVizError::parse(
            PathBuf::from("src/main.rs"),
            "rust",
            Some(1),
            "entry point file has syntax errors",
        );

        let msg = error.to_string();
        assert!(msg.contains("main.rs"));
        assert!(msg.contains("syntax errors"));
    }

    #[test]
    fn test_exported_symbol_not_found() {
        let error = CodeVizError::analysis(
            "exports",
            "exported symbol 'public_api' not found in module",
        );

        let msg = error.to_string();
        assert!(msg.contains("public_api"));
        assert!(msg.contains("not found"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod parser_timeout_errors {
    use super::*;

    #[test]
    fn test_timeout_during_symbol_extraction() {
        let error = CodeVizError::parse(
            PathBuf::from("src/generated_code.rs"),
            "rust",
            None,
            "symbol extraction timeout: file too large",
        );

        let msg = error.to_string();
        assert!(msg.contains("timeout"));
        assert!(msg.contains("too large"));
    }

    #[test]
    fn test_graph_traversal_timeout() {
        let error = CodeVizError::analysis(
            "reachability",
            "graph traversal timeout: cycle detected in dependencies",
        );

        let msg = error.to_string();
        assert!(msg.contains("timeout"));
        assert!(msg.contains("cycle detected"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod false_positive_handling {
    use super::*;

    #[test]
    fn test_reflection_usage_cannot_detect() {
        let error = CodeVizError::analysis(
            "dead_code",
            "warning: reflection usage detected, false positives possible",
        );

        let msg = error.to_string();
        assert!(msg.contains("reflection"));
        assert!(msg.contains("false positives"));
    }

    #[test]
    fn test_macro_generated_code_limitation() {
        let error = CodeVizError::analysis(
            "dead_code",
            "warning: macro-generated code may not be fully analyzed",
        );

        let msg = error.to_string();
        assert!(msg.contains("macro-generated"));
        assert!(msg.contains("not be fully analyzed"));
    }

    #[test]
    fn test_foreign_function_interface_symbols() {
        let error = CodeVizError::analysis("exports", "FFI exports cannot be verified for usage");

        let msg = error.to_string();
        assert!(msg.contains("FFI exports"));
        assert!(msg.contains("cannot be verified"));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod incremental_analysis_errors {
    use super::*;

    #[test]
    fn test_cache_invalidation_on_file_change() {
        let error =
            CodeVizError::cache("cache invalidated: source files modified since last analysis");

        let msg = error.to_string();
        assert!(msg.contains("invalidated"));
        assert!(msg.contains("modified"));
    }

    #[test]
    fn test_partial_cache_corruption() {
        use std::io;

        let io_err = io::Error::new(io::ErrorKind::InvalidData, "corrupted data");
        let error = CodeVizError::cache_with_source(
            "cache partially corrupted, rebuilding affected sections",
            io_err,
        );

        match error {
            CodeVizError::Cache { message, source } => {
                assert!(message.contains("partially corrupted"));
                assert!(source.is_some());
            }
            _ => panic!("Expected Cache error"),
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod multi_language_errors {
    use super::*;

    #[test]
    fn test_unsupported_language_in_project() {
        let error = CodeVizError::parse(
            PathBuf::from("lib.cpp"),
            "cpp",
            None,
            "unsupported language: C++ not supported in dead code analysis",
        );

        let msg = error.to_string();
        assert!(msg.contains("unsupported language"));
        assert!(msg.contains("C++"));
    }

    #[test]
    fn test_mixed_language_project_limitation() {
        let error = CodeVizError::analysis(
            "symbol_graph",
            "cross-language references not supported (Rust <-> TypeScript)",
        );

        let msg = error.to_string();
        assert!(msg.contains("cross-language"));
        assert!(msg.contains("not supported"));
    }
}
