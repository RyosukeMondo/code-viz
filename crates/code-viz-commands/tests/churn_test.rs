
use code_viz_commands::calculate_code_churn;
use code_viz_core::mocks::{MockContext, MockFileSystem, MockGit};
use code_viz_core::models::CodeChurn;
use std::path::{Path, PathBuf};

#[tokio::test]
async fn test_calculate_code_churn_success() {
    // Arrange
    let ctx = MockContext::new();
    let fs = MockFileSystem::new()
        .with_file("src/main.rs", "fn main() {}")
        .with_file("src/lib.rs", "pub fn foo() {}");

    let mut churn_data = std::collections::HashMap::new();
    churn_data.insert(PathBuf::from("src/main.rs"), (1, 1)); // 1 added, 1 deleted
    churn_data.insert(PathBuf::from("src/lib.rs"), (0, 0));  // no changes

    let git = MockGit::new().with_churn_summary(churn_data);

    // Act
    let result = calculate_code_churn(Path::new("src"), &ctx, &fs, &git)
        .await
        .unwrap();

    // Assert
    assert_eq!(result.len(), 2);
    assert_eq!(
        result.get(&PathBuf::from("src/main.rs")).unwrap(),
        &CodeChurn {
            added_lines: 1,
            deleted_lines: 1
        }
    );
    assert_eq!(
        result.get(&PathBuf::from("src/lib.rs")).unwrap(),
        &CodeChurn {
            added_lines: 0,
            deleted_lines: 0
        }
    );
}

#[tokio::test]
async fn test_calculate_code_churn_new_file() {
    // Arrange
    let ctx = MockContext::new();
    let fs = MockFileSystem::new().with_file("src/new.rs", "pub fn bar() {}");

    let mut churn_data = std::collections::HashMap::new();
    churn_data.insert(PathBuf::from("src/new.rs"), (1, 0)); // 1 added, 0 deleted (new file)

    let git = MockGit::new().with_churn_summary(churn_data);

    // Act
    let result = calculate_code_churn(Path::new("src"), &ctx, &fs, &git)
        .await
        .unwrap();

    // Assert
    assert_eq!(result.len(), 1);
    assert_eq!(
        result.get(&PathBuf::from("src/new.rs")).unwrap(),
        &CodeChurn {
            added_lines: 1,
            deleted_lines: 0
        }
    );
}

#[tokio::test]
async fn test_calculate_code_churn_empty_repo() {
    // Arrange
    let ctx = MockContext::new();
    let fs = MockFileSystem::new();
    let git = MockGit::new();

    // Act
    let result = calculate_code_churn(Path::new("src"), &ctx, &fs, &git)
        .await
        .unwrap();

    // Assert
    assert!(result.is_empty());
}
