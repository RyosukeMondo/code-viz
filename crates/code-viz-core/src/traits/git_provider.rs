use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Represents a Git commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub author: String,
    pub timestamp: i64,
    pub message: String,
}

/// Represents a Git diff.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Diff {
    pub added_lines: Vec<String>,
    pub deleted_lines: Vec<String>,
    pub modified_lines: Vec<(String, String)>, // (old_line, new_line)
}

/// Represents Git blame information for a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameInfo {
    pub file_path: PathBuf,
    pub lines: Vec<BlameLine>,
}

/// Represents a single line in a Git blame report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlameLine {
    pub line_number: usize,
    pub commit_sha: String,
    pub author: String,
    pub author_email: String,
    pub timestamp: i64,
}

use std::collections::HashMap;

/// GitProvider abstracts Git operations required for analysis.
#[async_trait]
pub trait GitProvider: Send + Sync + Clone {
    /// Get the commit history for a given repository path.
    async fn get_history(&self, path: &Path) -> Result<Vec<Commit>>;

    /// Get the diff between two commits or for a specific path.
    async fn get_diff(&self, path: &Path, from: Option<&str>, to: &str) -> Result<Diff>;

    /// Get blame information for a specific file.
    async fn get_blame(&self, file_path: &Path) -> Result<BlameInfo>;

    async fn get_churn_summary(
        &self,
        path: &Path,
        from: Option<&str>,
        to: &str,
    ) -> Result<HashMap<PathBuf, (usize, usize)>>;
}
