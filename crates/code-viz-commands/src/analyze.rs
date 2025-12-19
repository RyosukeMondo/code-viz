use anyhow::{Context, Result};
use code_viz_core::traits::{AppContext, FileSystem};
use code_viz_core::models::{AnalysisResult, FileMetrics};
use code_viz_core::{calculate_summary, parser, metrics};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Orchestrate repository analysis using trait-based dependencies.
pub async fn analyze_repository(
    path: &Path,
    ctx: impl AppContext,
    fs: impl FileSystem,
) -> Result<AnalysisResult> {
    ctx.report_progress(0.1, "Scanning directory...").await?;

    // 1. Scan directory
    let all_files = fs.read_dir_recursive(path)
        .with_context(|| format!("Failed to scan directory: {}", path.display()))?;
    
    // 2. Filter supported files
    let supported_files: Vec<PathBuf> = all_files.into_iter()
        .filter(|p| {
            if let Some(ext) = p.extension() {
                let ext_str = ext.to_string_lossy();
                matches!(ext_str.as_ref(), "ts" | "tsx" | "js" | "jsx" | "rs" | "py" | "go" | "cpp" | "cc" | "cxx" | "hpp" | "h")
            } else {
                false
            }
        })
        .collect();

    let total_files = supported_files.len();
    ctx.report_progress(0.2, &format!("Found {} files to analyze", total_files)).await?;

    // 3. Process files
    let mut results = Vec::new();
    for (i, file_path) in supported_files.iter().enumerate() {
        // Periodic progress reporting
        if total_files > 0 && i % (total_files / 10).max(1) == 0 {
            let percentage = 0.2 + (i as f32 / total_files as f32) * 0.7;
            ctx.report_progress(percentage, &format!("Analyzing files ({}/{})", i, total_files)).await?;
        }

        match analyze_single_file(file_path, &fs).await {
            Ok(metrics) => results.push(metrics),
            Err(e) => {
                // Log error but continue with other files
                // In a real app, we might want to report this to the UI
                tracing::warn!("Failed to analyze {}: {}", file_path.display(), e);
            }
        }
    }

    ctx.report_progress(0.9, "Calculating summary...").await?;

    // 4. Calculate summary
    let summary = calculate_summary(&results);

    let final_result = AnalysisResult {
        summary,
        files: results,
        timestamp: SystemTime::now(),
    };

    // 5. Emit completion event
    ctx.emit_event("analysis_complete", json!(final_result)).await?;
    ctx.report_progress(1.0, "Analysis complete").await?;

    Ok(final_result)
}

/// Analyze a single file using the FileSystem trait.
async fn analyze_single_file(path: &Path, fs: &impl FileSystem) -> Result<FileMetrics> {
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .context("File has no extension")?;

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

    let parser = parser::get_parser(language_key)
        .with_context(|| format!("Failed to get parser for language: {}", language_key))?;

    let source = fs.read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let metrics = metrics::calculate_metrics(path, &source, parser.as_ref())
        .with_context(|| format!("Failed to calculate metrics for: {}", path.display()))?;

    Ok(metrics)
}
