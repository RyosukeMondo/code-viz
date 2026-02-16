#![allow(clippy::unwrap_used, clippy::expect_used)]
use std::fs;
use std::path::PathBuf;

mod helpers;
use helpers::{assert_has_duplicates, assert_json_has_fields, assert_summary_stats, CliTest};

fn get_test_repo_path() -> PathBuf {
    use std::sync::OnceLock;
    static TEMP_REPO: OnceLock<PathBuf> = OnceLock::new();
    TEMP_REPO
        .get_or_init(|| {
            let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/test-repo");
            let temp = std::env::temp_dir().join("code-viz-e2e-test-repo");
            // Clean up any previous run
            let _ = fs::remove_dir_all(&temp);
            copy_dir_recursive(&source, &temp);

            // Initialize git repo so the scanner can find files
            use std::process::Command;
            let output = Command::new("git")
                .args(["init"])
                .current_dir(&temp)
                .output()
                .expect("git init failed");
            assert!(
                output.status.success(),
                "Command failed (exit {:?}). stdout: {}\nstderr: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let output = Command::new("git")
                .args(["add", "."])
                .current_dir(&temp)
                .output()
                .expect("git add failed");
            assert!(
                output.status.success(),
                "Command failed (exit {:?}). stdout: {}\nstderr: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let output = Command::new("git")
                .args([
                    "-c",
                    "user.email=test@test.com",
                    "-c",
                    "user.name=Test",
                    "commit",
                    "-m",
                    "init",
                ])
                .current_dir(&temp)
                .output()
                .expect("git commit failed");
            assert!(
                output.status.success(),
                "Command failed (exit {:?}). stdout: {}\nstderr: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            temp
        })
        .clone()
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) {
    fs::create_dir_all(dst).expect("Failed to create dir");
    for entry in fs::read_dir(src).expect("Failed to read dir") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        let dest = dst.join(entry.file_name());
        if path.is_dir() {
            if entry.file_name() == ".git" {
                continue; // Skip .git directories
            }
            copy_dir_recursive(&path, &dest);
        } else {
            fs::copy(&path, &dest).expect("Failed to copy file");
        }
    }
}

fn get_temp_output_path(name: &str) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    temp_dir.join(format!("code-viz-test-{}.json", name))
}

#[test]
fn test_basic_analyze() {
    let cli = CliTest::new();
    let repo_path = get_test_repo_path();

    let output = cli
        .analyze(&repo_path)
        .format("json")
        .expect_success()
        .expect("Failed to run analyze command");

    // Verify basic structure
    assert_json_has_fields(&output, &["summary", "files", "timestamp"]);

    // Verify we analyzed the expected files
    assert_summary_stats(&output, 5, 20); // 5 Rust files, at least 20 LOC
}

#[test]
fn test_analyze_with_duplicates() {
    let cli = CliTest::new();
    let repo_path = get_test_repo_path();

    let output = cli
        .analyze(&repo_path)
        .format("json")
        .duplicates_min_lines(2) // Use low threshold for test
        .expect_success()
        .expect("Failed to run analyze with duplicates");

    // Verify duplication analysis was included
    assert_has_duplicates(&output);
}

#[test]
fn test_analyze_with_hotspots() {
    let cli = CliTest::new();
    let repo_path = get_test_repo_path();

    let output = cli
        .analyze(&repo_path)
        .format("json")
        .hotspots(5)
        .expect_success()
        .expect("Failed to run analyze with hotspots");

    // Verify hotspot analysis field is present (even if empty)
    // Note: May be empty if no git history or low churn
    assert_json_has_fields(&output, &["hotspot_analysis"]);
}

#[test]
fn test_analyze_output_to_file() {
    let cli = CliTest::new();
    let repo_path = get_test_repo_path();
    let output_path = get_temp_output_path("analyze");

    // Clean up if exists
    let _ = fs::remove_file(&output_path);

    cli.analyze(&repo_path)
        .format("json")
        .output(&output_path)
        .expect_success()
        .expect("Failed to write output to file");

    // Verify file was created and contains valid JSON
    let content = fs::read_to_string(&output_path).expect("Failed to read output file");

    assert_json_has_fields(&content, &["summary", "files"]);

    // Clean up
    let _ = fs::remove_file(&output_path);
}

#[test]
fn test_analyze_all_features() {
    let cli = CliTest::new();
    let repo_path = get_test_repo_path();

    let output = cli
        .analyze(&repo_path)
        .format("json")
        .duplicates()
        .hotspots(10)
        .ai_commits()
        .expect_success()
        .expect("Failed to run analyze with all features");

    // Verify all analyses were included
    assert_json_has_fields(
        &output,
        &[
            "summary",
            "files",
            "timestamp",
            "duplication",
            "hotspot_analysis",
            "ai_commit_analysis",
        ],
    );
}

#[test]
#[ignore] // Dead code analysis requires entry points (package.json, main files)
fn test_analyze_with_dead_code() {
    let cli = CliTest::new();
    let repo_path = get_test_repo_path();

    let output = cli
        .analyze(&repo_path)
        .format("json")
        .dead_code()
        .expect_success()
        .expect("Failed to run analyze with dead code");

    // Verify dead code analysis was included
    assert_json_has_fields(&output, &["dead_code_analysis"]);
}

#[test]
fn test_analyze_with_ai_commits() {
    let cli = CliTest::new();
    let repo_path = get_test_repo_path();

    let output = cli
        .analyze(&repo_path)
        .format("json")
        .ai_commits()
        .expect_success()
        .expect("Failed to run analyze with AI commits");

    // Verify AI commit analysis was included
    assert_json_has_fields(&output, &["ai_commit_analysis"]);
}

#[test]
fn test_analyze_validates_basic_metrics() {
    let cli = CliTest::new();
    let repo_path = get_test_repo_path();

    let output = cli
        .analyze(&repo_path)
        .format("json")
        .expect_success()
        .expect("Failed to run basic analyze");

    // Verify all files have basic metrics
    assert_json_has_fields(&output, &["summary", "files", "timestamp"]);

    // Verify we got expected file count and minimum LOC
    assert_summary_stats(&output, 5, 20);
}
