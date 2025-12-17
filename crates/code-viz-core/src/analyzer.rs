use crate::models::{AnalysisResult, AnalysisConfig, Summary, FileMetrics};
use crate::scanner::{self, ScanError};
use crate::metrics::{self, MetricsError};
use crate::cache::CacheError;
use crate::parser;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use thiserror::Error;

#[tracing::instrument(skip(config), fields(root = %root.display(), exclude_patterns = config.exclude_patterns.len()))]
pub fn analyze(
    root: &Path,
    config: &AnalysisConfig,
) -> Result<AnalysisResult, AnalysisError> {
    tracing::info!("Starting repository analysis");

    let files = scanner::scan_directory(root, &config.exclude_patterns)?;
    tracing::info!(file_count = files.len(), "Directory scan completed");

    // Process files in parallel
    let mut results: Vec<FileMetrics> = files
        .par_iter()
        .filter_map(|path| {
            process_file(path).map_err(|e| {
                tracing::warn!(path = %path.display(), error = %e, "Failed to analyze file, skipping");
                e
            }).ok()
        })
        .collect();

    tracing::debug!(processed_files = results.len(), "File processing completed");

    // Sort results by path for deterministic output
    results.sort_by(|a, b| a.path.cmp(&b.path));

    let summary = calculate_summary(&results);
    tracing::info!(
        total_files = summary.total_files,
        total_loc = summary.total_loc,
        total_functions = summary.total_functions,
        "Analysis completed successfully"
    );

    Ok(AnalysisResult {
        summary,
        files: results,
        timestamp: SystemTime::now(),
    })
}

#[tracing::instrument(fields(path = %path.display()))]
pub fn process_file(path: &Path) -> Result<FileMetrics, AnalysisError> {
    tracing::debug!("Processing file");

    let extension = path.extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| AnalysisError::IoError(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No extension")))?;

    // Map extension to language (simple mapping for now, or let get_parser handle it)
    // get_parser handles "ts", "js", "tsx", etc.
    // We assume the extension *is* the language key for get_parser, or we map it.
    // parser::get_parser handles "typescript" | "ts", etc.
    let language_key = match extension {
        "ts" => "typescript",
        "tsx" => "tsx",
        "js" => "javascript",
        "jsx" => "javascript",
        "rs" => "rust",
        "py" => "python",
        "go" => "go",
        "cpp" | "cxx" | "cc" | "hpp" | "h" => "cpp",
        ext => ext,
    };

    tracing::debug!(extension = %extension, language = %language_key, "Language detected");

    let parser = parser::get_parser(language_key)
        .map_err(|e| AnalysisError::ParseFailed { path: path.to_path_buf(), source: e })?;

    let source = fs::read_to_string(path)?;
    tracing::debug!(source_size = source.len(), "File read successfully");

    let metrics = metrics::calculate_metrics(path, &source, parser.as_ref())
        .map_err(AnalysisError::MetricsFailed)?;

    tracing::debug!(loc = metrics.loc, functions = metrics.function_count, "Metrics calculated");

    Ok(metrics)
}

#[tracing::instrument(skip(files), fields(file_count = files.len()))]
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

    // Find top 10 largest files by LOC
    let mut sorted_files: Vec<&FileMetrics> = files.iter().collect();
    sorted_files.sort_by(|a, b| b.loc.cmp(&a.loc)); // Descending LOC

    let largest_files: Vec<PathBuf> = sorted_files
        .iter()
        .take(10)
        .map(|f| f.path.clone())
        .collect();

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_analyze_repository() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create a few files
        let ts_file = root.join("test.ts");
        let mut f = File::create(&ts_file).unwrap();
        writeln!(f, "function test() {{ console.log('hi'); }}").unwrap(); // 1 LOC, 1 func

        let js_file = root.join("index.js");
        let mut f = File::create(&js_file).unwrap();
        writeln!(f, "// Comment\nconst x = 1;").unwrap(); // 1 LOC, 0 func

        // Ignored file (no extension supported yet or ignored by scanner?)
        // txt is not supported by process_file language mapping usually unless we add it
        // and scanner filters extensions anyway.
        // Scanner supports: .ts, .tsx, .js, .jsx, .rs, .py
        // We only have parsers for TS/JS/TSX.
        // If we add .rs file, process_file will fail (get_parser("rust") -> Err), so it should be skipped with warning.
        let rs_file = root.join("main.rs");
        File::create(&rs_file).unwrap(); // Empty file

        let config = AnalysisConfig::default();
        let result = analyze(root, &config).unwrap();

        assert_eq!(result.summary.total_files, 3); // TS, JS, and RS.
        assert_eq!(result.summary.total_loc, 2);
        assert_eq!(result.summary.total_functions, 1);
        
        let file_names: Vec<_> = result.files.iter()
            .map(|f| f.path.file_name().unwrap().to_str().unwrap())
            .collect();
        assert!(file_names.contains(&"test.ts"));
        assert!(file_names.contains(&"index.js"));
        assert!(file_names.contains(&"main.rs"));
    }
}
