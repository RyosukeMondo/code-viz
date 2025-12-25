use code_viz_core::context::RealGit;
use code_viz_core::mocks::MockGit;
use code_viz_core::traits::{BlameInfo, BlameLine, GitProvider};
use git2::{Commit, Oid, Repository, Signature, Tree};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[tokio::test]
async fn test_get_blame_with_mock_git_success() {
    let file_path = PathBuf::from("src/main.rs");
    let blame_info = BlameInfo {
        file_path: file_path.clone(),
        lines: vec![
            BlameLine {
                line_number: 1,
                commit_sha: "c1".to_string(),
                author: "Author 1".to_string(),
                author_email: "author1@example.com".to_string(),
                timestamp: 1620000000,
            },
            BlameLine {
                line_number: 2,
                commit_sha: "c2".to_string(),
                author: "Author 2".to_string(),
                author_email: "author2@example.com".to_string(),
                timestamp: 1620000010,
            },
        ],
    };

    let git_provider = MockGit::new().with_blame(blame_info.clone());

    let result = git_provider.get_blame(&file_path).await;
    assert!(result.is_ok());

    let fetched_blame_info = result.unwrap();
    assert_eq!(fetched_blame_info.file_path, file_path);
    assert_eq!(fetched_blame_info.lines.len(), 2);
    assert_eq!(fetched_blame_info.lines[0], blame_info.lines[0]);
    assert_eq!(fetched_blame_info.lines[1], blame_info.lines[1]);
}

#[tokio::test]
async fn test_get_blame_with_mock_git_file_not_found() {
    let file_path = PathBuf::from("src/main.rs");
    let other_file_path = PathBuf::from("src/other.rs");
    let blame_info = BlameInfo {
        file_path: other_file_path,
        lines: vec![],
    };

    let git_provider = MockGit::new().with_blame(blame_info);

    let result = git_provider.get_blame(&file_path).await;
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap().to_string(),
        "No mock blame data found for file 'src/main.rs'"
    );
}

// Helper to commit a file to a repo with a workdir.
fn commit_file<'a>(
    repo: &'a Repository,
    file_name: &str,
    content: &str,
    message: &str,
) -> (Oid, Commit<'a>) {
    let full_path = repo.workdir().unwrap().join(file_name);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&full_path, content).unwrap();

    let mut index = repo.index().unwrap();
    index
        .add_path(std::path::Path::new(file_name))
        .unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    commit_tree(repo, &tree, message)
}

// Helper to commit a file to a bare repo.
fn commit_to_bare<'a>(
    repo: &'a Repository,
    tree: Option<&Tree>,
    file_name: &str,
    content: &str,
    message: &str,
) -> (Oid, Commit<'a>) {
    let blob_id = repo.blob(content.as_bytes()).unwrap();
    let mut builder = match tree {
        Some(t) => repo.treebuilder(Some(t)).unwrap(),
        None => repo.treebuilder(None).unwrap(),
    };
    builder.insert(file_name, blob_id, 0o100644).unwrap();
    let tree_id = builder.write().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    commit_tree(repo, &tree, message)
}

// Generic commit helper.
fn commit_tree<'a>(repo: &'a Repository, tree: &Tree, message: &str) -> (Oid, Commit<'a>) {
    let signature = Signature::now("Test Author", "test@example.com").unwrap();
    let head = repo.head().ok();
    let parent_commit = head
        .as_ref()
        .and_then(|h| h.target())
        .and_then(|oid| repo.find_commit(oid).ok());
    let parents: Vec<&Commit> = parent_commit.iter().collect();
    let oid = repo
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            tree,
            &parents,
        )
        .unwrap();
    (oid, repo.find_commit(oid).unwrap())
}

#[tokio::test]
async fn test_get_blame_with_real_git_success() {
    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    let repo = Repository::init(repo_path).unwrap();

    // Commit 1
    let (c1_oid, _) = commit_file(&repo, "test.txt", "hello\n", "Initial commit");

    // Commit 2
    let (c2_oid, _) = commit_file(&repo, "test.txt", "hello\nworld\n", "Add world");

    let file_to_blame = repo_path.join("test.txt");

    let git_provider = RealGit::new();
    let result = git_provider.get_blame(&file_to_blame).await;

    assert!(result.is_ok(), "get_blame failed: {:?}", result.err());

    let blame_info = result.unwrap();
    assert_eq!(blame_info.file_path, file_to_blame);
    assert_eq!(blame_info.lines.len(), 2);

    let line1 = &blame_info.lines[0];
    assert_eq!(line1.line_number, 1);
    assert_eq!(line1.commit_sha, c1_oid.to_string());
    assert_eq!(line1.author, "Test Author");

    let line2 = &blame_info.lines[1];
    assert_eq!(line2.line_number, 2);
    assert_eq!(line2.commit_sha, c2_oid.to_string());
    assert_eq!(line2.author, "Test Author");
}

#[tokio::test]
async fn test_get_blame_with_real_git_bare_repo_success() {
    let dir = tempdir().unwrap();
    let repo_path = dir.path();
    let repo = Repository::init_bare(repo_path).unwrap();

    // Commit 1
    let (c1_oid, c1) = commit_to_bare(&repo, None, "test.txt", "hello\n", "Initial commit");

    // Commit 2
    let (c2_oid, _) = commit_to_bare(
        &repo,
        Some(&c1.tree().unwrap()),
        "test.txt",
        "hello\nworld\n",
        "Add world",
    );

    let file_to_blame = repo_path.join("test.txt");

    let git_provider = RealGit::new();
    let result = git_provider.get_blame(&file_to_blame).await;

    assert!(
        result.is_ok(),
        "get_blame for bare repo failed: {:?}",
        result.err()
    );

    let blame_info = result.unwrap();
    assert_eq!(blame_info.file_path, file_to_blame);
    assert_eq!(blame_info.lines.len(), 2);

    let line1 = &blame_info.lines[0];
    assert_eq!(line1.line_number, 1);
    assert_eq!(line1.commit_sha, c1_oid.to_string());
    assert_eq!(line1.author, "Test Author");

    let line2 = &blame_info.lines[1];
    assert_eq!(line2.line_number, 2);
    assert_eq!(line2.commit_sha, c2_oid.to_string());
    assert_eq!(line2.author, "Test Author");
}
