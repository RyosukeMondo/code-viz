use super::{FormatterError, MetricsFormatter};
use code_viz_core::AnalysisResult;

pub struct CsvFormatter;

impl MetricsFormatter for CsvFormatter {
    fn format(&self, _result: &AnalysisResult) -> Result<String, FormatterError> {
        todo!("Implement CSV formatter")
    }
}
