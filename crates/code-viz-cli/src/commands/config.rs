use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file already exists")]
    FileExists,

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

const TEMPLATE: &str = r#"# CodeViz Configuration

[analysis]
# Glob patterns to exclude from analysis
# exclude = ["node_modules/**", "target/**", "dist/**", ".git/**"]

[output]
# Default output format (text, json, csv)
# format = "text"

[cache]
# Enable caching to speed up re-analysis
# enabled = true
"#;

pub fn run_init() -> Result<(), ConfigError> {
    let path = Path::new(".code-viz.toml");

    if path.exists() {
        return Err(ConfigError::FileExists);
    }

    fs::write(path, TEMPLATE)?;
    println!("Created .code-viz.toml with default configuration");

    Ok(())
}
