use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crate::traits::{Commit, Diff, BlameInfo, GitProvider};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Mock implementation of GitProvider for unit testing.
/// Provides a configurable commit history and tracks operations.
#[derive(Clone, Default)]
pub struct MockGit {
    commits: Arc<Mutex<Vec<Commit>>>,
    diffs: Arc<Mutex<Vec<(String, Option<String>, String)>>>,
    mock_diffs: Arc<Mutex<HashMap<PathBuf, Diff>>>,
    churn_summary: Arc<Mutex<HashMap<PathBuf, (usize, usize)>>>,
    blames: Arc<Mutex<Vec<BlameInfo>>>,
    diff_calls: Arc<Mutex<Vec<(String, Option<String>, String)>>>,
    mock_diff: Arc<Mutex<Option<Diff>>>,
    file_contents: Arc<Mutex<HashMap<(PathBuf, String), String>>>,
}

impl MockGit {
    /// Create a new MockGit.
    pub fn new() -> Self {
        Self {
            commits: Arc::new(Mutex::new(Vec::new())),
            diffs: Arc::new(Mutex::new(Vec::new())),
            mock_diffs: Arc::new(Mutex::new(HashMap::new())),
            churn_summary: Arc::new(Mutex::new(HashMap::new())),
            blames: Arc::new(Mutex::new(Vec::new())),
            diff_calls: Arc::new(Mutex::new(Vec::new())),
            mock_diff: Arc::new(Mutex::new(None)),
            file_contents: Arc::new(Mutex::new(HashMap::new())),
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

    /// Set the default diff to be returned by get_diff.
    pub fn with_mock_diff(self, diff: Diff) -> Self {
        *self.mock_diff.lock().unwrap() = Some(diff);
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

    /// Add a mock diff for a specific file path.
    pub fn with_diff_for_path(self, path: &str, diff: Diff) -> Self {
        self.mock_diffs
            .lock()
            .unwrap()
            .insert(PathBuf::from(path), diff);
        self
    }

    pub fn with_churn_summary(self, summary: HashMap<PathBuf, (usize, usize)>) -> Self {
        *self.churn_summary.lock().unwrap() = summary;
        self
    }

    /// Add blame information to the mock git provider.
    pub fn with_blame(self, blame: BlameInfo) -> Self {
        self.blames.lock().unwrap().push(blame);
        self
    }

    /// Add mock file content for a specific revision.
    pub fn with_file_content(self, path: &Path, sha: &str, content: &str) -> Self {
        self.file_contents
            .lock()
            .unwrap()
            .insert((path.to_path_buf(), sha.to_string()), content.to_string());
        self
    }
}

#[async_trait]
impl GitProvider for MockGit {
    async fn get_history(&self, _path: &Path) -> Result<Vec<Commit>> {
        Ok(self.commits.lock().unwrap().clone())
    }

    async fn get_diff(&self, path: &Path, from: Option<&str>, to: &str) -> Result<Diff> {
        self.diff_calls.lock().unwrap().push((
            path.display().to_string(),
            from.map(|s| s.to_string()),
            to.to_string(),
        ));

        if let Some(diff) = self.mock_diff.lock().unwrap().clone() {
            Ok(diff)
        } else {
            Ok(Diff {
                added_lines: vec![],
                deleted_lines: vec![],
                modified_lines: vec![],
            })
        }
    }

    async fn get_blame(&self, file_path: &Path) -> Result<BlameInfo> {
        let blames = self.blames.lock().unwrap();
        let file_path_buf = file_path.to_path_buf();

        for blame in blames.iter() {
            if blame.file_path == file_path_buf {
                return Ok(blame.clone());
            }
        }

        Err(anyhow!(
            "No mock blame data found for file '{}'",
            file_path.display()
        ))
    }

    async fn get_churn_summary(
        &self,
        _path: &Path,
        _from: Option<&str>,
        _to: &str,
    ) -> Result<HashMap<PathBuf, (usize, usize)>> {
        Ok(self.churn_summary.lock().unwrap().clone())
    }

    async fn get_file_content_at_revision(&self, file_path: &Path, sha: &str) -> Result<String> {
        let key = (file_path.to_path_buf(), sha.to_string());
        let contents = self.file_contents.lock().unwrap();
        
        contents
            .get(&key)
            .cloned()
            .ok_or_else(|| anyhow!("Mock content not found for {}@{}", file_path.display(), sha))
    }
}
