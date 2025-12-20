use crate::output::{self, MetricsFormatter};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyzeError {
    #[error("Analysis failed: {0}")]
    AnalysisFailed(#[from] code_viz_core::analyzer::AnalysisError),

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
}

use code_viz_core::traits::{AppContext, FileSystem, GitProvider};

pub fn run(
    config: AnalyzeConfig,
    ctx: impl AppContext + Clone,
    fs: impl FileSystem + Clone,
    git: impl GitProvider,
) -> Result<(), AnalyzeError> {
    let AnalyzeConfig {
        path,
        format,
        exclude: _,
        verbose,
        threshold,
        output,
        baseline,
        dead_code,
    } = config;
    // Setup logging
    let mut builder = env_logger::Builder::from_default_env();
    if verbose {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    let _ = builder.try_init();

    // Use code-viz-commands to run analysis
    let mut result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(code_viz_commands::analyze_repository(&path, ctx.clone(), fs.clone()))
        .map_err(|e| AnalyzeError::DeadCodeFailed(e.to_string()))?;

    // Perform dead code analysis if enabled
    if dead_code {
        log::info!("Running dead code analysis");
        let dead_code_result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(code_viz_commands::calculate_dead_code(&path, ctx, fs.clone(), git))
            .map_err(|e| AnalyzeError::DeadCodeFailed(e.to_string()))?;

        // Merge dead code info into result files
        merge_dead_code_results(&mut result.files, dead_code_result);
    }

    // Handle baseline comparison
    if let Some(baseline_path) = baseline {
        let baseline_content = fs.read_to_string(&baseline_path)
            .map_err(|e| AnalyzeError::IoError(std::io::Error::other(e)))?;
        let baseline: code_viz_core::AnalysisResult = serde_json::from_str(&baseline_content)
            .map_err(|e| AnalyzeError::IoError(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

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
            eprintln!("Error: Total LOC increased by {:.1}% (limit: 10%)", delta_percent);
            process::exit(3);
        }
    }

    // Handle threshold
    if let Some(threshold_str) = threshold {
        check_threshold(&threshold_str, &result.files)?;
    }

    // Format output
    // CLI format arg takes precedence
    let format_str = format.as_str();
    let formatter: Box<dyn MetricsFormatter> = match format_str {
        "json" => Box::new(output::json::JsonFormatter),
        "csv" => Box::new(output::csv::CsvFormatter),
        "text" => Box::new(output::text::TextFormatter),
        _ => Box::new(output::text::TextFormatter),
    };

    let formatted_output = formatter.format(&result)?;

    // Write output
    if let Some(output_path) = output {
        fs.write(&output_path, &formatted_output)
            .map_err(|e| AnalyzeError::IoError(std::io::Error::other(e)))?;
    } else {
        println!("{}", formatted_output);
    }

    Ok(())
}

fn check_threshold(threshold_str: &str, files: &[code_viz_core::FileMetrics]) -> Result<(), AnalyzeError> {
    let parts: Vec<&str> = threshold_str.split('=').collect();
    if parts.len() != 2 {
        return Err(AnalyzeError::InvalidThreshold(threshold_str.to_string()));
    }

    let key = parts[0];

    match key {
        "loc" => {
            let value = parts[1].parse::<usize>().map_err(|_| AnalyzeError::InvalidThreshold(threshold_str.to_string()))?;
            let violating_files: Vec<_> = files.iter()
                .filter(|f| f.loc > value)
                .collect();

            if !violating_files.is_empty() {
                eprintln!("Error: The following files exceed the LOC threshold of {}:", value);
                for file in violating_files {
                    eprintln!("  {} ({} LOC)", file.path.display(), file.loc);
                }
                process::exit(3);
            }
        }
        "dead_code_ratio" => {
            let value = parts[1].parse::<f64>().map_err(|_| AnalyzeError::InvalidThreshold(threshold_str.to_string()))?;
            let violating_files: Vec<_> = files.iter()
                .filter(|f| f.dead_code_ratio.unwrap_or(0.0) > value)
                .collect();

            if !violating_files.is_empty() {
                eprintln!("Error: The following files exceed the dead code ratio threshold of {:.2}:", value);
                for file in violating_files {
                    eprintln!("  {} ({:.2}% dead code)", file.path.display(), file.dead_code_ratio.unwrap_or(0.0) * 100.0);
                }
                process::exit(3);
            }
        }
        _ => return Err(AnalyzeError::InvalidThreshold(format!("Unknown metric '{}'", key))),
    }

    Ok(())
}

fn merge_dead_code_results(
    file_metrics: &mut [code_viz_core::FileMetrics],
    dead_code_result: code_viz_dead_code::DeadCodeResult,
) {
    // Create a map of file -> dead code info for efficient lookup
    let mut dead_code_by_file: HashMap<PathBuf, &code_viz_dead_code::FileDeadCode> =
        HashMap::new();

    for file_dead_code in &dead_code_result.files {
        dead_code_by_file.insert(file_dead_code.path.clone(), file_dead_code);
    }

    // Update file metrics with dead code info
    for file_metric in file_metrics.iter_mut() {
        if let Some(dead_code_info) = dead_code_by_file.get(&file_metric.path) {
            let dead_function_count = dead_code_info.dead_code.iter()
                .filter(|s| matches!(
                    s.kind,
                    code_viz_dead_code::models::SymbolKind::Function
                        | code_viz_dead_code::models::SymbolKind::ArrowFunction
                        | code_viz_dead_code::models::SymbolKind::Method
                ))
                .count();

            let dead_code_loc: usize = dead_code_info.dead_code.iter()
                .map(|s| s.loc)
                .sum();

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
