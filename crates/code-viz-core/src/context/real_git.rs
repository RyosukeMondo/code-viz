use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use crate::traits::{Commit, Diff, BlameInfo, GitProvider};
use git2::Repository;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
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

    async fn get_blame(&self, file_path: &Path) -> Result<BlameInfo> {
        let file_path_buf = file_path.to_path_buf();
        task::spawn_blocking(move || {
            let discovery_path = file_path_buf.parent().unwrap_or(&file_path_buf);
            let repo = Repository::discover(discovery_path).with_context(|| {
                format!(
                    "Failed to discover git repository near path '{}'",
                    discovery_path.display()
                )
            })?;

            let repo_root = if repo.is_bare() {
                // For a bare repo, its path is the root. We assume file_path is relative to this.
                repo.path().to_path_buf()
            } else {
                // For a non-bare repo, the workdir is the root.
                repo.workdir().map(|p| p.to_path_buf()).ok_or_else(|| {
                    anyhow!("Could not find repository workdir for non-bare repo")
                })?
            };

            let relative_path = file_path_buf
                .strip_prefix(&repo_root)
                .with_context(|| {
                    format!(
                        "File '{}' is not inside the git repository root '{}'",
                        file_path_buf.display(),
                        repo_root.display()
                    )
                })?;

            let mut blame_opts = git2::BlameOptions::new();
            blame_opts.track_copies_same_file(true);

            let blame = repo
                .blame_file(relative_path, Some(&mut blame_opts))
                .with_context(|| format!("Failed to blame file '{}'", file_path_buf.display()))?;

            let mut lines = Vec::new();
            let mut commit_cache = std::collections::HashMap::new();

            for hunk in blame.iter() {
                let commit_id = hunk.final_commit_id();

                let commit_info = match commit_cache.entry(commit_id) {
                    std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        let commit = repo.find_commit(commit_id).with_context(|| {
                            format!("Failed to find commit '{}' for blame hunk", commit_id)
                        })?;
                        let author = commit.author();
                        let info = (
                            author.name().unwrap_or("Unknown").to_string(),
                            author.email().unwrap_or("Unknown").to_string(),
                            commit.time().seconds(),
                        );
                        entry.insert(info)
                    }
                };
                let (author_name, author_email, timestamp) = commit_info;

                for line_num in
                    hunk.final_start_line()..(hunk.final_start_line() + hunk.lines_in_hunk())
                {
                    lines.push(crate::traits::BlameLine {
                        line_number: line_num,
                        commit_sha: commit_id.to_string(),
                        author: author_name.clone(),
                        author_email: author_email.clone(),
                        timestamp: *timestamp,
                    });
                }
            }

            Ok(BlameInfo {
                file_path: file_path_buf,
                lines,
            })
        })
        .await
        .map_err(|e| anyhow!("Blocking task for git blame failed: {}", e))?
    }

    async fn get_churn_summary(
        &self,
        _path: &Path,
        _from: Option<&str>,
        _to: &str,
    ) -> Result<HashMap<PathBuf, (usize, usize)>> {
        // TODO: Implement actual churn calculation using git2
        // For now, return empty map (no churn data)
        Ok(HashMap::new())
    }
}
