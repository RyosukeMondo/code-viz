use super::{FormatterError, MetricsFormatter};
use code_viz_core::AnalysisResult;
use csv::WriterBuilder;

pub struct CsvFormatter;

impl MetricsFormatter for CsvFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, FormatterError> {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);

        // Check if any file has dead code metrics
        let has_dead_code = result.files.iter().any(|f| f.dead_code_ratio.is_some());

        // Write header
        let header = if has_dead_code {
            vec!["path", "language", "loc", "functions", "size_bytes", "dead_functions", "dead_loc", "dead_code_ratio"]
        } else {
            vec!["path", "language", "loc", "functions", "size_bytes"]
        };
        wtr.write_record(&header)
            .map_err(|_| FormatterError::FormattingFailed)?;

        for file in &result.files {
            let mut record = vec![
                file.path.to_string_lossy().to_string(),
                file.language.clone(),
                file.loc.to_string(),
                file.function_count.to_string(),
                file.size_bytes.to_string(),
            ];

            if has_dead_code {
                record.push(file.dead_function_count.map_or_else(|| "0".to_string(), |v| v.to_string()));
                record.push(file.dead_code_loc.map_or_else(|| "0".to_string(), |v| v.to_string()));
                record.push(file.dead_code_ratio.map_or_else(|| "0.0".to_string(), |v| format!("{:.4}", v)));
            }

            wtr.write_record(&record)
                .map_err(|_| FormatterError::FormattingFailed)?;
        }

        let data = wtr.into_inner().map_err(|_| FormatterError::FormattingFailed)?;
        String::from_utf8(data).map_err(|_| FormatterError::FormattingFailed)
    }
}
