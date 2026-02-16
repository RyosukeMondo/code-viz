#![allow(clippy::unwrap_used, clippy::expect_used)]
use code_viz_core::coverage::{
    apply_coverage_to_metrics, calculate_coverage_analysis, parse_coverage_report, CoverageParser,
    LlvmCovParser, TarpaulinParser,
};
use code_viz_core::models::{FileMetrics, TestCoverage};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

fn create_test_file_metrics(path: &str, loc: usize) -> FileMetrics {
    FileMetrics {
        path: PathBuf::from(path),
        language: "rust".to_string(),
        loc,
        size_bytes: 1000,
        function_count: 5,
        last_modified: SystemTime::now(),
        dead_function_count: None,
        dead_code_loc: None,
        dead_code_ratio: None,
        code_churn: None,
        coupling: None,
        ai_bloat_index: None,
        cognitive_complexity: None,
        test_coverage: None,
    }
}

#[test]
fn test_llvm_cov_parser_basic() {
    let json = r#"{
        "data": [{
            "files": [{
                "filename": "src/main.rs",
                "summary": {
                    "lines": {
                        "count": 100,
                        "covered": 80,
                        "percent": 80.0
                    },
                    "branches": {
                        "percent": 75.0
                    }
                }
            }]
        }]
    }"#;

    let parser = LlvmCovParser;
    let coverage = parser.parse(json).unwrap();

    assert_eq!(coverage.len(), 1);
    let main_coverage = coverage.get(&PathBuf::from("src/main.rs")).unwrap();
    assert_eq!(main_coverage.line_coverage, 80.0);
    assert_eq!(main_coverage.lines_covered, 80);
    assert_eq!(main_coverage.total_lines, 100);
    assert_eq!(main_coverage.branch_coverage, Some(75.0));
}

#[test]
fn test_llvm_cov_parser_no_branches() {
    let json = r#"{
        "data": [{
            "files": [{
                "filename": "src/lib.rs",
                "summary": {
                    "lines": {
                        "count": 50,
                        "covered": 50,
                        "percent": 100.0
                    }
                }
            }]
        }]
    }"#;

    let parser = LlvmCovParser;
    let coverage = parser.parse(json).unwrap();

    assert_eq!(coverage.len(), 1);
    let lib_coverage = coverage.get(&PathBuf::from("src/lib.rs")).unwrap();
    assert_eq!(lib_coverage.line_coverage, 100.0);
    assert_eq!(lib_coverage.branch_coverage, None);
}

#[test]
fn test_llvm_cov_parser_multiple_files() {
    let json = r#"{
        "data": [{
            "files": [
                {
                    "filename": "src/main.rs",
                    "summary": {
                        "lines": {
                            "count": 100,
                            "covered": 80,
                            "percent": 80.0
                        }
                    }
                },
                {
                    "filename": "src/lib.rs",
                    "summary": {
                        "lines": {
                            "count": 50,
                            "covered": 45,
                            "percent": 90.0
                        }
                    }
                }
            ]
        }]
    }"#;

    let parser = LlvmCovParser;
    let coverage = parser.parse(json).unwrap();

    assert_eq!(coverage.len(), 2);
}

#[test]
fn test_llvm_cov_parser_invalid_json() {
    let json = "{ invalid json }";

    let parser = LlvmCovParser;
    let result = parser.parse(json);

    assert!(result.is_err());
}

#[test]
fn test_tarpaulin_parser_basic() {
    let json = r#"{
        "files": {
            "src/lib.rs": {
                "covered": [1, 2, 3, 4, 5],
                "coverable": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            }
        }
    }"#;

    let parser = TarpaulinParser;
    let coverage = parser.parse(json).unwrap();

    assert_eq!(coverage.len(), 1);
    let lib_coverage = coverage.get(&PathBuf::from("src/lib.rs")).unwrap();
    assert_eq!(lib_coverage.line_coverage, 50.0);
    assert_eq!(lib_coverage.lines_covered, 5);
    assert_eq!(lib_coverage.total_lines, 10);
    assert_eq!(lib_coverage.branch_coverage, None);
}

#[test]
fn test_tarpaulin_parser_full_coverage() {
    let json = r#"{
        "files": {
            "src/main.rs": {
                "covered": [1, 2, 3],
                "coverable": [1, 2, 3]
            }
        }
    }"#;

    let parser = TarpaulinParser;
    let coverage = parser.parse(json).unwrap();

    let main_coverage = coverage.get(&PathBuf::from("src/main.rs")).unwrap();
    assert_eq!(main_coverage.line_coverage, 100.0);
}

#[test]
fn test_tarpaulin_parser_zero_coverage() {
    let json = r#"{
        "files": {
            "src/dead.rs": {
                "covered": [],
                "coverable": [1, 2, 3, 4, 5]
            }
        }
    }"#;

    let parser = TarpaulinParser;
    let coverage = parser.parse(json).unwrap();

    let dead_coverage = coverage.get(&PathBuf::from("src/dead.rs")).unwrap();
    assert_eq!(dead_coverage.line_coverage, 0.0);
    assert_eq!(dead_coverage.lines_covered, 0);
    assert_eq!(dead_coverage.total_lines, 5);
}

#[test]
fn test_tarpaulin_parser_no_lines() {
    let json = r#"{
        "files": {
            "src/empty.rs": {
                "covered": [],
                "coverable": []
            }
        }
    }"#;

    let parser = TarpaulinParser;
    let coverage = parser.parse(json).unwrap();

    let empty_coverage = coverage.get(&PathBuf::from("src/empty.rs")).unwrap();
    assert_eq!(empty_coverage.line_coverage, 0.0);
    assert_eq!(empty_coverage.total_lines, 0);
}

#[test]
fn test_tarpaulin_parser_multiple_files() {
    let json = r#"{
        "files": {
            "src/main.rs": {
                "covered": [1, 2],
                "coverable": [1, 2, 3, 4]
            },
            "src/lib.rs": {
                "covered": [1, 2, 3],
                "coverable": [1, 2, 3]
            }
        }
    }"#;

    let parser = TarpaulinParser;
    let coverage = parser.parse(json).unwrap();

    assert_eq!(coverage.len(), 2);
}

#[test]
fn test_apply_coverage_exact_match() {
    let mut files = vec![create_test_file_metrics("src/main.rs", 100)];

    let mut coverage_map = HashMap::new();
    coverage_map.insert(
        PathBuf::from("src/main.rs"),
        TestCoverage {
            line_coverage: 80.0,
            lines_covered: 80,
            total_lines: 100,
            branch_coverage: None,
        },
    );

    apply_coverage_to_metrics(&mut files, coverage_map);

    assert!(files[0].test_coverage.is_some());
    assert_eq!(files[0].test_coverage.as_ref().unwrap().line_coverage, 80.0);
}

#[test]
fn test_apply_coverage_normalized_path() {
    let mut files = vec![create_test_file_metrics("src/lib.rs", 50)];

    let mut coverage_map = HashMap::new();
    coverage_map.insert(
        PathBuf::from("src/lib.rs"),
        TestCoverage {
            line_coverage: 90.0,
            lines_covered: 45,
            total_lines: 50,
            branch_coverage: None,
        },
    );

    apply_coverage_to_metrics(&mut files, coverage_map);

    assert!(files[0].test_coverage.is_some());
    assert_eq!(files[0].test_coverage.as_ref().unwrap().line_coverage, 90.0);
}

#[test]
fn test_apply_coverage_filename_match() {
    let mut files = vec![create_test_file_metrics("project/src/main.rs", 100)];

    let mut coverage_map = HashMap::new();
    coverage_map.insert(
        PathBuf::from("other/path/main.rs"),
        TestCoverage {
            line_coverage: 70.0,
            lines_covered: 70,
            total_lines: 100,
            branch_coverage: None,
        },
    );

    apply_coverage_to_metrics(&mut files, coverage_map);

    assert!(files[0].test_coverage.is_some());
    assert_eq!(files[0].test_coverage.as_ref().unwrap().line_coverage, 70.0);
}

#[test]
fn test_apply_coverage_no_match() {
    let mut files = vec![create_test_file_metrics("src/main.rs", 100)];

    let mut coverage_map = HashMap::new();
    coverage_map.insert(
        PathBuf::from("src/other.rs"),
        TestCoverage {
            line_coverage: 80.0,
            lines_covered: 80,
            total_lines: 100,
            branch_coverage: None,
        },
    );

    apply_coverage_to_metrics(&mut files, coverage_map);

    assert!(files[0].test_coverage.is_none());
}

#[test]
fn test_apply_coverage_multiple_files() {
    let mut files = vec![
        create_test_file_metrics("src/main.rs", 100),
        create_test_file_metrics("src/lib.rs", 50),
        create_test_file_metrics("src/utils.rs", 30),
    ];

    let mut coverage_map = HashMap::new();
    coverage_map.insert(
        PathBuf::from("src/main.rs"),
        TestCoverage {
            line_coverage: 80.0,
            lines_covered: 80,
            total_lines: 100,
            branch_coverage: None,
        },
    );
    coverage_map.insert(
        PathBuf::from("src/lib.rs"),
        TestCoverage {
            line_coverage: 90.0,
            lines_covered: 45,
            total_lines: 50,
            branch_coverage: None,
        },
    );

    apply_coverage_to_metrics(&mut files, coverage_map);

    assert!(files[0].test_coverage.is_some());
    assert!(files[1].test_coverage.is_some());
    assert!(files[2].test_coverage.is_none()); // No coverage data for utils.rs
}

#[test]
fn test_calculate_coverage_analysis_basic() {
    let mut file1 = create_test_file_metrics("src/main.rs", 100);
    file1.test_coverage = Some(TestCoverage {
        line_coverage: 80.0,
        lines_covered: 80,
        total_lines: 100,
        branch_coverage: None,
    });

    let mut file2 = create_test_file_metrics("src/lib.rs", 50);
    file2.test_coverage = Some(TestCoverage {
        line_coverage: 90.0,
        lines_covered: 45,
        total_lines: 50,
        branch_coverage: None,
    });

    let files = vec![file1, file2];
    let analysis = calculate_coverage_analysis(&files).unwrap();

    assert_eq!(analysis.total_lines_covered, 125);
    assert_eq!(analysis.total_executable_lines, 150);
    assert_eq!(analysis.file_count, 2);
    assert!((analysis.overall_coverage - 83.33).abs() < 0.1);
}

#[test]
fn test_calculate_coverage_analysis_no_coverage() {
    let files = vec![
        create_test_file_metrics("src/main.rs", 100),
        create_test_file_metrics("src/lib.rs", 50),
    ];

    let analysis = calculate_coverage_analysis(&files);
    assert!(analysis.is_none());
}

#[test]
fn test_calculate_coverage_analysis_partial_coverage() {
    let mut file1 = create_test_file_metrics("src/main.rs", 100);
    file1.test_coverage = Some(TestCoverage {
        line_coverage: 80.0,
        lines_covered: 80,
        total_lines: 100,
        branch_coverage: None,
    });

    let file2 = create_test_file_metrics("src/lib.rs", 50); // No coverage

    let files = vec![file1, file2];
    let analysis = calculate_coverage_analysis(&files).unwrap();

    assert_eq!(analysis.file_count, 1); // Only one file has coverage
    assert_eq!(analysis.total_lines_covered, 80);
    assert_eq!(analysis.total_executable_lines, 100);
}

#[test]
fn test_calculate_coverage_analysis_zero_lines() {
    let mut file = create_test_file_metrics("src/empty.rs", 0);
    file.test_coverage = Some(TestCoverage {
        line_coverage: 0.0,
        lines_covered: 0,
        total_lines: 0,
        branch_coverage: None,
    });

    let files = vec![file];
    let analysis = calculate_coverage_analysis(&files).unwrap();

    assert_eq!(analysis.overall_coverage, 0.0);
    assert_eq!(analysis.total_executable_lines, 0);
}

#[test]
fn test_parse_coverage_report_llvm_cov() {
    let json = r#"{
        "data": [{
            "files": [{
                "filename": "src/main.rs",
                "summary": {
                    "lines": {
                        "count": 100,
                        "covered": 80,
                        "percent": 80.0
                    }
                }
            }]
        }]
    }"#;

    let coverage = parse_coverage_report(json).unwrap();
    assert_eq!(coverage.len(), 1);
}

#[test]
fn test_parse_coverage_report_tarpaulin() {
    let json = r#"{
        "files": {
            "src/lib.rs": {
                "covered": [1, 2, 3],
                "coverable": [1, 2, 3, 4, 5]
            }
        }
    }"#;

    let coverage = parse_coverage_report(json).unwrap();
    assert_eq!(coverage.len(), 1);
}

#[test]
fn test_parse_coverage_report_invalid_format() {
    let json = r#"{ "unknown": "format" }"#;

    let result = parse_coverage_report(json);
    assert!(result.is_err());

    let error_msg = result.err().unwrap().to_string();
    assert!(error_msg.contains("Unsupported coverage report format"));
}

#[test]
fn test_parse_coverage_report_empty_json() {
    let json = "{}";

    let result = parse_coverage_report(json);
    assert!(result.is_err());
}
