use crate::models::FileMetrics;
use crate::parser::LanguageParser;
use std::path::Path;
use thiserror::Error;

pub fn calculate_metrics(
    path: &Path,
    source: &str,
    parser: &dyn LanguageParser,
) -> Result<FileMetrics, MetricsError> {
    todo!("Implement LOC calculation and metrics aggregation")
}

#[derive(Debug, Error)]
pub enum MetricsError {
    #[error("Parse failed: {0}")]
    ParseFailed(#[from] crate::parser::ParseError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
