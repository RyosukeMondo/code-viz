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
        writeln!(output, "Total Files: {}", summary.total_files).map_err(|_| FormatterError::FormattingFailed)?;
        writeln!(output, "Total LOC:   {}", summary.total_loc).map_err(|_| FormatterError::FormattingFailed)?;
        writeln!(output, "Functions:   {}", summary.total_functions).map_err(|_| FormatterError::FormattingFailed)?;
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
            ).map_err(|_| FormatterError::FormattingFailed)?;
        }

        Ok(output)
    }
}
