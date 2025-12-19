use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

/// Create a simple test repository with known dead code
fn create_test_repo(temp: &assert_fs::TempDir) {
    // Create a file with dead function
    temp.child("src/dead.ts")
        .write_str(
            r#"
// This function is never imported or called
export function unusedFunction() {
    return "dead";
}

// This function is also never used
function internalDead() {
    return "also dead";
}
"#,
        )
        .unwrap();

    // Create main entry point that doesn't use dead code
    temp.child("src/main.ts")
        .write_str(
            r#"
function main() {
    console.log("Running");
}

main();
"#,
        )
        .unwrap();
}

#[test]
fn test_e2e_dead_code_json_output() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("dead-code")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"summary\""))
        .stdout(predicate::str::contains("\"deadFunctions\""))
        .stdout(predicate::str::contains("\"deadClasses\""))
        .stdout(predicate::str::contains("\"totalDeadLoc\""))
        .stdout(predicate::str::contains("\"deadCodeRatio\""))
        .stdout(predicate::str::contains("\"files\""));
}

#[test]
fn test_e2e_dead_code_text_output() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("dead-code")
        .arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Dead Code Analysis Summary"))
        .stdout(predicate::str::contains("Total files analyzed:"))
        .stdout(predicate::str::contains("Files with dead code:"))
        .stdout(predicate::str::contains("Dead functions:"))
        .stdout(predicate::str::contains("Dead code ratio:"));
}

#[test]
fn test_e2e_min_confidence_filter() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    // Run with very high confidence filter (90)
    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    let output_high = cmd
        .arg("dead-code")
        .arg(temp.path())
        .arg("--min-confidence")
        .arg("90")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    // Run with low confidence filter (0)
    let mut cmd2 = Command::cargo_bin("code-viz-cli").unwrap();
    let output_low = cmd2
        .arg("dead-code")
        .arg(temp.path())
        .arg("--min-confidence")
        .arg("0")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let high_json: serde_json::Value = serde_json::from_slice(&output_high).unwrap();
    let low_json: serde_json::Value = serde_json::from_slice(&output_low).unwrap();

    // High confidence filter should have fewer or equal dead symbols
    let high_dead_funcs = high_json["summary"]["deadFunctions"].as_u64().unwrap();
    let low_dead_funcs = low_json["summary"]["deadFunctions"].as_u64().unwrap();

    assert!(
        high_dead_funcs <= low_dead_funcs,
        "High confidence filter should have fewer or equal dead functions: {} vs {}",
        high_dead_funcs,
        low_dead_funcs
    );
}

#[test]
fn test_e2e_analyze_with_dead_code_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("analyze")
        .arg(temp.path())
        .arg("--dead-code")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"total_files\""))
        .stdout(predicate::str::contains("\"total_loc\""));

    // Verify CSV format includes dead code columns when enabled
    let mut cmd2 = Command::cargo_bin("code-viz-cli").unwrap();
    cmd2.arg("analyze")
        .arg(temp.path())
        .arg("--dead-code")
        .arg("--format")
        .arg("csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("dead_functions"))
        .stdout(predicate::str::contains("dead_loc"))
        .stdout(predicate::str::contains("dead_code_ratio"));
}

#[test]
fn test_e2e_threshold_violation_dead_code_ratio() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    // First, verify the repo has some dead code by running without threshold
    let mut cmd_check = Command::cargo_bin("code-viz-cli").unwrap();
    let output = cmd_check
        .arg("dead-code")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let dead_ratio = json["summary"]["dead_code_ratio"].as_f64().unwrap();

    // Only test threshold if we actually have dead code
    if dead_ratio > 0.0 {
        // Set a threshold slightly below the actual ratio to trigger failure
        let threshold = dead_ratio * 0.5;
        let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
        cmd.arg("dead-code")
            .arg(temp.path())
            .arg("--threshold")
            .arg(format!("dead_code_ratio={}", threshold))
            .assert()
            .failure()
            .code(3)
            .stderr(predicate::str::contains("Dead code ratio"));
    }
}

#[test]
fn test_e2e_threshold_violation_dead_functions() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    // Get actual dead function count
    let mut cmd_check = Command::cargo_bin("code-viz-cli").unwrap();
    let output = cmd_check
        .arg("dead-code")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let dead_functions = json["summary"]["dead_functions"].as_u64().unwrap();

    // Only test threshold if we actually have dead functions
    if dead_functions > 0 {
        // Set threshold below actual count
        let threshold = dead_functions - 1;
        let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
        cmd.arg("dead-code")
            .arg(temp.path())
            .arg("--threshold")
            .arg(format!("dead_functions={}", threshold))
            .assert()
            .failure()
            .code(3)
            .stderr(predicate::str::contains("Dead functions"));
    }
}

#[test]
fn test_e2e_threshold_pass() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    // Set a very high threshold that won't be exceeded
    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("dead-code")
        .arg(temp.path())
        .arg("--threshold")
        .arg("dead_code_ratio=1.0")
        .assert()
        .success();
}

#[test]
fn test_e2e_output_to_file() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    let output_file = temp.child("dead-code-report.json");

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("dead-code")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .arg("--output")
        .arg(output_file.path())
        .assert()
        .success();

    // Verify file was created and contains valid JSON
    assert!(output_file.exists());
    let content = std::fs::read_to_string(output_file.path()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(json["summary"].is_object());
}

#[test]
fn test_e2e_exclude_patterns() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);
    temp.child("tests/test.ts")
        .write_str("function test() {}")
        .unwrap();

    // Exclude tests directory - should analyze fewer files
    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    let output = cmd
        .arg("dead-code")
        .arg(temp.path())
        .arg("--exclude")
        .arg("tests/**")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let files_analyzed = json["summary"]["totalFiles"].as_u64().unwrap();

    // Should have analyzed some files but not the tests directory
    assert!(files_analyzed > 0, "Should analyze at least some files");
    assert!(files_analyzed <= 2, "Should only analyze src/ files, not tests/");
}

#[test]
fn test_e2e_verbose_logging() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("dead-code")
        .arg(temp.path())
        .arg("--verbose")
        .assert()
        .success();

    // We can't easily test for debug log output since it goes to stderr
    // and the format depends on env_logger configuration,
    // but we verify the command succeeds with --verbose flag
}

#[test]
fn test_e2e_empty_directory() {
    let temp = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("dead-code")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"totalFiles\": 0"));
}

#[test]
fn test_e2e_no_dead_code() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Create a simple file with only used code
    temp.child("src/main.ts")
        .write_str("function main() { console.log('hello'); }\nmain();")
        .unwrap();

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("dead-code")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"deadFunctions\": 0"))
        .stdout(predicate::str::contains("\"deadCodeRatio\": 0"));
}

#[test]
fn test_e2e_invalid_threshold_format() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    cmd.arg("dead-code")
        .arg(temp.path())
        .arg("--threshold")
        .arg("invalid_format")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid threshold"));
}

#[test]
fn test_e2e_confidence_tiers_in_text_output() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    let output = cmd
        .arg("dead-code")
        .arg(temp.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).unwrap();

    // Check that text output includes summary sections
    assert!(text.contains("Dead Code Analysis Summary"));
    assert!(text.contains("Total files analyzed:"));
    assert!(text.contains("Dead code ratio:"));

    // If there's dead code, confidence tiers should appear
    // Otherwise, should say "No dead code found!"
    assert!(
        text.contains("High Confidence")
            || text.contains("Medium Confidence")
            || text.contains("Low Confidence")
            || text.contains("No dead code found!"),
        "Text output should contain either confidence tier sections or 'No dead code found!'"
    );
}

#[test]
fn test_e2e_json_schema_validation() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
    let output = cmd
        .arg("dead-code")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();

    // Validate JSON structure matches expected schema
    assert!(json.is_object());
    assert!(json["summary"].is_object());
    assert!(json["summary"]["totalFiles"].is_number());
    assert!(json["summary"]["filesWithDeadCode"].is_number());
    assert!(json["summary"]["deadFunctions"].is_number());
    assert!(json["summary"]["deadClasses"].is_number());
    assert!(json["summary"]["totalDeadLoc"].is_number());
    assert!(json["summary"]["deadCodeRatio"].is_number());
    assert!(json["files"].is_array());

    // Validate each file entry has correct structure
    if let Some(files) = json["files"].as_array() {
        for file in files {
            assert!(file["path"].is_string());
            assert!(file["deadCode"].is_array());

            // Validate each dead symbol has correct structure
            if let Some(symbols) = file["deadCode"].as_array() {
                for symbol in symbols {
                    assert!(symbol["symbol"].is_string());
                    assert!(symbol["kind"].is_string());
                    assert!(symbol["lineStart"].is_number());
                    assert!(symbol["lineEnd"].is_number());
                    assert!(symbol["confidence"].is_number());
                    assert!(symbol["reason"].is_string());
                }
            }
        }
    }
}

#[test]
fn test_e2e_analyze_threshold_dead_code_ratio() {
    let temp = assert_fs::TempDir::new().unwrap();
    create_test_repo(&temp);

    // Get actual dead code ratio first
    let mut cmd_check = Command::cargo_bin("code-viz-cli").unwrap();
    let output = cmd_check
        .arg("analyze")
        .arg(temp.path())
        .arg("--dead-code")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();

    // Find max dead_code_ratio from files
    let mut max_ratio = 0.0;
    if let Some(files) = json["files"].as_array() {
        for file in files {
            if let Some(ratio) = file["dead_code_ratio"].as_f64() {
                if ratio > max_ratio {
                    max_ratio = ratio;
                }
            }
        }
    }

    // Only test threshold if we have dead code
    if max_ratio > 0.0 {
        let threshold = max_ratio * 0.5;
        let mut cmd = Command::cargo_bin("code-viz-cli").unwrap();
        cmd.arg("analyze")
            .arg(temp.path())
            .arg("--dead-code")
            .arg("--threshold")
            .arg(format!("dead_code_ratio={}", threshold))
            .assert()
            .failure()
            .code(3)
            .stderr(predicate::str::contains("exceed the dead code ratio threshold"));
    }
}
