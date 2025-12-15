use super::{FormatterError, MetricsFormatter};
use code_viz_core::AnalysisResult;

pub struct TextFormatter;

impl MetricsFormatter for TextFormatter {
    fn format(&self, _result: &AnalysisResult) -> Result<String, FormatterError> {
        todo!("Implement Text formatter")
    }
}
