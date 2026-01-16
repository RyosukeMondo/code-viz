//! Metrics Module - Code Quality and Complexity Analysis
//!
//! This module provides comprehensive code metrics including:
//! - Lines of code (LOC) calculation
//! - Cognitive complexity analysis
//! - Code churn tracking
//!
//! ## Module Organization
//! - `loc_calculator`: LOC counting excluding comments/blanks
//! - `complexity_analyzer`: Cognitive complexity and function detection
//! - `churn_calculator`: Git-based code churn metrics

mod churn_calculator;
mod complexity_analyzer;
mod loc_calculator;

// Re-export public APIs
pub use churn_calculator::{calculate_churn_summary, ChurnError};
pub use complexity_analyzer::analyze_complexity;
pub use loc_calculator::calculate_loc;

use crate::models::FileMetrics;
use crate::parser::LanguageParser;
use std::path::Path;
use std::time::SystemTime;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetricsError {
    #[error("Parse failed: {0}")]
    ParseFailed(#[from] crate::parser::ParseError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Git error: {0}")]
    GitError(String),

    #[error("Churn calculation failed: {0}")]
    ChurnFailed(#[from] ChurnError),
}

/// Calculate comprehensive file metrics
///
/// # Arguments
/// * `path` - File path
/// * `source` - Source code content
/// * `parser` - Language parser implementing LanguageParser trait
/// * `last_modified` - Optional last modified timestamp (defaults to now)
///
/// # Returns
/// Complete file metrics including LOC, complexity, and metadata
pub fn calculate_metrics(
    path: &Path,
    source: &str,
    parser: &dyn LanguageParser,
    last_modified: Option<SystemTime>,
) -> Result<FileMetrics, MetricsError> {
    let tree = parser.parse(source).map_err(MetricsError::ParseFailed)?;
    let function_count = parser.count_functions(&tree);
    let comment_ranges = parser.find_comment_ranges(&tree);

    let loc = calculate_loc(source, &comment_ranges);
    let size_bytes = source.len() as u64;

    let total_non_blank_lines = source.lines().filter(|l| !l.trim().is_empty()).count();
    let comment_lines = total_non_blank_lines.saturating_sub(loc);

    let ai_bloat_index = if loc > 0 {
        Some((comment_lines as f64 / loc as f64) * 100.0)
    } else if comment_lines > 0 {
        Some(999.0) // Effectively infinite bloat for file with only comments
    } else {
        Some(0.0) // No code and no comments
    };

    // Calculate cognitive complexity
    let cognitive_complexity = analyze_complexity(source, parser);

    // Use provided last_modified or fallback to now()
    let last_modified = last_modified.unwrap_or_else(SystemTime::now);

    Ok(FileMetrics {
        path: path.to_path_buf(),
        language: parser.language_key().to_string(),
        loc,
        size_bytes,
        function_count,
        last_modified,
        dead_function_count: None,
        dead_code_loc: None,
        dead_code_ratio: None,
        code_churn: None,
        coupling: None,
        ai_bloat_index,
        cognitive_complexity,
        test_coverage: None,
    })
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::get_parser;
    use tempfile::TempDir;

    // NOTE: Test code uses unwrap() for test fixtures and assertions.
    // This is acceptable because:
    // 1. Test data is controlled and known to be valid
    // 2. Test failures (panics) are the desired outcome when setup fails
    // 3. Panics in tests provide clear failure points for debugging

    #[test]
    fn test_rust_loc_calculation() {
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            function main() {
                // This is a comment
                console.log("Hello"); // Inline comment
                /* Block comment
                   spanning lines */
                let x = 1;
            }
        "#;
        // LOC:
        // 1. empty
        // 2. function... (LOC)
        // 3. // ... (Skip)
        // 4. console... (LOC)
        // 5. /* ... (Skip)
        // 6. ... */ (Skip)
        // 7. let x = 1; (LOC)
        // 8. } (LOC)
        // 9. empty
        // Total: 4

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");

        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.loc, 4);
    }

    #[test]
    fn test_comments_excluded() {
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            // Line 1
            // Line 2

            /* Block
               Line 4 */
        "#;
        // All comments or blank. LOC = 0.
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");

        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.loc, 0);
    }

    #[test]
    fn test_mixed_line_comments() {
        let parser = get_parser("typescript").unwrap();
        let source = "let x = 1; // Comment";
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.loc, 1);
    }

    #[test]
    fn test_multiline_comments_excluded() {
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            /*
             * Multi-line
             * Comment
             */
        "#;
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.loc, 0);
    }

    #[test]
    fn test_comment_then_code() {
        // Rare case: /* c */ code
        let parser = get_parser("typescript").unwrap();
        let source = "/* c */ let x = 1;";
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.loc, 1);
    }

    #[test]
    fn test_code_inside_comment_block() {
        // Checking boundaries
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            let a = 1;
            /* start
            mid
            end */ let b = 2;
        "#;
        // 1. let a (LOC)
        // 2. /* start (Skip)
        // 3. mid (Skip)
        // 4. end */ let b (LOC)
        // Total: 2
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.loc, 2);
    }

    #[test]
    fn test_function_count() {
        let parser = get_parser("typescript").unwrap();
        let source = "function a() {} function b() {}";
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.function_count, 2);
    }

    #[test]
    fn test_file_metadata_defaults_to_now() {
        let parser = get_parser("typescript").unwrap();
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let source = "let x = 1;";

        // When last_modified is None, it should default to now()
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.size_bytes, source.len() as u64);

        // last_modified should be close to now (within 1 second)
        let duration = SystemTime::now().duration_since(metrics.last_modified).unwrap();
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_file_metadata_with_provided_time() {
        let parser = get_parser("typescript").unwrap();
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let source = "let x = 1;";

        // When last_modified is provided, it should use that value
        let provided_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1000000);
        let metrics = calculate_metrics(&path, source, parser.as_ref(), Some(provided_time)).unwrap();

        assert_eq!(metrics.last_modified, provided_time);
    }

    #[test]
    fn test_ai_bloat_index_calculation() {
        let parser = get_parser("typescript").unwrap();
        let source = r#"
            // This is a comment
            let x = 1; // Another comment
        "#;
        // 2 non-blank lines, 1 LOC. So, 1 comment line.
        // (1 / 1) * 100 = 100
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.ai_bloat_index, Some(100.0));
    }

    #[test]
    fn test_ai_bloat_index_zero_loc() {
        let parser = get_parser("typescript").unwrap();
        let source = "// Just a comment";
        // 1 non-blank line, 0 LOC. So, 1 comment line.
        // Should return 999.0
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.ai_bloat_index, Some(999.0));
    }

    #[test]
    fn test_ai_bloat_index_no_comments() {
        let parser = get_parser("typescript").unwrap();
        let source = "let x = 1;";
        // 1 non-blank line, 1 LOC. 0 comment lines.
        // (0 / 1) * 100 = 0
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.ai_bloat_index, Some(0.0));
    }

    #[test]
    fn test_ai_bloat_index_empty_file() {
        let parser = get_parser("typescript").unwrap();
        let source = "";
        // 0 non-blank lines, 0 LOC. 0 comment lines.
        // Should be 0.0
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.ts");
        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap();
        assert_eq!(metrics.ai_bloat_index, Some(0.0));
    }
}
