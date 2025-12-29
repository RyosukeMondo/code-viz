use code_viz_core::AnalysisResult;
use thiserror::Error;

pub mod csv;
pub mod dead_code;
pub mod json;
pub mod markdown;
pub mod text;

#[derive(Error, Debug)]
pub enum FormatterError {
    #[error("Formatting failed")]
    FormattingFailed,
}

pub trait MetricsFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, FormatterError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use code_viz_core::models::{Summary, FileMetrics};
    use std::path::PathBuf;
    use std::time::SystemTime;

    fn create_sample_result() -> AnalysisResult {
        let files = vec![
            FileMetrics {
                path: PathBuf::from("src/main.rs"),
                language: "rust".to_string(),
                loc: 100,
                size_bytes: 1024,
                function_count: 5,
                last_modified: SystemTime::now(),
                dead_function_count: None,
                dead_code_loc: None,
                dead_code_ratio: None,
                code_churn: None,
                coupling: None,
                ai_bloat_index: None,
                cognitive_complexity: None,
            },
            FileMetrics {
                path: PathBuf::from("src/lib.rs"),
                language: "rust".to_string(),
                loc: 50,
                size_bytes: 512,
                function_count: 2,
                last_modified: SystemTime::now(),
                dead_function_count: None,
                dead_code_loc: None,
                dead_code_ratio: None,
                code_churn: None,
                coupling: None,
                ai_bloat_index: None,
                cognitive_complexity: None,
            },
        ];

        let summary = Summary {
            total_files: 2,
            total_loc: 150,
            total_functions: 7,
            largest_files: vec![PathBuf::from("src/main.rs"), PathBuf::from("src/lib.rs")],
        };

        AnalysisResult {
            summary,
            files,
            timestamp: SystemTime::now(),
            duplication: None,
            ai_commit_analysis: None,
            hotspot_analysis: None,
        }
    }

    #[test]
    fn test_json_formatter() {
        let result = create_sample_result();
        let formatter = json::JsonFormatter;
        let output = formatter.format(&result).unwrap();
        
        // Verify it parses back
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["summary"]["total_files"], 2);
        assert_eq!(parsed["summary"]["total_loc"], 150);
        assert!(parsed["files"].as_array().unwrap().len() == 2);
    }

    #[test]
    fn test_csv_formatter() {
        let result = create_sample_result();
        let formatter = csv::CsvFormatter;
        let output = formatter.format(&result).unwrap();
        
        let lines: Vec<&str> = output.lines().collect();
        assert!(lines.len() >= 3); // Header + 2 rows
        assert_eq!(lines[0], "path,language,loc,functions,size_bytes");
        assert!(lines[1].contains("src/main.rs"));
        assert!(lines[1].contains("100"));
    }

    #[test]
    fn test_text_formatter() {
        let result = create_sample_result();
        let formatter = text::TextFormatter;
        let output = formatter.format(&result).unwrap();
        
        assert!(output.contains("Total Files: 2"));
        assert!(output.contains("Total LOC:   150"));
        assert!(output.contains("Largest Files:"));
        assert!(output.contains("src/main.rs (100 LOC)"));
    }

    #[test]
    fn test_markdown_formatter() {
        let result = create_sample_result();
        let formatter = markdown::MarkdownFormatter;
        let output = formatter.format(&result).unwrap();
        
        assert!(output.contains("# Code Analysis Report"));
        assert!(output.contains("## Summary"));
        assert!(output.contains("| Total Files     | 2      |"));
        assert!(output.contains("## LOC Breakdown by Language"));
        assert!(output.contains("| rust      | 150            |"));
        assert!(output.contains("## File Metrics"));
        assert!(output.contains("| src/main.rs | rust | 100 | 5 |"));
    }
}
