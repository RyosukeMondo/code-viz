use crate::{churn::calculate_code_churn, shared::scan_and_filter_files};
use anyhow::{Context, Result};
use code_viz_core::{
    analyzer, calculate_summary, coupling, coverage,
    duplication::DuplicationDetector,
    hotspot::HotspotDetector,
    metrics,
    models::{AICommitAnalysis, AnalysisResult, FileMetrics},
    parser,
    parser::LanguageParser,
    traits::{AppContext, FileSystem, GitProvider},
};
use serde_json::json;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Clone, Copy)]
pub struct DuplicationConfig {
    pub min_lines: usize,
    pub similarity_threshold: f64,
}

#[derive(Clone, Copy)]
pub struct HotspotConfig {
    pub max_hotspots: usize,
}

#[derive(Clone)]
pub struct CoverageConfig {
    pub report_path: PathBuf,
}

/// Orchestrate repository analysis using trait-based dependencies.
pub async fn analyze_repository(
    path: &Path,
    ctx: impl AppContext,
    fs: impl FileSystem,
    git: &impl GitProvider,
    duplication_config: Option<DuplicationConfig>,
    hotspot_config: Option<HotspotConfig>,
    coverage_config: Option<CoverageConfig>,
) -> Result<AnalysisResult> {
    ctx.report_progress(0.1, "Scanning directory...").await?;
    let supported_files = scan_and_filter_files(&fs, path)?;

    ctx.report_progress(
        0.2,
        &format!("Found {} files to analyze", supported_files.len()),
    )
    .await?;

    let churn_map = calculate_code_churn(path, &ctx, &fs, git).await?;

    let (results, file_contents) = process_files(
        &supported_files,
        &fs,
        &churn_map,
        &ctx,
        duplication_config.is_some(),
    )
    .await?;

    let mut results = results;
    ctx.report_progress(0.85, "Analyzing dependencies...")
        .await?;
    coupling::calculate_coupling(&mut results, &fs, path);

    let coverage_analysis = process_coverage(&ctx, &fs, coverage_config, &mut results).await?;
    let duplication = process_duplication(&ctx, duplication_config, &file_contents).await?;
    let hotspot_analysis = process_hotspots(&ctx, hotspot_config, &results).await?;

    ctx.report_progress(0.95, "Calculating summary...").await?;
    let summary = calculate_summary(&results);

    let final_result = AnalysisResult {
        summary,
        files: results,
        timestamp: SystemTime::now(),
        duplication,
        ai_commit_analysis: None,
        hotspot_analysis,
        coverage_analysis,
    };

    ctx.emit_event("analysis_complete", json!(final_result))
        .await?;
    ctx.report_progress(1.0, "Analysis complete").await?;

    Ok(final_result)
}

async fn process_files(
    supported_files: &[PathBuf],
    fs: &impl FileSystem,
    churn_map: &HashMap<PathBuf, code_viz_core::models::CodeChurn>,
    ctx: &impl AppContext,
    collect_contents: bool,
) -> Result<(Vec<FileMetrics>, Vec<(PathBuf, String)>)> {
    let mut results = Vec::new();
    let mut file_contents = Vec::new();
    let total_files = supported_files.len();

    for (i, file_path) in supported_files.iter().enumerate() {
        if should_report_progress(i, total_files) {
            let percentage = 0.2 + (i as f32 / total_files as f32) * 0.7;
            ctx.report_progress(
                percentage,
                &format!("Analyzing files ({}/{})", i, total_files),
            )
            .await?;
        }

        if let Ok(source) = fs.read_to_string(file_path) {
            if collect_contents {
                file_contents.push((file_path.clone(), source.clone()));
            }

            if let Ok(metrics) = analyze_file_with_churn(file_path, &source, churn_map).await {
                results.push(metrics);
            }
        }
    }

    Ok((results, file_contents))
}

fn should_report_progress(index: usize, total: usize) -> bool {
    total > 0 && index.is_multiple_of((total / 10).max(1))
}

async fn analyze_file_with_churn(
    file_path: &Path,
    source: &str,
    churn_map: &HashMap<PathBuf, code_viz_core::models::CodeChurn>,
) -> Result<FileMetrics> {
    let mut metrics = analyze_single_file_with_source(file_path, source).await?;
    if let Some(churn) = churn_map.get(file_path) {
        metrics.code_churn = Some(churn.clone());
    }
    Ok(metrics)
}

async fn process_coverage(
    ctx: &impl AppContext,
    fs: &impl FileSystem,
    coverage_config: Option<CoverageConfig>,
    results: &mut [FileMetrics],
) -> Result<Option<code_viz_core::models::CoverageAnalysis>> {
    let Some(coverage_cfg) = coverage_config else {
        return Ok(None);
    };

    ctx.report_progress(0.87, "Loading coverage report...")
        .await?;
    let Ok(coverage_json) = fs.read_to_string(&coverage_cfg.report_path) else {
        return Ok(None);
    };

    let Ok(coverage_map) = coverage::parse_coverage_report(&coverage_json) else {
        return Ok(None);
    };

    coverage::apply_coverage_to_metrics(results, coverage_map);
    Ok(coverage::calculate_coverage_analysis(results))
}

async fn process_duplication(
    ctx: &impl AppContext,
    duplication_config: Option<DuplicationConfig>,
    file_contents: &[(PathBuf, String)],
) -> Result<Option<code_viz_core::models::DuplicationAnalysis>> {
    let Some(config) = duplication_config else {
        return Ok(None);
    };

    ctx.report_progress(0.9, "Running duplication analysis...")
        .await?;
    let detector = DuplicationDetector::new(config.min_lines, config.similarity_threshold);
    let parsers = build_language_parsers();
    Ok(Some(detector.run(file_contents, &parsers)))
}

fn build_language_parsers() -> HashMap<String, Box<dyn LanguageParser>> {
    let mut parsers: HashMap<String, Box<dyn LanguageParser>> = HashMap::new();
    ["ts", "tsx", "js", "jsx", "rs", "py", "go", "cpp"]
        .iter()
        .for_each(|ext| {
            if let Ok(parser) = parser::get_parser(ext) {
                parsers.insert(ext.to_string(), parser);
            }
        });
    parsers
}

async fn process_hotspots(
    ctx: &impl AppContext,
    hotspot_config: Option<HotspotConfig>,
    results: &[FileMetrics],
) -> Result<Option<code_viz_core::models::HotspotAnalysis>> {
    let Some(config) = hotspot_config else {
        return Ok(None);
    };

    ctx.report_progress(0.93, "Calculating hotspots...").await?;
    let detector = HotspotDetector::new(config.max_hotspots);
    Ok(Some(detector.calculate(results)))
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
    ctx.report_progress(0.1, "Analyzing AI commits...").await?;
    let result = analyzer::ai_commit_analyzer::analyze_ai_commits(&git, path).await?;
    ctx.report_progress(1.0, "AI commit analysis complete")
        .await?;
    Ok(result)
}
