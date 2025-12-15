use super::{FormatterError, MetricsFormatter};
use code_viz_core::AnalysisResult;

pub struct JsonFormatter;

impl MetricsFormatter for JsonFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, FormatterError> {
        serde_json::to_string_pretty(result).map_err(|_| FormatterError::FormattingFailed)
    }
}
