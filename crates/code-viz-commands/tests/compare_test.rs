use code_viz_commands::compare::{compare_branches, CompareArgs};
use code_viz_core::mocks::MockGit;
use code_viz_core::traits::git_provider::{ChangedFile, ChangeType};
use std::path::PathBuf;

#[tokio::test]
async fn test_compare_branches_added_file() {
    let git = MockGit::new()
        .with_changed_files(vec![ChangedFile {
            path: PathBuf::from("src/new_file.rs"),
            change_type: ChangeType::Added,
        }])
        .with_file_content("feature", "src/new_file.rs", "fn new() {}");

    let args = CompareArgs {
        spec: "main..feature".to_string(),
        base: Some("main".to_string()),
        head: Some("feature".to_string()),
    };

    let result = compare_branches(&args, &git).await.unwrap();

    assert_eq!(result.files.len(), 1);
    let file_comp = &result.files[0];
    assert!(file_comp.base.is_none());
    assert!(file_comp.head.is_some());
    assert_eq!(file_comp.head.as_ref().unwrap().loc, 1);
}

#[tokio::test]
async fn test_compare_branches_deleted_file() {
    let git = MockGit::new()
        .with_changed_files(vec![ChangedFile {
            path: PathBuf::from("src/old_file.rs"),
            change_type: ChangeType::Deleted,
        }])
        .with_file_content("main", "src/old_file.rs", "fn old() {}");

    let args = CompareArgs {
        spec: "main..feature".to_string(),
        base: Some("main".to_string()),
        head: Some("feature".to_string()),
    };

    let result = compare_branches(&args, &git).await.unwrap();

    assert_eq!(result.files.len(), 1);
    let file_comp = &result.files[0];
    assert!(file_comp.base.is_some());
    assert!(file_comp.head.is_none());
    assert_eq!(file_comp.base.as_ref().unwrap().loc, 1);
}

#[tokio::test]
async fn test_compare_branches_modified_file() {
    let git = MockGit::new()
        .with_changed_files(vec![ChangedFile {
            path: PathBuf::from("src/modified_file.rs"),
            change_type: ChangeType::Modified,
        }])
        .with_file_content("main", "src/modified_file.rs", "fn modified() {}")
        .with_file_content(
            "feature",
            "src/modified_file.rs",
            "fn modified() {}\nfn another() {}",
        );

    let args = CompareArgs {
        spec: "main..feature".to_string(),
        base: Some("main".to_string()),
        head: Some("feature".to_string()),
    };

    let result = compare_branches(&args, &git).await.unwrap();

    assert_eq!(result.files.len(), 1);
    let file_comp = &result.files[0];
    assert!(file_comp.base.is_some());
    assert!(file_comp.head.is_some());
    assert_eq!(file_comp.base.as_ref().unwrap().loc, 1);
    assert_eq!(file_comp.head.as_ref().unwrap().loc, 2);
}
