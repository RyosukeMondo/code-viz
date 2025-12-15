use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_e2e_analyze_json_output() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("src/main.ts")
        .write_str("function main() { console.log('hello'); }")
        .unwrap();
    temp.child("src/utils.ts")
        .write_str("export const x = 1;")
        .unwrap();

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("analyze")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"total_files\": 2"))
        .stdout(predicate::str::contains("\"total_loc\": 2"));
}

#[test]
fn test_e2e_analyze_text_output() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("src/main.ts")
        .write_str("function main() { console.log('hello'); }")
        .unwrap();

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("analyze")
        .arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Total Files: 1"))
        .stdout(predicate::str::contains("Total LOC:   1"));
}

#[test]
fn test_e2e_threshold_violation() {
    let temp = assert_fs::TempDir::new().unwrap();
    // Create a large file
    let content = "x\n".repeat(600);
    temp.child("large.ts").write_str(&content).unwrap();

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("analyze")
        .arg(temp.path())
        .arg("--threshold")
        .arg("loc=500")
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("exceed the LOC threshold"));
}

#[test]
fn test_e2e_config_file_integration() {
    let temp = assert_fs::TempDir::new().unwrap();
    
    // Config to exclude tests/
    temp.child(".code-viz.toml")
        .write_str(r#"
            [analysis]
            exclude = ["tests/**"]
        "#)
        .unwrap();

    temp.child("src/main.ts").write_str("x").unwrap();
    temp.child("tests/test.ts").write_str("x").unwrap();

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    // We need to run analyze in the temp dir for it to pick up config?
    // analyze command currently loads config from `project_root` passed to `load_config`.
    // But `analyze` command logic in `run` doesn't call `load_config` yet!
    // I missed that in 2.1.1 implementation.
    // 2.1.1 task prompt said: "create AnalysisConfig with exclude_patterns (merge defaults + custom)".
    // It didn't explicitly say "load config file using config_loader".
    // But 2.2.1 implemented `config_loader`.
    // I need to update `analyze` command to use `config_loader` to pass this test.
    // So I will implement the test, assume it fails, then fix `analyze.rs`.
    // Or I'll skip this test for now? No, I should fix `analyze.rs`.
    // I'll add the test.
    
    // Note: The CLI arg `path` is the target to analyze.
    // `load_config` takes `project_root`.
    // If I run `code-viz analyze .` in `temp`, it should load `.code-viz.toml` from `.`.
    // `analyze.rs` needs to call `config_loader::load_config`.
    
    cmd.current_dir(temp.path())
        .arg("analyze")
        .arg(".")
        .assert()
        .success()
        // If config works, tests/test.ts is excluded. Total files = 1.
        .stdout(predicate::str::contains("Total Files: 1"));
}

#[test]
fn test_e2e_config_init() {
    let temp = assert_fs::TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.current_dir(temp.path())
        .arg("config")
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created .code-viz.toml"));
        
    assert!(temp.child(".code-viz.toml").exists());
}
