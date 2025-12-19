use anyhow::Result;
use code_viz_core::traits::{AppContext, FileSystem, GitProvider};
use code_viz_dead_code::{analyze_dead_code, DeadCodeResult};
use std::path::Path;

/// Orchestrate dead code analysis using trait-based dependencies.
pub async fn calculate_dead_code(
    path: &Path,
    _ctx: impl AppContext,
    _fs: impl FileSystem,
    _git: impl GitProvider,
) -> Result<DeadCodeResult> {
    // Note: code_viz_dead_code currently uses std::fs internally.
    // In a full refactor, we would make it use the FileSystem trait too.
    // For now, we wrap it to satisfy the trait-based command layer.
    
    let result = analyze_dead_code(path, None)
        .map_err(|e| anyhow::anyhow!("Dead code analysis failed: {}", e))?;

    Ok(result)
}