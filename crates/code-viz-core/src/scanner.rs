use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn scan_directory(
    path: &Path,
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>, ScanError> {
    todo!("Implement file discovery with glob exclusions")
}

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),

    #[error("Access denied: {0}")]
    PermissionDenied(PathBuf),
}
