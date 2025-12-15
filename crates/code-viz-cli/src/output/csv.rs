use super::{FormatterError, MetricsFormatter};
use code_viz_core::AnalysisResult;
use csv::WriterBuilder;

pub struct CsvFormatter;

impl MetricsFormatter for CsvFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, FormatterError> {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);

        // Write header
        wtr.write_record(&["path", "language", "loc", "functions", "size_bytes"])
            .map_err(|_| FormatterError::FormattingFailed)?;

        for file in &result.files {
            wtr.write_record(&[
                file.path.to_string_lossy().as_ref(),
                &file.language,
                &file.loc.to_string(),
                &file.function_count.to_string(),
                &file.size_bytes.to_string(),
            ])
            .map_err(|_| FormatterError::FormattingFailed)?;
        }

        let data = wtr.into_inner().map_err(|_| FormatterError::FormattingFailed)?;
        String::from_utf8(data).map_err(|_| FormatterError::FormattingFailed)
    }
}
