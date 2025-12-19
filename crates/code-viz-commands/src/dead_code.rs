use anyhow::Result;
use code_viz_core::traits::{AppContext, FileSystem, GitProvider};
use std::path::Path;

/// Orchestrate dead code analysis.
pub async fn calculate_dead_code(
    _path: &Path,
    _ctx: impl AppContext,
    _fs: impl FileSystem,
    _git: impl GitProvider,
) -> Result<()> {
    todo!("Implement calculate_dead_code orchestration")
}
