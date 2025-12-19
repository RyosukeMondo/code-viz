use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use crate::traits::{Commit, Diff, BlameInfo, GitProvider};
use git2::Repository;
use std::path::Path;
use tokio::task;

/// Production implementation of GitProvider that uses the git2 crate.
/// Methods are executed on a blocking thread using tokio::task::spawn_blocking.
#[derive(Clone, Copy)]
pub struct RealGit;

impl RealGit {
    /// Create a new RealGit instance.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl GitProvider for RealGit {
    async fn get_history(&self, path: &Path) -> Result<Vec<Commit>> {
        let repo_path = path.to_path_buf();
        task::spawn_blocking(move || {
            let repo = Repository::open(&repo_path)
                .with_context(|| format!("Failed to open repository at {}", repo_path.display()))?;
            
            let mut revwalk = repo.revwalk()
                .context("Failed to create revwalk")?;
            revwalk.push_head()
                .context("Failed to push HEAD to revwalk")?;

            let mut commits = Vec::new();
            for id in revwalk {
                let id = id.context("Failed to get commit ID")?;
                let commit = repo.find_commit(id)
                    .context("Failed to find commit")?;
                
                commits.push(Commit {
                    sha: commit.id().to_string(),
                    author: commit.author().name().unwrap_or("Unknown").to_string(),
                    timestamp: commit.time().seconds(),
                    message: commit.message().unwrap_or("").to_string(),
                });
            }
            Ok(commits)
        })
        .await
        .map_err(|e| anyhow!("Blocking task failed: {}", e))?
    }

    async fn get_diff(&self, _path: &Path, _from: Option<&str>, _to: &str) -> Result<Diff> {
        // TODO: Implement actual diffing using git2
        Ok(Diff {
            content: "Diff implementation pending".to_string(),
        })
    }

    async fn get_blame(&self, _file_path: &Path) -> Result<BlameInfo> {
        // TODO: Implement actual blame using git2
        Err(anyhow!("Blame implementation pending"))
    }
}
