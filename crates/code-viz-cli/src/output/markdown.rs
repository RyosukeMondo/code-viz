use super::{FormatterError, MetricsFormatter};
use anyhow::Result;
use code_viz_core::{AnalysisResult, FileMetrics};
use std::collections::HashMap;

pub struct MarkdownFormatter;

impl MetricsFormatter for MarkdownFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, FormatterError> {
        let mut output = String::new();
        output.push_str("# Code Analysis Report\n\n");
        output.push_str(&render_summary(&result.summary));
        output.push_str(&render_language_breakdown(&result.files));
        output.push_str(&render_file_metrics(&result.files));
        Ok(output)
    }
}

fn render_summary(summary: &code_viz_core::Summary) -> String {
    let mut s = String::new();
    s.push_str("## Summary\n\n");
    s.push_str("| Metric          | Value   |\n");
    s.push_str("|-----------------|---------|\n");
    s.push_str(&format!("| Total Files     | {}      |\n", summary.total_files));
    s.push_str(&format!("| Total LOC       | {}      |\n", summary.total_loc));
    s.push_str(&format!(
        "| Total Functions | {}      |\n",
        summary.total_functions
    ));
    s.push_str("\n");
    s
}

fn render_language_breakdown(files: &[FileMetrics]) -> String {
    let mut breakdown = HashMap::new();
    for file in files {
        *breakdown.entry(file.language.clone()).or_insert(0) += file.loc;
    }

    let mut sorted_breakdown: Vec<_> = breakdown.into_iter().collect();
    sorted_breakdown.sort_by(|a, b| b.1.cmp(&a.1));

    let mut s = String::new();
    s.push_str("## LOC Breakdown by Language\n\n");
    s.push_str("| Language   | Lines of Code |\n");
    s.push_str("|------------|---------------|\n");
    for (language, loc) in sorted_breakdown {
        s.push_str(&format!("| {}      | {}            |\n", language, loc));
    }
    s.push_str("\n");
    s
}

fn render_file_metrics(files: &[FileMetrics]) -> String {
    let mut sorted_files = files.to_vec();
    sorted_files.sort_by(|a, b| b.loc.cmp(&a.loc));

    let mut s = String::new();
    s.push_str("## File Metrics\n\n");
    s.push_str("| File Path | Language | LOC | Functions |\n");
    s.push_str("|---|---|---|---|\n");
    for file in sorted_files {
        s.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            file.path.display(),
            file.language,
            file.loc,
            file.function_count
        ));
    }
    s
}
