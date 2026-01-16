use crate::models::{CoverageAnalysis, FileMetrics, TestCoverage};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Trait for parsing coverage report formats
pub trait CoverageParser {
    /// Parse coverage data from JSON string
    fn parse(&self, json: &str) -> Result<HashMap<PathBuf, TestCoverage>>;
}

/// Parser for llvm-cov JSON export format
pub struct LlvmCovParser;

/// Parser for Tarpaulin JSON format
pub struct TarpaulinParser;

// llvm-cov JSON structures
#[derive(Debug, Deserialize)]
struct LlvmCovReport {
    data: Vec<LlvmCovData>,
}

#[derive(Debug, Deserialize)]
struct LlvmCovData {
    files: Vec<LlvmCovFile>,
}

#[derive(Debug, Deserialize)]
struct LlvmCovFile {
    filename: String,
    summary: LlvmCovSummary,
}

#[derive(Debug, Deserialize)]
struct LlvmCovSummary {
    lines: LlvmCovLineSummary,
    #[serde(default)]
    branches: Option<LlvmCovBranchSummary>,
}

#[derive(Debug, Deserialize)]
struct LlvmCovLineSummary {
    count: usize,
    covered: usize,
    percent: f64,
}

#[derive(Debug, Deserialize)]
struct LlvmCovBranchSummary {
    percent: f64,
}

// Tarpaulin JSON structures
#[derive(Debug, Deserialize)]
struct TarpaulinReport {
    files: HashMap<String, TarpaulinFile>,
}

#[derive(Debug, Deserialize)]
struct TarpaulinFile {
    covered: Vec<usize>,
    coverable: Vec<usize>,
}

impl CoverageParser for LlvmCovParser {
    fn parse(&self, json: &str) -> Result<HashMap<PathBuf, TestCoverage>> {
        let report: LlvmCovReport = serde_json::from_str(json)
            .context("Failed to parse llvm-cov JSON report")?;

        let mut coverage_map = HashMap::new();

        for data in report.data {
            for file in data.files {
                let path = normalize_path(&file.filename);
                let coverage = TestCoverage {
                    line_coverage: file.summary.lines.percent,
                    lines_covered: file.summary.lines.covered,
                    total_lines: file.summary.lines.count,
                    branch_coverage: file.summary.branches.map(|b| b.percent),
                };
                coverage_map.insert(path, coverage);
            }
        }

        Ok(coverage_map)
    }
}

impl CoverageParser for TarpaulinParser {
    fn parse(&self, json: &str) -> Result<HashMap<PathBuf, TestCoverage>> {
        let report: TarpaulinReport = serde_json::from_str(json)
            .context("Failed to parse Tarpaulin JSON report")?;

        let mut coverage_map = HashMap::new();

        for (filename, file) in report.files {
            let path = normalize_path(&filename);
            let lines_covered = file.covered.len();
            let total_lines = file.coverable.len();
            let line_coverage = if total_lines > 0 {
                (lines_covered as f64 / total_lines as f64) * 100.0
            } else {
                0.0
            };

            let coverage = TestCoverage {
                line_coverage,
                lines_covered,
                total_lines,
                branch_coverage: None,
            };
            coverage_map.insert(path, coverage);
        }

        Ok(coverage_map)
    }
}

/// Normalize file paths to be relative and consistent
fn normalize_path(path_str: &str) -> PathBuf {
    let path = Path::new(path_str);

    // Strip common prefixes that might appear in coverage reports
    let stripped = path
        .strip_prefix("/")
        .or_else(|_| path.strip_prefix("./"))
        .unwrap_or(path);

    // Convert to PathBuf
    stripped.to_path_buf()
}

/// Match coverage data to file metrics by path
pub fn apply_coverage_to_metrics(
    files: &mut [FileMetrics],
    coverage_map: HashMap<PathBuf, TestCoverage>,
) {
    for file in files.iter_mut() {
        // Try exact match first
        if let Some(coverage) = coverage_map.get(&file.path) {
            file.test_coverage = Some(coverage.clone());
            continue;
        }

        // Try normalized path matching
        let normalized_file_path = normalize_path(&file.path.to_string_lossy());
        if let Some(coverage) = coverage_map.get(&normalized_file_path) {
            file.test_coverage = Some(coverage.clone());
            continue;
        }

        // Try matching by filename only (last resort)
        if let Some(filename) = file.path.file_name() {
            for (cov_path, coverage) in &coverage_map {
                if cov_path.file_name() == Some(filename) {
                    file.test_coverage = Some(coverage.clone());
                    break;
                }
            }
        }
    }
}

/// Calculate aggregated coverage analysis
pub fn calculate_coverage_analysis(files: &[FileMetrics]) -> Option<CoverageAnalysis> {
    let files_with_coverage: Vec<&FileMetrics> = files
        .iter()
        .filter(|f| f.test_coverage.is_some())
        .collect();

    if files_with_coverage.is_empty() {
        return None;
    }

    let total_lines_covered: usize = files_with_coverage
        .iter()
        .filter_map(|f| f.test_coverage.as_ref())
        .map(|cov| cov.lines_covered)
        .sum();

    let total_executable_lines: usize = files_with_coverage
        .iter()
        .filter_map(|f| f.test_coverage.as_ref())
        .map(|cov| cov.total_lines)
        .sum();

    let overall_coverage = if total_executable_lines > 0 {
        (total_lines_covered as f64 / total_executable_lines as f64) * 100.0
    } else {
        0.0
    };

    Some(CoverageAnalysis {
        overall_coverage,
        total_lines_covered,
        total_executable_lines,
        file_count: files_with_coverage.len(),
    })
}

/// Auto-detect coverage format and parse
pub fn parse_coverage_report(json: &str) -> Result<HashMap<PathBuf, TestCoverage>> {
    // Try llvm-cov format first
    if let Ok(coverage) = LlvmCovParser.parse(json) {
        return Ok(coverage);
    }

    // Try tarpaulin format
    if let Ok(coverage) = TarpaulinParser.parse(json) {
        return Ok(coverage);
    }

    anyhow::bail!("Unsupported coverage report format. Expected llvm-cov or Tarpaulin JSON.")
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llvm_cov_parser() {
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
        let coverage = parser.parse(json).unwrap(); // Test-only unwrap: test JSON is valid

        assert_eq!(coverage.len(), 1);
        let main_coverage = coverage.get(&PathBuf::from("src/main.rs")).unwrap(); // Test-only unwrap: test data contains this file
        assert_eq!(main_coverage.line_coverage, 80.0);
        assert_eq!(main_coverage.lines_covered, 80);
        assert_eq!(main_coverage.total_lines, 100);
        assert_eq!(main_coverage.branch_coverage, Some(75.0));
    }

    #[test]
    fn test_tarpaulin_parser() {
        let json = r#"{
            "files": {
                "src/lib.rs": {
                    "covered": [1, 2, 3, 4, 5],
                    "coverable": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
                }
            }
        }"#;

        let parser = TarpaulinParser;
        let coverage = parser.parse(json).unwrap(); // Test-only unwrap: test JSON is valid

        assert_eq!(coverage.len(), 1);
        let lib_coverage = coverage.get(&PathBuf::from("src/lib.rs")).unwrap(); // Test-only unwrap: test data contains this file
        assert_eq!(lib_coverage.line_coverage, 50.0);
        assert_eq!(lib_coverage.lines_covered, 5);
        assert_eq!(lib_coverage.total_lines, 10);
        assert_eq!(lib_coverage.branch_coverage, None);
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("/home/user/project/src/main.rs"), PathBuf::from("home/user/project/src/main.rs"));
        assert_eq!(normalize_path("./src/lib.rs"), PathBuf::from("src/lib.rs"));
        assert_eq!(normalize_path("src/utils.rs"), PathBuf::from("src/utils.rs"));
    }
}
