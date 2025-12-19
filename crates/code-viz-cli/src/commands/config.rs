use std::path::Path;
use thiserror::Error;
use code_viz_core::traits::FileSystem;

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

pub fn run_init(fs: impl FileSystem) -> Result<(), ConfigError> {
    let path = Path::new(".code-viz.toml");

    if fs.exists(path) {
        return Err(ConfigError::FileExists);
    }

    fs.write(path, TEMPLATE)
        .map_err(|e| ConfigError::IoError(std::io::Error::other(e)))?;
    println!("Created .code-viz.toml with default configuration");

    Ok(())
}
