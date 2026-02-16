#![allow(clippy::unwrap_used, clippy::expect_used)]

use serde_json::Value;

/// Assert that JSON output contains expected fields
pub fn assert_json_has_fields(json_str: &str, fields: &[&str]) {
    let json: Value = serde_json::from_str(json_str).expect("Failed to parse JSON output");

    for field in fields {
        assert!(
            json.get(field).is_some(),
            "JSON output missing field: {}",
            field
        );
    }
}

/// Assert that JSON summary matches expected values
pub fn assert_summary_stats(json_str: &str, expected_files: usize, min_loc: usize) {
    let json: Value = serde_json::from_str(json_str).expect("Failed to parse JSON output");

    let summary = json.get("summary").expect("JSON missing 'summary' field");

    let total_files = summary
        .get("total_files")
        .and_then(|v| v.as_u64())
        .expect("summary.total_files not found or not a number");

    let total_loc = summary
        .get("total_loc")
        .and_then(|v| v.as_u64())
        .expect("summary.total_loc not found or not a number");

    assert_eq!(
        total_files as usize, expected_files,
        "Expected {} files, got {}",
        expected_files, total_files
    );

    assert!(
        total_loc as usize >= min_loc,
        "Expected at least {} LOC, got {}",
        min_loc,
        total_loc
    );
}

/// Assert that duplication analysis was included
pub fn assert_has_duplicates(json_str: &str) {
    let json: Value = serde_json::from_str(json_str).expect("Failed to parse JSON output");

    let duplication = json
        .get("duplication")
        .expect("JSON missing 'duplication' field");

    let pairs = duplication
        .get("pairs")
        .and_then(|v| v.as_array())
        .expect("duplication.pairs not found or not an array");

    assert!(
        !pairs.is_empty(),
        "Expected duplicate pairs, but found none"
    );
}
