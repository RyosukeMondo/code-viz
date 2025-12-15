use crate::output::{self, MetricsFormatter};
use code_viz_core::{analyze, AnalysisConfig};
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
}

pub fn run(
    path: PathBuf,
    format: String,
    exclude: Vec<String>,
    verbose: bool,
    threshold: Option<String>,
    output: Option<PathBuf>,
    _baseline: Option<PathBuf>,
) -> Result<(), AnalyzeError> {
    // Setup logging
    let mut builder = env_logger::Builder::from_default_env();
    if verbose {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    let _ = builder.try_init(); // Ignore if already initialized

    // Build config
    let mut config = AnalysisConfig::default();
    config.exclude_patterns.extend(exclude);

    // Run analysis
    let result = analyze(&path, &config)?;

    // Handle threshold
    if let Some(threshold_str) = threshold {
        check_threshold(&threshold_str, &result.files)?;
    }

    // Format output
    let formatter: Box<dyn MetricsFormatter> = match format.as_str() {
        "json" => Box::new(output::json::JsonFormatter),
        "csv" => Box::new(output::csv::CsvFormatter),
        "text" => Box::new(output::text::TextFormatter),
        _ => Box::new(output::text::TextFormatter), // Default to text
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
