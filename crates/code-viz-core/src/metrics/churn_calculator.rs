//! Code Churn Calculation Module
//!
//! Calculates code churn metrics using git history.
//! Tracks lines added and deleted per file.

use crate::models::CodeChurn;
use crate::traits::GitProvider;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChurnError {
    #[error("Git error: {0}")]
    GitError(String),
}

/// Calculate code churn summary for repository
///
/// # Arguments
/// * `git_provider` - Git provider implementing GitProvider trait
/// * `path` - Repository path
///
/// # Returns
/// Map of file paths to code churn metrics (added/deleted lines)
pub async fn calculate_churn_summary(
    git_provider: &impl GitProvider,
    path: &Path,
) -> Result<HashMap<PathBuf, CodeChurn>, ChurnError> {
    let summary = git_provider
        .get_churn_summary(path, Some("HEAD~1"), "HEAD")
        .await
        .map_err(|e| {
            tracing::warn!("Could not get churn summary: {}", e);
            ChurnError::GitError(e.to_string())
        })?;

    let result = summary
        .into_iter()
        .map(|(path, (added, deleted))| {
            (
                path,
                CodeChurn {
                    added_lines: added,
                    deleted_lines: deleted,
                },
            )
        })
        .collect();

    Ok(result)
}
