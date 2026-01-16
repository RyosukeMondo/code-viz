use std::path::PathBuf;
use std::process;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeadCodeError {
    #[error("Dead code analysis failed: {0}")]
    AnalysisFailed(#[from] code_viz_dead_code::AnalysisError),

    #[error("Formatting failed: {0}")]
    FormattingFailed(#[from] crate::output::FormatterError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid threshold format: {0}")]
    InvalidThreshold(String),
}

use code_viz_core::traits::{AppContext, FileSystem, GitProvider};

pub struct DeadCodeConfig {
    pub path: PathBuf,
    pub format: String,
    pub min_confidence: u8,
    pub verbose: bool,
    pub threshold: Option<String>,
    pub output: Option<PathBuf>,
}

pub async fn run(
    config: DeadCodeConfig,
    ctx: impl AppContext,
    fs: impl FileSystem + Clone,
    git: impl GitProvider,
) -> Result<(), DeadCodeError> {
    let DeadCodeConfig {
        path,
        format,
        min_confidence,
        verbose,
        threshold,
        output,
    } = config;
    // Setup logging
    let mut builder = env_logger::Builder::from_default_env();
    if verbose {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    let _ = builder.try_init();

    // Use code-viz-commands to run dead code analysis
    let result = code_viz_commands::calculate_dead_code(&path, ctx, fs.clone(), git)
        .await
        .map_err(|e| DeadCodeError::IoError(std::io::Error::other(e)))?;

    // Filter by minimum confidence
    let filtered_result = if min_confidence > 0 {
        result.filter_by_confidence(min_confidence)
    } else {
        result
    };

    // Handle threshold
    if let Some(threshold_str) = threshold {
        check_threshold(&threshold_str, &filtered_result)?;
    }

    // Format output
    let formatted_output = match format.as_str() {
        "json" => format_json(&filtered_result)?,
        _ => format_text(&filtered_result), // Default to text
    };

    // Write output
    if let Some(output_path) = output {
        fs.write(&output_path, &formatted_output)
            .map_err(|e| DeadCodeError::IoError(std::io::Error::other(e)))?;
    } else {
        println!("{}", formatted_output);
    }

    Ok(())
}

fn check_threshold(
    threshold_str: &str,
    result: &code_viz_dead_code::DeadCodeResult,
) -> Result<(), DeadCodeError> {
    let parts: Vec<&str> = threshold_str.split('=').collect();
    if parts.len() != 2 {
        return Err(DeadCodeError::InvalidThreshold(threshold_str.to_string()));
    }

    let key = parts[0];
    let value_str = parts[1];

    match key {
        "dead_code_ratio" => {
            let threshold: f64 = value_str
                .parse()
                .map_err(|_| DeadCodeError::InvalidThreshold(threshold_str.to_string()))?;

            if result.summary.dead_code_ratio > threshold {
                eprintln!(
                    "Error: Dead code ratio {:.2}% exceeds threshold {:.2}%",
                    result.summary.dead_code_ratio * 100.0,
                    threshold * 100.0
                );
                process::exit(3);
            }
        }
        "dead_functions" => {
            let threshold: usize = value_str
                .parse()
                .map_err(|_| DeadCodeError::InvalidThreshold(threshold_str.to_string()))?;

            if result.summary.dead_functions > threshold {
                eprintln!(
                    "Error: Dead functions {} exceeds threshold {}",
                    result.summary.dead_functions, threshold
                );
                process::exit(3);
            }
        }
        _ => {
            return Err(DeadCodeError::InvalidThreshold(format!(
                "Unknown metric '{}'",
                key
            )))
        }
    }

    Ok(())
}

fn format_json(result: &code_viz_dead_code::DeadCodeResult) -> Result<String, DeadCodeError> {
    serde_json::to_string_pretty(result)
        .map_err(|e| DeadCodeError::IoError(std::io::Error::other(e)))
}

fn format_text(result: &code_viz_dead_code::DeadCodeResult) -> String {
    use std::fmt::Write;

    let mut output = String::new();

    // Summary section
    // Note: Writing to String never fails, so these write operations are safe to unwrap
    // or we can use expect with clear messages. Using let _ = is cleaner for infallible operations.
    let _ = writeln!(&mut output, "Dead Code Analysis Summary");
    let _ = writeln!(&mut output, "===========================");
    let _ = writeln!(&mut output);
    let _ = writeln!(
        &mut output,
        "Total files analyzed:       {}",
        result.summary.total_files
    );
    let _ = writeln!(
        &mut output,
        "Files with dead code:       {}",
        result.summary.files_with_dead_code
    );
    let _ = writeln!(
        &mut output,
        "Dead functions:             {}",
        result.summary.dead_functions
    );
    let _ = writeln!(
        &mut output,
        "Dead classes:               {}",
        result.summary.dead_classes
    );
    let _ = writeln!(
        &mut output,
        "Total dead LOC:             {}",
        result.summary.total_dead_loc
    );
    let _ = writeln!(
        &mut output,
        "Dead code ratio:            {:.2}%",
        result.summary.dead_code_ratio * 100.0
    );
    let _ = writeln!(&mut output);

    if result.files.is_empty() {
        let _ = writeln!(&mut output, "No dead code found!");
        return output;
    }

    // Group by confidence tiers
    let mut high_confidence = Vec::new();
    let mut medium_confidence = Vec::new();
    let mut low_confidence = Vec::new();

    for file in &result.files {
        for symbol in &file.dead_code {
            if symbol.confidence >= 80 {
                high_confidence.push((file, symbol));
            } else if symbol.confidence >= 60 {
                medium_confidence.push((file, symbol));
            } else {
                low_confidence.push((file, symbol));
            }
        }
    }

    // High confidence section
    if !high_confidence.is_empty() {
        let _ = writeln!(&mut output, "High Confidence Deletions (>= 80%)");
        let _ = writeln!(&mut output, "-----------------------------------");
        for (file, symbol) in &high_confidence {
            let _ = writeln!(
                &mut output,
                "  {} ({}:{})",
                symbol.symbol,
                file.path.display(),
                symbol.line_start
            );
            let _ = writeln!(
                &mut output,
                "    Kind: {:?}, Lines: {}-{}, Confidence: {}%",
                symbol.kind, symbol.line_start, symbol.line_end, symbol.confidence
            );
        }
        let _ = writeln!(&mut output);
    }

    // Medium confidence section
    if !medium_confidence.is_empty() {
        let _ = writeln!(&mut output, "Medium Confidence (60-79%)");
        let _ = writeln!(&mut output, "--------------------------");
        for (file, symbol) in &medium_confidence {
            let _ = writeln!(
                &mut output,
                "  {} ({}:{})",
                symbol.symbol,
                file.path.display(),
                symbol.line_start
            );
            let _ = writeln!(
                &mut output,
                "    Kind: {:?}, Lines: {}-{}, Confidence: {}%",
                symbol.kind, symbol.line_start, symbol.line_end, symbol.confidence
            );
        }
        let _ = writeln!(&mut output);
    }

    // Low confidence section
    if !low_confidence.is_empty() {
        let _ = writeln!(&mut output, "Low Confidence (< 60%)");
        let _ = writeln!(&mut output, "----------------------");
        for (file, symbol) in &low_confidence {
            let _ = writeln!(
                &mut output,
                "  {} ({}:{})",
                symbol.symbol,
                file.path.display(),
                symbol.line_start
            );
            let _ = writeln!(
                &mut output,
                "    Kind: {:?}, Lines: {}-{}, Confidence: {}%",
                symbol.kind, symbol.line_start, symbol.line_end, symbol.confidence
            );
        }
        let _ = writeln!(&mut output);
    }

    output
}
