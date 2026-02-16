use super::{FormatterError, MetricsFormatter};
use code_viz_core::AnalysisResult;
use std::fmt::Write;

pub struct TextFormatter;

impl MetricsFormatter for TextFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, FormatterError> {
        let mut output = String::new();
        let summary = &result.summary;

        writeln!(output, "Code Analysis Summary").map_err(|_| FormatterError::FormattingFailed)?;
        writeln!(output, "=====================").map_err(|_| FormatterError::FormattingFailed)?;
        writeln!(output, "Total Files: {}", summary.total_files)
            .map_err(|_| FormatterError::FormattingFailed)?;
        writeln!(output, "Total LOC:   {}", summary.total_loc)
            .map_err(|_| FormatterError::FormattingFailed)?;
        writeln!(output, "Functions:   {}", summary.total_functions)
            .map_err(|_| FormatterError::FormattingFailed)?;
        writeln!(output).map_err(|_| FormatterError::FormattingFailed)?;

        writeln!(output, "Largest Files:").map_err(|_| FormatterError::FormattingFailed)?;

        // Find top 10 files by LOC
        let mut files: Vec<_> = result.files.iter().collect();
        files.sort_by(|a, b| b.loc.cmp(&a.loc));

        for (i, file) in files.iter().take(10).enumerate() {
            writeln!(
                output,
                "  {}. {} ({} LOC)",
                i + 1,
                file.path.display(),
                file.loc
            )
            .map_err(|_| FormatterError::FormattingFailed)?;
        }

        if let Some(ai_analysis) = &result.ai_commit_analysis {
            writeln!(output).map_err(|_| FormatterError::FormattingFailed)?;
            writeln!(output, "AI Commit Analysis").map_err(|_| FormatterError::FormattingFailed)?;
            writeln!(output, "==================").map_err(|_| FormatterError::FormattingFailed)?;
            writeln!(
                output,
                "Total Commits Scanned: {}",
                ai_analysis.total_commits
            )
            .map_err(|_| FormatterError::FormattingFailed)?;
            writeln!(
                output,
                "AI-Generated Commits: {} ({:.1}%)",
                ai_analysis.ai_generated_count,
                if ai_analysis.total_commits > 0 {
                    (ai_analysis.ai_generated_count as f32 / ai_analysis.total_commits as f32)
                        * 100.0
                } else {
                    0.0
                }
            )
            .map_err(|_| FormatterError::FormattingFailed)?;

            if !ai_analysis.confidence_scores.is_empty() {
                writeln!(output, "\nCommits with Highest Confidence:")
                    .map_err(|_| FormatterError::FormattingFailed)?;
                let mut scores = ai_analysis.confidence_scores.clone();
                scores.sort_by(|a, b| b.1.cmp(&a.1));
                for (sha, score) in scores.iter().take(5) {
                    writeln!(output, "  - {} ({}% confidence)", &sha[..7], score)
                        .map_err(|_| FormatterError::FormattingFailed)?;
                }
            }
        }

        Ok(output)
    }
}
