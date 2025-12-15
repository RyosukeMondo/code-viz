use serde::Deserialize;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config parse failed: {0}")]
    ParseFailed(#[from] toml::de::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Deserialize, Default)]
pub struct ConfigFile {
    pub analysis: Option<AnalysisConfigSection>,
    pub output: Option<OutputConfigSection>,
    pub cache: Option<CacheConfigSection>,
}

#[derive(Debug, Deserialize)]
pub struct AnalysisConfigSection {
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct OutputConfigSection {
    pub format: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CacheConfigSection {
    pub enabled: Option<bool>,
}

pub fn load_config(project_root: &Path) -> Result<ConfigFile, ConfigError> {
    let config_path = project_root.join(".code-viz.toml");
    
    if !config_path.exists() {
        return Ok(ConfigFile::default());
    }

    let content = fs::read_to_string(&config_path)?;
    let config: ConfigFile = toml::from_str(&content)?;
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::io::Write;
    use std::fs::File;

    #[test]
    fn test_load_valid_config() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        
        let mut f = File::create(root.join(".code-viz.toml")).unwrap();
        writeln!(f, r#"
            [analysis]
            exclude = ["node_modules/**", "dist/**"]

            [output]
            format = "json"
            
            [cache]
            enabled = true
        "#).unwrap();

        let config = load_config(root).unwrap();
        
        assert!(config.analysis.is_some());
        let analysis = config.analysis.unwrap();
        assert_eq!(analysis.exclude.unwrap().len(), 2);
        
        assert!(config.output.is_some());
        assert_eq!(config.output.unwrap().format.unwrap(), "json");
        
        assert!(config.cache.is_some());
        assert_eq!(config.cache.unwrap().enabled.unwrap(), true);
    }

    #[test]
    fn test_load_missing_config() {
        let temp_dir = TempDir::new().unwrap();
        let config = load_config(temp_dir.path()).unwrap();
        assert!(config.analysis.is_none());
    }

    #[test]
    fn test_load_invalid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        
        let mut f = File::create(root.join(".code-viz.toml")).unwrap();
        writeln!(f, "invalid toml [").unwrap();

        let result = load_config(root);
        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ConfigError::ParseFailed(_)));
    }
}
