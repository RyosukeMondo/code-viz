use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crate::traits::{Commit, Diff, BlameInfo, GitProvider};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Mock implementation of GitProvider for unit testing.
/// Provides a configurable commit history and tracks operations.
#[derive(Clone, Default)]
pub struct MockGit {
    commits: Arc<Mutex<Vec<Commit>>>,
    diffs: Arc<Mutex<Vec<(String, Option<String>, String)>>>,
}

impl MockGit {
    /// Create a new MockGit.
    pub fn new() -> Self {
        Self {
            commits: Arc::new(Mutex::new(Vec::new())),
            diffs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a commit to the mock history.
    pub fn with_commit(self, commit: Commit) -> Self {
        self.commits.lock().unwrap().push(commit);
        self
    }

    /// Add multiple commits to the mock history.
    pub fn with_commits(self, commits: Vec<Commit>) -> Self {
        self.commits.lock().unwrap().extend(commits);
        self
    }

    /// Helper to add a commit using simple parameters.
    pub fn add_commit(self, sha: &str, author: &str, message: &str) -> Self {
        self.with_commit(Commit {
            sha: sha.to_string(),
            author: author.to_string(),
            timestamp: 0,
            message: message.to_string(),
        })
    }
}

#[async_trait]
impl GitProvider for MockGit {
    async fn get_history(&self, _path: &Path) -> Result<Vec<Commit>> {
        Ok(self.commits.lock().unwrap().clone())
    }

    async fn get_diff(&self, path: &Path, from: Option<&str>, to: &str) -> Result<Diff> {
        self.diffs.lock().unwrap().push((
            path.display().to_string(),
            from.map(|s| s.to_string()),
            to.to_string()
        ));
        Ok(Diff {
            content: "Mock diff content".to_string(),
        })
    }

    async fn get_blame(&self, _file_path: &Path) -> Result<BlameInfo> {
        Err(anyhow!("Mock blame not implemented"))
    }
}
