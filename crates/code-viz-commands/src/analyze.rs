use crate::churn::calculate_code_churn;
use crate::shared::scan_and_filter_files;
use anyhow::{Context, Result};
use code_viz_core::analyzer;
use code_viz_core::duplication::DuplicationDetector;
use code_viz_core::hotspot::HotspotDetector;
use code_viz_core::models::{AICommitAnalysis, AnalysisResult, FileMetrics};
use code_viz_core::parser::LanguageParser;
use code_viz_core::traits::{AppContext, FileSystem, GitProvider};
use code_viz_core::{calculate_summary, coupling, metrics, parser};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy)]
pub struct DuplicationConfig {
    pub min_lines: usize,
    pub similarity_threshold: f64,
}

#[derive(Clone, Copy)]
pub struct HotspotConfig {
    pub max_hotspots: usize,
}

/// Orchestrate repository analysis using trait-based dependencies.
pub async fn analyze_repository(
    path: &Path,
    ctx: impl AppContext,
    fs: impl FileSystem,
    git: &impl GitProvider,
    duplication_config: Option<DuplicationConfig>,
    hotspot_config: Option<HotspotConfig>,
) -> Result<AnalysisResult> {
    ctx.report_progress(0.1, "Scanning directory...").await?;

    // 1. Scan and filter files
    let supported_files = scan_and_filter_files(&fs, path)?;

    let total_files = supported_files.len();
    ctx.report_progress(0.2, &format!("Found {} files to analyze", total_files))
        .await?;

    // 2. Calculate churn
    let churn_map = calculate_code_churn(path, &ctx, &fs, git).await?;

    // 3. Process files and collect contents for duplication analysis
    let mut results = Vec::new();
    let mut file_contents = Vec::new();
    for (i, file_path) in supported_files.iter().enumerate() {
        if total_files > 0 && i % (total_files / 10).max(1) == 0 {
            let percentage = 0.2 + (i as f32 / total_files as f32) * 0.7;
            ctx.report_progress(
                percentage,
                &format!("Analyzing files ({}/{})", i, total_files),
            )
            .await?;
        }

        match fs.read_to_string(file_path) {
            Ok(source) => {
                // Collect contents if duplication analysis is enabled
                if duplication_config.is_some() {
                    file_contents.push((file_path.clone(), source.clone()));
                }

                // Analyze file metrics
                match analyze_single_file_with_source(file_path, &source).await {
                    Ok(mut metrics) => {
                        // Add churn data if available
                        if let Some(churn) = churn_map.get(file_path) {
                            metrics.code_churn = Some(churn.clone());
                        }
                        results.push(metrics)
                    }
                    Err(e) => tracing::warn!("Failed to analyze {}: {}", file_path.display(), e),
                }
            }
            Err(e) => tracing::warn!("Failed to read {}: {}", file_path.display(), e),
        }
    }

    // 4. Calculate coupling metrics
    ctx.report_progress(0.85, "Analyzing dependencies...").await?;
    coupling::calculate_coupling(&mut results, &fs, path);

    // 5. Run duplication analysis if enabled
    let duplication = if let Some(config) = duplication_config {
        ctx.report_progress(0.9, "Running duplication analysis...")
            .await?;
        let detector = DuplicationDetector::new(config.min_lines, config.similarity_threshold);
        let mut parsers: HashMap<String, Box<dyn LanguageParser>> = HashMap::new();
        ["ts", "tsx", "js", "jsx", "rs", "py", "go", "cpp"]
            .iter()
            .for_each(|ext| {
                if let Ok(parser) = parser::get_parser(ext) {
                    parsers.insert(ext.to_string(), parser);
                }
            });
        let analysis = detector.run(&file_contents, &parsers);
        Some(analysis)
    } else {
        None
    };

    // 6. Calculate hotspot analysis if enabled
    let hotspot_analysis = if let Some(config) = hotspot_config {
        ctx.report_progress(0.93, "Calculating hotspots...")
            .await?;
        let detector = HotspotDetector::new(config.max_hotspots);
        Some(detector.calculate(&results))
    } else {
        None
    };

    // 7. Calculate summary
    ctx.report_progress(0.95, "Calculating summary...").await?;
    let summary = calculate_summary(&results);

    let final_result = AnalysisResult {
        summary,
        files: results,
        timestamp: SystemTime::now(),
        duplication,
        ai_commit_analysis: None,
        hotspot_analysis,
    };

    ctx.emit_event("analysis_complete", json!(final_result))
        .await?;
    ctx.report_progress(1.0, "Analysis complete").await?;

    Ok(final_result)
}

async fn analyze_single_file_with_source(path: &Path, source: &str) -> Result<FileMetrics> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .context("File has no extension")?;

    let language_key = match extension {
        "ts" => "typescript",
        "tsx" => "tsx",
        "js" | "jsx" => "javascript",
        "rs" => "rust",
        "py" => "python",
        "go" => "go",
        "cpp" | "cxx" | "cc" | "hpp" | "h" => "cpp",
        ext => ext,
    };

    let parser = parser::get_parser(language_key)
        .with_context(|| format!("Failed to get parser for language: {}", language_key))?;

    metrics::calculate_metrics(path, source, parser.as_ref(), None)
        .with_context(|| format!("Failed to calculate metrics for: {}", path.display()))
}

pub async fn analyze_ai_commits(
    path: &Path,
    ctx: impl AppContext,
    git: impl GitProvider,
) -> Result<AICommitAnalysis> {
    ctx.report_progress(0.1, "Analyzing AI commits...")
        .await?;
    let result = analyzer::ai_commit_analyzer::analyze_ai_commits(&git, path).await?;
    ctx.report_progress(1.0, "AI commit analysis complete")
        .await?;
    Ok(result)
}
