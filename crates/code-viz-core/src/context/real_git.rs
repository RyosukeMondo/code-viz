use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use crate::traits::{BlameInfo, Commit, Diff, GitProvider};
use crate::traits::git_provider::{ChangedFile, ChangeType};
use git2::{DiffOptions, Repository};
use std::cell::RefCell;
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

type CommitInfo = (String, String, i64);

fn process_blame_hunk(
    repo: &Repository,
    hunk: &git2::BlameHunk,
    commit_cache: &mut HashMap<git2::Oid, CommitInfo>,
    lines: &mut Vec<crate::traits::BlameLine>,
) -> Result<()> {
    let commit_id = hunk.final_commit_id();
    let commit_info = get_or_cache_commit_info(repo, commit_id, commit_cache)?;
    let (author_name, author_email, timestamp) = commit_info;

    for line_num in hunk.final_start_line()..(hunk.final_start_line() + hunk.lines_in_hunk()) {
        lines.push(crate::traits::BlameLine {
            line_number: line_num,
            commit_sha: commit_id.to_string(),
            author: author_name.clone(),
            author_email: author_email.clone(),
            timestamp: *timestamp,
        });
    }
    Ok(())
}

fn get_or_cache_commit_info<'a>(
    repo: &Repository,
    commit_id: git2::Oid,
    cache: &'a mut HashMap<git2::Oid, CommitInfo>,
) -> Result<&'a CommitInfo> {
    if !cache.contains_key(&commit_id) {
        let commit = repo.find_commit(commit_id)
            .with_context(|| format!("Failed to find commit '{}' for blame hunk", commit_id))?;
        let author = commit.author();
        let info = (
            author.name().unwrap_or("Unknown").to_string(),
            author.email().unwrap_or("Unknown").to_string(),
            commit.time().seconds(),
        );
        cache.insert(commit_id, info);
    }
    cache.get(&commit_id).ok_or_else(|| anyhow!("Commit info not in cache after insertion"))
}

#[async_trait]
impl GitProvider for RealGit {
    async fn get_history(&self, path: &Path) -> Result<Vec<Commit>> {
        let path = path.to_path_buf();
        task::spawn_blocking(move || {
            let repo = Repository::discover(&path)
                .with_context(|| format!("Failed to discover repository at {}", path.display()))?;

            let workdir = repo.workdir().context("Repository is bare")?;
            let relative_path = path.strip_prefix(workdir)
                .with_context(|| "File path is not inside the repository workdir")?;

            let mut revwalk = repo.revwalk().context("Failed to create revwalk")?;
            revwalk.push_head().context("Failed to push HEAD to revwalk")?;
            revwalk.set_sorting(git2::Sort::TIME)?;

            let mut commits = Vec::new();

            for oid_result in revwalk {
                let oid = oid_result.context("Failed to get commit ID")?;
                let commit = repo.find_commit(oid).context("Failed to find commit")?;
                let tree = commit.tree().context("Failed to get commit tree")?;

                let mut path_changed = false;
                if let Some(parent) = commit.parents().next() {
                    let parent_tree = parent.tree().context("Failed to get parent tree")?;
                    let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)
                        .context("Failed to create diff")?;

                    for delta in diff.deltas() {
                        if let Some(p) = delta.new_file().path() {
                            if p == relative_path {
                                path_changed = true;
                                break;
                            }
                        }
                    }
                } else {
                    // Initial commit, check if the file exists in the tree
                    if tree.get_path(relative_path).is_ok() {
                        path_changed = true;
                    }
                }

                if path_changed {
                    commits.push(Commit {
                        sha: commit.id().to_string(),
                        author: commit.author().name().unwrap_or("Unknown").to_string(),
                        timestamp: commit.time().seconds(),
                        message: commit.message().unwrap_or("").to_string(),
                    });
                }
            }
            Ok(commits)
        })
        .await
        .map_err(|e| anyhow!("Blocking task failed: {}", e))?
    }

    async fn get_diff(&self, path: &Path, from: Option<&str>, to: &str) -> Result<Diff> {
        let path_buf = path.to_path_buf();
        let from_str = from.map(|s| s.to_string());
        let to_str = to.to_string();

        task::spawn_blocking(move || {
            let repo = Repository::discover(&path_buf)
                .with_context(|| format!("Failed to discover repository at {}", path_buf.display()))?;

            let to_obj = repo.revparse_single(&to_str)
                .with_context(|| format!("Failed to find 'to' revision: {}", to_str))?;
            let to_tree = to_obj.peel_to_tree()
                .with_context(|| format!("Failed to peel 'to' revision to tree: {}", to_str))?;

            let mut diff_opts = git2::DiffOptions::new();
            let diff = if let Some(from_str) = from_str {
                let from_obj = repo.revparse_single(&from_str)
                    .with_context(|| format!("Failed to find 'from' revision: {}", from_str))?;
                let from_tree = from_obj.peel_to_tree()
                    .with_context(|| format!("Failed to peel 'from' revision to tree: {}", from_str))?;
                repo.diff_tree_to_tree(Some(&from_tree), Some(&to_tree), Some(&mut diff_opts))
            } else {
                repo.diff_tree_to_workdir_with_index(Some(&to_tree), Some(&mut diff_opts))
            }
            .context("Failed to create diff")?;

            
            let added_lines = RefCell::new(Vec::<String>::new());
            let deleted_lines = RefCell::new(Vec::<String>::new());
            let modified_lines = RefCell::new(Vec::new());
            let hunk_added = RefCell::new(Vec::<String>::new());
            let hunk_deleted = RefCell::new(Vec::<String>::new());

            diff.foreach(
                &mut |_, _| true, // file_cb
                None,             // binary_cb
                Some(&mut |_, _| { // hunk_cb
                    let deleted = hunk_deleted.borrow();
                    let added = hunk_added.borrow();
                    let mut del_iter = deleted.iter().peekable();
                    let mut add_iter = added.iter().peekable();

                    while let (Some(del), Some(add)) = (del_iter.peek(), add_iter.peek()) {
                        modified_lines.borrow_mut().push(((*del).clone(), (*add).clone()));
                        del_iter.next();
                        add_iter.next();
                    }

                    added_lines.borrow_mut().extend(add_iter.cloned());
                    deleted_lines.borrow_mut().extend(del_iter.cloned());

                    hunk_added.borrow_mut().clear();
                    hunk_deleted.borrow_mut().clear();
                    true
                }),
                Some(&mut |_, _, line| { // line_cb
                    let content = String::from_utf8_lossy(line.content()).to_string();
                    match line.origin() {
                        '+' => hunk_added.borrow_mut().push(content),
                        '-' => hunk_deleted.borrow_mut().push(content),
                        _ => {}
                    }
                    true
                }),
            )
            .context("Failed to process diff")?;

            Ok(Diff {
                added_lines: added_lines.into_inner(),
                deleted_lines: deleted_lines.into_inner(),
                modified_lines: modified_lines.into_inner(),
            })
        })
        .await
        .map_err(|e| anyhow!("Blocking task for diff failed: {}", e))?
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
                process_blame_hunk(&repo, &hunk, &mut commit_cache, &mut lines)?;
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

    async fn get_file_content_at_revision(&self, file_path: &Path, sha: &str) -> Result<String> {
        let file_path = file_path.to_path_buf();
        let sha = sha.to_string();

        task::spawn_blocking(move || {
            let repo = Repository::discover(&file_path)
                .with_context(|| format!("Failed to discover repository at {}", file_path.display()))?;

            let workdir = repo.workdir().context("Repository is bare")?;
            let relative_path = file_path.strip_prefix(workdir)
                .with_context(|| "File path is not inside the repository workdir")?;

            let oid = git2::Oid::from_str(&sha)?;
            let commit = repo.find_commit(oid)?;

            let tree = commit.tree()?;
            let entry = tree.get_path(relative_path)?;

            let object = entry.to_object(&repo)?;
            let blob = object.as_blob().ok_or_else(|| anyhow!("Object is not a blob"))?;

            Ok(String::from_utf8_lossy(blob.content()).to_string())
        })
        .await
        .map_err(|e| anyhow!("Blocking task failed: {}", e))?
    }

    async fn get_changed_files(&self, base: &str, head: &str) -> Result<Vec<ChangedFile>> {
        let base = base.to_string();
        let head = head.to_string();

        task::spawn_blocking(move || {
            let repo = Repository::open(".")?;
            let base_oid = repo.revparse_single(&base)?.id();
            let head_oid = repo.revparse_single(&head)?.id();

            let base_commit = repo.find_commit(base_oid)?;
            let head_commit = repo.find_commit(head_oid)?;

            let base_tree = base_commit.tree()?;
            let head_tree = head_commit.tree()?;

            let mut diff_opts = DiffOptions::new();
            let diff = repo.diff_tree_to_tree(Some(&base_tree), Some(&head_tree), Some(&mut diff_opts))?;

            let mut changed_files = Vec::new();
            diff.foreach(
                &mut |delta, _| {
                    if let Some(path) = delta.new_file().path() {
                        let change_type = match delta.status() {
                            git2::Delta::Added => ChangeType::Added,
                            git2::Delta::Deleted => ChangeType::Deleted,
                            git2::Delta::Modified => ChangeType::Modified,
                            git2::Delta::Renamed => ChangeType::Renamed,
                            _ => return true, // Continue
                        };
                        changed_files.push(ChangedFile {
                            path: path.to_path_buf(),
                            change_type,
                        });
                    }
                    true
                },
                None,
                None,
                None,
            )?;

            Ok(changed_files)
        })
        .await
        .map_err(|e| anyhow!("Blocking task failed: {}", e))?
    }
}
