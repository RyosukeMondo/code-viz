use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyzeError {
    #[error("Analysis failed")]
    AnalysisFailed,
}

pub fn run(
    _path: PathBuf,
    _format: String,
    _exclude: Vec<String>,
    _verbose: bool,
    _threshold: Option<String>,
    _output: Option<PathBuf>,
    _baseline: Option<PathBuf>,
) -> Result<(), AnalyzeError> {
    todo!("Implement analyze command")
}
