use crate::output::{self, MetricsFormatter};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyzeError {
    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("Formatting failed: {0}")]
    FormattingFailed(#[from] crate::output::FormatterError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid threshold format: {0}")]
    InvalidThreshold(String),

    #[error("Config error: {0}")]
    ConfigError(#[from] crate::config_loader::ConfigError),

    #[error("Dead code analysis failed: {0}")]
    DeadCodeFailed(String),

    #[error("AI commit analysis failed: {0}")]
    AICommitAnalysisFailed(String),
}

pub struct AnalyzeConfig {
    pub path: PathBuf,
    pub format: String,
    #[allow(dead_code)]
    pub exclude: Vec<String>,
    pub verbose: bool,
    pub threshold: Option<String>,
    pub output: Option<PathBuf>,
    pub baseline: Option<PathBuf>,
    pub dead_code: bool,
    pub duplicates: bool,
    pub min_duplicate_lines: usize,
    pub ai_commits: bool,
    pub hotspots: bool,
    pub max_hotspots: usize,
    pub coverage_report: Option<PathBuf>,
}

use code_viz_commands::analyze::{CoverageConfig, DuplicationConfig, HotspotConfig};
use code_viz_core::traits::{AppContext, FileSystem, GitProvider};

/// Orchestrates the analysis process
struct AnalysisOrchestrator<C, F, G> {
    config: AnalyzeConfig,
    ctx: C,
    fs: F,
    git: G,
}

impl<C, F, G> AnalysisOrchestrator<C, F, G>
where
    C: AppContext + Clone,
    F: FileSystem + Clone,
    G: GitProvider + Clone,
{
    fn new(config: AnalyzeConfig, ctx: C, fs: F, git: G) -> Self {
        Self {
            config,
            ctx,
            fs,
            git,
        }
    }

    async fn execute(&self) -> Result<(), AnalyzeError> {
        setup_logging(self.config.verbose);

        let mut result = self.run_base_analysis().await?;
        self.enrich_with_dead_code(&mut result).await?;
        self.enrich_with_ai_commits(&mut result).await?;
        self.apply_baseline(&result)?;
        self.check_threshold(&result)?;
        self.output_result(&result)?;

        Ok(())
    }

    async fn run_base_analysis(&self) -> Result<code_viz_core::AnalysisResult, AnalyzeError> {
        let duplication_config = build_duplication_config(&self.config);
        let hotspot_config = build_hotspot_config(&self.config);
        let coverage_config = build_coverage_config(&self.config);

        code_viz_commands::analyze_repository(
            &self.config.path,
            self.ctx.clone(),
            self.fs.clone(),
            &self.git,
            duplication_config,
            hotspot_config,
            coverage_config,
        )
        .await
        .map_err(|e| AnalyzeError::AnalysisFailed(e.to_string()))
    }

    async fn enrich_with_dead_code(
        &self,
        result: &mut code_viz_core::AnalysisResult,
    ) -> Result<(), AnalyzeError> {
        if !self.config.dead_code {
            return Ok(());
        }

        log::info!("Running dead code analysis");
        let dead_code_result = code_viz_commands::calculate_dead_code(
            &self.config.path,
            self.ctx.clone(),
            self.fs.clone(),
            self.git.clone(),
        )
        .await
        .map_err(|e| AnalyzeError::DeadCodeFailed(e.to_string()))?;

        merge_dead_code_results(&mut result.files, dead_code_result);
        Ok(())
    }

    async fn enrich_with_ai_commits(
        &self,
        result: &mut code_viz_core::AnalysisResult,
    ) -> Result<(), AnalyzeError> {
        if !self.config.ai_commits {
            return Ok(());
        }

        log::info!("Running AI commit analysis");
        let ai_commit_result = code_viz_commands::analyze_ai_commits(
            &self.config.path,
            self.ctx.clone(),
            self.git.clone(),
        )
        .await
        .map_err(|e| AnalyzeError::AICommitAnalysisFailed(e.to_string()))?;

        result.ai_commit_analysis = Some(ai_commit_result);
        Ok(())
    }

    fn apply_baseline(&self, result: &code_viz_core::AnalysisResult) -> Result<(), AnalyzeError> {
        let Some(baseline_path) = &self.config.baseline else {
            return Ok(());
        };

        compare_with_baseline(&self.fs, baseline_path, result)
    }

    fn check_threshold(&self, result: &code_viz_core::AnalysisResult) -> Result<(), AnalyzeError> {
        if let Some(threshold_str) = &self.config.threshold {
            check_threshold(threshold_str, &result.files)?;
        }
        Ok(())
    }

    fn output_result(&self, result: &code_viz_core::AnalysisResult) -> Result<(), AnalyzeError> {
        let formatter = create_formatter(&self.config.format);
        let formatted_output = formatter.format(result)?;

        if let Some(output_path) = &self.config.output {
            self.fs
                .write(output_path, &formatted_output)
                .map_err(|e| AnalyzeError::IoError(std::io::Error::other(e)))?;
        } else {
            println!("{}", formatted_output);
        }

        Ok(())
    }
}

pub async fn run(
    config: AnalyzeConfig,
    ctx: impl AppContext + Clone,
    fs: impl FileSystem + Clone,
    git: impl GitProvider,
) -> Result<(), AnalyzeError> {
    let orchestrator = AnalysisOrchestrator::new(config, ctx, fs, git);
    orchestrator.execute().await
}

fn setup_logging(verbose: bool) {
    let mut builder = env_logger::Builder::from_default_env();
    if verbose {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    let _ = builder.try_init();
}

fn build_duplication_config(config: &AnalyzeConfig) -> Option<DuplicationConfig> {
    if config.duplicates {
        Some(DuplicationConfig {
            min_lines: config.min_duplicate_lines,
            similarity_threshold: 0.8,
        })
    } else {
        None
    }
}

fn build_hotspot_config(config: &AnalyzeConfig) -> Option<HotspotConfig> {
    if config.hotspots {
        Some(HotspotConfig {
            max_hotspots: config.max_hotspots,
        })
    } else {
        None
    }
}

fn build_coverage_config(config: &AnalyzeConfig) -> Option<CoverageConfig> {
    config
        .coverage_report
        .as_ref()
        .map(|report_path| CoverageConfig {
            report_path: report_path.clone(),
        })
}

fn create_formatter(format: &str) -> Box<dyn MetricsFormatter> {
    match format {
        "json" => Box::new(output::json::JsonFormatter),
        "csv" => Box::new(output::csv::CsvFormatter),
        "text" => Box::new(output::text::TextFormatter),
        "markdown" => Box::new(output::markdown::MarkdownFormatter),
        _ => Box::new(output::text::TextFormatter),
    }
}

fn compare_with_baseline(
    fs: &impl FileSystem,
    baseline_path: &Path,
    result: &code_viz_core::AnalysisResult,
) -> Result<(), AnalyzeError> {
    let baseline_content = fs
        .read_to_string(baseline_path)
        .map_err(|e| AnalyzeError::IoError(std::io::Error::other(e)))?;
    let baseline: code_viz_core::AnalysisResult =
        serde_json::from_str(&baseline_content).map_err(|e| {
            AnalyzeError::IoError(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        })?;

    let current_loc = result.summary.total_loc;
    let baseline_loc = baseline.summary.total_loc;

    let delta = current_loc as isize - baseline_loc as isize;
    let delta_percent = if baseline_loc > 0 {
        (delta as f64 / baseline_loc as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "Baseline comparison: {} -> {} ({:+.1}%)",
        baseline_loc, current_loc, delta_percent
    );

    if delta_percent > 10.0 {
        eprintln!(
            "Error: Total LOC increased by {:.1}% (limit: 10%)",
            delta_percent
        );
        process::exit(3);
    }

    Ok(())
}

fn check_threshold(
    threshold_str: &str,
    files: &[code_viz_core::FileMetrics],
) -> Result<(), AnalyzeError> {
    let parts: Vec<&str> = threshold_str.split('=').collect();
    if parts.len() != 2 {
        return Err(AnalyzeError::InvalidThreshold(threshold_str.to_string()));
    }

    let key = parts[0];

    match key {
        "loc" => {
            let value = parts[1]
                .parse::<usize>()
                .map_err(|_| AnalyzeError::InvalidThreshold(threshold_str.to_string()))?;
            let violating_files: Vec<_> = files.iter().filter(|f| f.loc > value).collect();

            if !violating_files.is_empty() {
                eprintln!(
                    "Error: The following files exceed the LOC threshold of {}:",
                    value
                );
                for file in violating_files {
                    eprintln!("  {} ({} LOC)", file.path.display(), file.loc);
                }
                process::exit(3);
            }
        }
        "dead_code_ratio" => {
            let value = parts[1]
                .parse::<f64>()
                .map_err(|_| AnalyzeError::InvalidThreshold(threshold_str.to_string()))?;
            let violating_files: Vec<_> = files
                .iter()
                .filter(|f| f.dead_code_ratio.unwrap_or(0.0) > value)
                .collect();

            if !violating_files.is_empty() {
                eprintln!(
                    "Error: The following files exceed the dead code ratio threshold of {:.2}:",
                    value
                );
                for file in violating_files {
                    eprintln!(
                        "  {} ({:.2}% dead code)",
                        file.path.display(),
                        file.dead_code_ratio.unwrap_or(0.0) * 100.0
                    );
                }
                process::exit(3);
            }
        }
        _ => {
            return Err(AnalyzeError::InvalidThreshold(format!(
                "Unknown metric '{}'",
                key
            )))
        }
    }

    Ok(())
}

fn merge_dead_code_results(
    file_metrics: &mut [code_viz_core::FileMetrics],
    dead_code_result: code_viz_dead_code::DeadCodeResult,
) {
    // Create a map of file -> dead code info for efficient lookup
    let mut dead_code_by_file: HashMap<PathBuf, &code_viz_dead_code::FileDeadCode> = HashMap::new();

    for file_dead_code in &dead_code_result.files {
        dead_code_by_file.insert(file_dead_code.path.clone(), file_dead_code);
    }

    // Update file metrics with dead code info
    for file_metric in file_metrics.iter_mut() {
        if let Some(dead_code_info) = dead_code_by_file.get(&file_metric.path) {
            let dead_function_count = dead_code_info
                .dead_code
                .iter()
                .filter(|s| {
                    matches!(
                        s.kind,
                        code_viz_dead_code::models::SymbolKind::Function
                            | code_viz_dead_code::models::SymbolKind::ArrowFunction
                            | code_viz_dead_code::models::SymbolKind::Method
                    )
                })
                .count();

            let dead_code_loc: usize = dead_code_info.dead_code.iter().map(|s| s.loc).sum();

            let dead_code_ratio = if file_metric.loc > 0 {
                dead_code_loc as f64 / file_metric.loc as f64
            } else {
                0.0
            };

            file_metric.dead_function_count = Some(dead_function_count);
            file_metric.dead_code_loc = Some(dead_code_loc);
            file_metric.dead_code_ratio = Some(dead_code_ratio);
        }
    }
}
