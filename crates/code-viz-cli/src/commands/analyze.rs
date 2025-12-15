use crate::config_loader;
use crate::output::{self, MetricsFormatter};
use code_viz_core::{analyze, AnalysisConfig};
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
}

pub fn run(
    path: PathBuf,
    format: String,
    exclude: Vec<String>,
    verbose: bool,
    threshold: Option<String>,
    output: Option<PathBuf>,
    baseline: Option<PathBuf>,
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
    config.exclude_patterns.extend(exclude);

    // Run analysis
    let result = analyze(&path, &config)?;

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
    let value = parts[1].parse::<usize>().map_err(|_| AnalyzeError::InvalidThreshold(threshold_str.to_string()))?;

    match key {
        "loc" => {
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
        _ => return Err(AnalyzeError::InvalidThreshold(format!("Unknown metric '{}'", key))),
    }

    Ok(())
}
