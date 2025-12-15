use super::{FormatterError, MetricsFormatter};
use code_viz_core::AnalysisResult;

pub struct JsonFormatter;

impl MetricsFormatter for JsonFormatter {
    fn format(&self, _result: &AnalysisResult) -> Result<String, FormatterError> {
        todo!("Implement JSON formatter")
    }
}
