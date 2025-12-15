use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WatchError {
    #[error("Watch failed")]
    WatchFailed,
}

pub fn run(_path: PathBuf, _format: String, _verbose: bool) -> Result<(), WatchError> {
    todo!("Implement watch command")
}
