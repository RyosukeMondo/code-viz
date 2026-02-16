use crate::shared::scan_and_filter_files;
use anyhow::Result;
use code_viz_core::metrics;
use code_viz_core::models::CodeChurn;
use code_viz_core::traits::{AppContext, FileSystem, GitProvider};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Orchestrate code churn calculation using trait-based dependencies.
pub async fn calculate_code_churn(
    path: &Path,
    ctx: &impl AppContext,
    fs: &impl FileSystem,
    git: &impl GitProvider,
) -> Result<HashMap<PathBuf, CodeChurn>> {
    ctx.report_progress(0.1, "Scanning directory for churn analysis...")
        .await?;

    // 1. Scan and filter files
    let supported_files = scan_and_filter_files(fs, path)?;

    let total_files = supported_files.len();
    ctx.report_progress(
        0.2,
        &format!("Found {} files to analyze for churn", total_files),
    )
    .await?;

    // 3. Calculate churn for entire repository
    ctx.report_progress(0.3, "Calculating code churn...")
        .await?;

    let churn_map = metrics::calculate_churn_summary(git, path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to calculate churn: {}", e))?;

    ctx.report_progress(
        0.9,
        &format!("Churn calculated for {} files", churn_map.len()),
    )
    .await?;

    Ok(churn_map)
}
