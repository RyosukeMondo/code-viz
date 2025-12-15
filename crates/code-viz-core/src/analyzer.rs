use crate::models::{AnalysisResult, AnalysisConfig};
use crate::scanner::ScanError;
use crate::metrics::MetricsError;
use crate::cache::CacheError;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn analyze(
    root: &Path,
    config: &AnalysisConfig,
) -> Result<AnalysisResult, AnalysisError> {
    todo!("Orchestrate scan → parse → metrics pipeline")
}

#[derive(Debug, Error)]
pub enum AnalysisError {
    #[error("Failed to scan directory: {0}")]
    ScanFailed(#[from] ScanError),

    #[error("Parse error in {path}: {source}")]
    ParseFailed {
        path: PathBuf,
        source: crate::parser::ParseError,
    },

    #[error("Metrics calculation error: {0}")]
    MetricsFailed(#[from] MetricsError),

    #[error("Cache error: {0}")]
    CacheFailed(#[from] CacheError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
