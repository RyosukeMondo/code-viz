use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config init failed")]
    InitFailed,
}

pub fn run_init() -> Result<(), ConfigError> {
    todo!("Implement config init command")
}
