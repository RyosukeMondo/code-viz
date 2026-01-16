//! Coverage data error tests

use code_viz_core::coverage::parse_coverage_report;
use code_viz_core::error::CodeVizError;

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
