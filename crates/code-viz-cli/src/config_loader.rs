use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config load failed")]
    LoadFailed,
}

pub struct ConfigFile;

pub fn load_config(_project_root: &Path) -> Result<ConfigFile, ConfigError> {
    todo!("Implement config loader")
}
