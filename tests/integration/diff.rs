
use code_viz_core::traits::{Diff, GitProvider};
use code_viz_core::context::RealGit;
use code_viz_core::mocks::MockGit;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;

#[tokio::test]
async fn test_get_diff_with_mock_git() {
    let mock_diff = Diff {
        added_lines: vec!["added".to_string()],
        deleted_lines: vec!["deleted".to_string()],
        modified_lines: vec![("old".to_string(), "new".to_string())],
    };
    let mock_git = MockGit::new().with_diff(mock_diff.clone());

    let diff = mock_git.get_diff(Path::new("."), Some("a"), "b").await.unwrap();

    assert_eq!(diff, mock_diff);
}

#[tokio::test]
async fn test_get_diff_with_real_git() {
    let dir = tempdir().unwrap();
    let path = dir.path();
    let repo = git2::Repository::init(path).unwrap();

    // First commit
    let mut index = repo.index().unwrap();
    let mut config = repo.config().unwrap();
    config.set_str("user.name", "Test").unwrap();
    config.set_str("user.email", "test@example.com").unwrap();
    let signature = repo.signature().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let oid1 = repo.commit(Some("HEAD"), &signature, &signature, "Initial commit", &tree, &[]).unwrap();

    // Second commit
    let file_path = path.join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "hello").unwrap();
    index.add_path(Path::new("test.txt")).unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let parent_commit = repo.find_commit(oid1).unwrap();
    let oid2 = repo.commit(Some("HEAD"), &signature, &signature, "Add test.txt", &tree, &[&parent_commit]).unwrap();
    
    // Third commit
    let mut file = fs::OpenOptions::new().append(true).open(&file_path).unwrap();
    writeln!(file, "world").unwrap();
    index.add_path(Path::new("test.txt")).unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let parent_commit = repo.find_commit(oid2).unwrap();
    repo.commit(Some("HEAD"), &signature, &signature, "Append to test.txt", &tree, &[&parent_commit]).unwrap();

    let git = RealGit::new();
    let diff = git.get_diff(path, Some(&oid1.to_string()), &oid2.to_string()).await.unwrap();

    assert_eq!(diff.added_lines, vec!["hello\n"]);
    assert_eq!(diff.deleted_lines.len(), 0);
    assert_eq!(diff.modified_lines.len(), 0);

    // Third commit (modify and delete)
    let mut file = fs::OpenOptions::new().write(true).truncate(true).open(&file_path).unwrap();
    writeln!(file, "hello mars").unwrap();
    index.add_path(Path::new("test.txt")).unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let parent_commit = repo.find_commit(oid2).unwrap();
    let oid3 = repo.commit(Some("HEAD"), &signature, &signature, "Modify and delete from test.txt", &tree, &[&parent_commit]).unwrap();

    let diff = git.get_diff(path, Some(&oid2.to_string()), &oid3.to_string()).await.unwrap();
    assert_eq!(diff.modified_lines, vec![("hello\n".to_string(), "hello mars\n".to_string())]);
    assert_eq!(diff.deleted_lines, vec!["world\n".to_string()]);
    assert_eq!(diff.added_lines.len(), 0);
}
