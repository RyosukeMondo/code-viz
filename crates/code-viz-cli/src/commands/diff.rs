use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiffError {
    #[error("Diff failed")]
    DiffFailed,
}

pub fn run(_old: PathBuf, _new: PathBuf) -> Result<(), DiffError> {
    todo!("Implement diff command")
}
