#![allow(clippy::unwrap_used, clippy::expect_used)]
use anyhow::Result;
use assert_cmd::Command;
use git2::{Commit, Repository, Signature};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn setup_git_repo(path: &Path) -> Result<Repository> {
    let repo = Repository::init(path)?;
    let sig = Signature::now("test", "test@test.com")?;

    let tree_id = {
        let mut index = repo.index()?;
        index.write_tree()?
    };
    let tree = repo.find_tree(tree_id)?;

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;
    drop(tree);
    Ok(repo)
}

fn commit_file<'a>(repo: &'a Repository, path: &Path, content: &str, msg: &str) -> Result<Commit<'a>> {
    let sig = Signature::now("test", "test@test.com")?;
    let mut index = repo.index()?;
    
    let p = path.strip_prefix(repo.workdir().unwrap()).unwrap();
    fs::write(path, content)?;
    index.add_path(p)?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent_commit = repo.head()?.peel_to_commit()?;

    let commit_oid = repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&parent_commit])?;
    let commit = repo.find_commit(commit_oid)?;
    Ok(commit)
}


#[test]
#[allow(deprecated)] // cargo_bin is deprecated but needed for cross-crate binary testing
fn test_timeline_command() -> Result<()> {
    let dir = tempdir()?;
    let repo_path = dir.path();
    let repo = setup_git_repo(repo_path)?;

    let file_path = repo_path.join("test.rs");
    commit_file(&repo, &file_path, "fn main() {}", "First commit")?;
    commit_file(&repo, &file_path, "fn main() {\n    println!(\"hello\");\n}", "Second commit")?;

    let mut cmd = Command::cargo_bin("code-viz-cli")?;
    cmd.arg("timeline")
        .arg(&file_path)
        .current_dir(repo_path);

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("\"loc\": 1"))
        .stdout(predicates::str::contains("\"loc\": 3"));

    Ok(())
}
