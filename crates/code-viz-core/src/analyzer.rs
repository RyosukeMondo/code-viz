pub mod ai_commit_analyzer;

use crate::models::{Summary, FileMetrics};
use crate::scanner::ScanError;
use crate::metrics::{self, MetricsError};
use crate::cache::CacheError;
use crate::parser;
use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::traits::FileSystem;

/// Extract file extension and convert to string
fn get_file_extension(path: &Path) -> Result<&str, AnalysisError> {
    path.extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| AnalysisError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No extension"
        )))
}

/// Map file extension to language key
fn extension_to_language(extension: &str) -> &str {
    match extension {
        "ts" => "typescript",
        "tsx" => "tsx",
        "js" => "javascript",
        "jsx" => "javascript",
        "rs" => "rust",
        "py" => "python",
        "go" => "go",
        "cpp" | "cxx" | "cc" | "hpp" | "h" => "cpp",
        ext => ext,
    }
}

/// Process a single file with FileSystem trait (NEW - trait-based)
#[tracing::instrument(skip(fs), fields(path = %path.display()))]
#[allow(clippy::cognitive_complexity)]
pub fn process_file_with_fs(path: &Path, fs: &impl FileSystem) -> Result<FileMetrics, AnalysisError> {
    tracing::debug!("Processing file");

    let extension = get_file_extension(path)?;
    let language_key = extension_to_language(extension);

    tracing::debug!(extension = %extension, language = %language_key, "Language detected");

    let parser = parser::get_parser(language_key)
        .map_err(|e| AnalysisError::ParseFailed { path: path.to_path_buf(), source: e })?;

    let source = fs.read_to_string(path)
        .map_err(|e| AnalysisError::IoError(std::io::Error::other(e)))?;
    tracing::debug!(source_size = source.len(), "File read successfully");

    let metrics = metrics::calculate_metrics(path, &source, parser.as_ref(), None)
        .map_err(AnalysisError::MetricsFailed)?;

    tracing::debug!(loc = metrics.loc, functions = metrics.function_count, "Metrics calculated");

    Ok(metrics)
}

/// Extract the top N largest files by LOC
fn get_largest_files(files: &[FileMetrics], count: usize) -> Vec<PathBuf> {
    let mut sorted_files: Vec<&FileMetrics> = files.iter().collect();
    sorted_files.sort_by(|a, b| b.loc.cmp(&a.loc));

    sorted_files
        .iter()
        .take(count)
        .map(|f| f.path.clone())
        .collect()
}

#[tracing::instrument(skip(files), fields(file_count = files.len()))]
#[allow(clippy::cognitive_complexity)]
pub fn calculate_summary(files: &[FileMetrics]) -> Summary {
    tracing::debug!("Calculating summary statistics");

    let total_files = files.len();
    let total_loc = files.iter().map(|f| f.loc).sum();
    let total_functions = files.iter().map(|f| f.function_count).sum();

    tracing::debug!(
        total_files = total_files,
        total_loc = total_loc,
        total_functions = total_functions,
        "Basic statistics calculated"
    );

    let largest_files = get_largest_files(files, 10);
    tracing::debug!(largest_files_count = largest_files.len(), "Identified largest files");

    Summary {
        total_files,
        total_loc,
        total_functions,
        largest_files,
    }
}

#[derive(Debug, Error)]
pub enum AnalysisError {
    #[error("Failed to scan directory: {0}")]
    ScanFailed(#[from] ScanError),

    #[error("Parse error in {path:?}: {source}")]
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

