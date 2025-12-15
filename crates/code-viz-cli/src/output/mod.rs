use code_viz_core::AnalysisResult;
use thiserror::Error;

pub mod csv;
pub mod json;
pub mod text;

#[derive(Error, Debug)]
pub enum FormatterError {
    #[error("Formatting failed")]
    FormattingFailed,
}

pub trait MetricsFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String, FormatterError>;
}
