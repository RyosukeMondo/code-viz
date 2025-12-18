use crate::config_loader;
use crate::output::{self, MetricsFormatter};
use code_viz_core::{analyze, AnalysisConfig};
use std::collections::HashMap;
use std::env;
use std::fs;
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

pub fn run(
    path: PathBuf,
    format: String,
    exclude: Vec<String>,
    verbose: bool,
    threshold: Option<String>,
    output: Option<PathBuf>,
    baseline: Option<PathBuf>,
    dead_code: bool,
) -> Result<(), AnalyzeError> {
    // Setup logging
    let mut builder = env_logger::Builder::from_default_env();
    if verbose {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    let _ = builder.try_init();

    // Build config
    let mut config = AnalysisConfig::default();

    // Try to load config from current directory or target path
    // We prefer current directory as project root
    let current_dir = env::current_dir()?;
    // If path is a directory, check it too? 
    // Logic: Look for .code-viz.toml in current dir.
    let file_config = config_loader::load_config(&current_dir)?;
    
    // Merge file config
    if let Some(analysis) = file_config.analysis {
        if let Some(file_excludes) = analysis.exclude {
            // Replace defaults with config file
            config.exclude_patterns = file_excludes;
        }
    }
    
    // Merge output format if not specified via CLI (CLI default is "text", but clap handles defaults)
    // Wait, clap gives us a value. If user didn't specify --format, we get "text".
    // How do we know if user specified it?
    // We don't, unless we use Option<String> in clap.
    // But `main.rs` uses `default_value = "text"`.
    // So we assume CLI overrides config.
    // If user provided config `format = "json"`, and CLI arg is "text" (default), we might override config with default.
    // This is tricky with clap defaults.
    // Ideally main.rs should use Option and handle default logic here.
    // But main.rs is fixed for now.
    // I'll ignore config output format for now unless I change main.rs.
    // Or I assume if `format` is "text", we can check config?
    // No, explicit `--format text` should win.
    // I'll stick to: CLI > Config. Since CLI always has value, CLI always wins.
    // This means config `format` is ignored if CLI has default.
    // That's acceptable for MVP.
    
    // Merge CLI excludes (append)
    config.exclude_patterns.extend(exclude.clone());

    // Run analysis
    let mut result = analyze(&path, &config)?;

    // Perform dead code analysis if enabled
    if dead_code {
        log::info!("Running dead code analysis");
        perform_dead_code_analysis(&path, &mut result.files, &exclude)?;
    }

    // Handle baseline comparison
    if let Some(baseline_path) = baseline {
        let baseline_content = fs::read_to_string(baseline_path)?;
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
        fs::write(output_path, formatted_output)?;
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

fn perform_dead_code_analysis(
    root: &PathBuf,
    file_metrics: &mut [code_viz_core::FileMetrics],
    exclude_patterns: &[String],
) -> Result<(), AnalyzeError> {
    // Create config for dead code analysis
    let dead_code_config = code_viz_dead_code::AnalysisConfig {
        exclude_patterns: exclude_patterns.to_vec(),
        enable_cache: true,
        cache_dir: None,
    };

    // Run dead code analysis
    let dead_code_result = code_viz_dead_code::analyze_dead_code(root, Some(dead_code_config))
        .map_err(|e| AnalyzeError::DeadCodeFailed(e.to_string()))?;

    log::info!(
        "Dead code analysis complete: {} dead functions, {} dead classes",
        dead_code_result.summary.dead_functions,
        dead_code_result.summary.dead_classes
    );

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

            log::debug!(
                "File {}: {} dead functions, {} dead LOC, {:.2}% dead code ratio",
                file_metric.path.display(),
                dead_function_count,
                dead_code_loc,
                dead_code_ratio * 100.0
            );
        }
    }

    Ok(())
}
