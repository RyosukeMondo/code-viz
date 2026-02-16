#![allow(clippy::unwrap_used, clippy::expect_used)]
use code_viz_commands::analyze::{analyze_repository, DuplicationConfig, HotspotConfig};
use code_viz_core::mocks::{MockContext, MockFileSystem, MockGit};
use std::path::Path;

/// Helper: Create a test repository with standard files
fn create_test_repository() -> MockFileSystem {
    MockFileSystem::new()
        .with_file(
            "/test_repo/src/main.rs",
            r#"
fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#,
        )
        .with_file(
            "/test_repo/src/lib.rs",
            r#"
pub fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

pub fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}
"#,
        )
        .with_file(
            "/test_repo/src/utils.ts",
            r#"
export function formatName(name: string): string {
    return name.trim().toLowerCase();
}

export function calculateAge(birthYear: number): number {
    const currentYear = new Date().getFullYear();
    return currentYear - birthYear;
}
"#,
        )
        .with_file(
            "/test_repo/tests/test_lib.rs",
            r#"
#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
    }
}
"#,
        )
}

/// Helper: Create a large test repository
fn create_large_repository() -> MockFileSystem {
    let mut fs = MockFileSystem::new();

    // Generate 50 files
    for i in 0..50 {
        let path = format!("/test_repo/src/module_{}.rs", i);
        let content = format!(
            r#"
pub fn function_{}() -> i32 {{
    let x = {};
    let y = x * 2;
    let z = y + 1;
    z
}}

pub fn helper_{}() -> String {{
    "module_{}".to_string()
}}
"#,
            i, i, i, i
        );
        fs = fs.with_file(path, content);
    }

    fs
}

/// Helper: Create repository with malformed files
fn create_malformed_repository() -> MockFileSystem {
    MockFileSystem::new()
        .with_file(
            "/test_repo/src/good.rs",
            r#"
fn good_function() {
    println!("This is fine");
}
"#,
        )
        .with_file(
            "/test_repo/src/broken.rs",
            r#"
fn broken_function( {
    // Missing closing brace and param list
"#,
        )
        .with_file(
            "/test_repo/src/another_good.ts",
            r#"
function goodTs() {
    console.log("TypeScript works");
}
"#,
        )
}

/// Helper: Create empty repository
fn create_empty_repository() -> MockFileSystem {
    MockFileSystem::new()
}

/// Helper: Create multi-language repository
fn create_multi_language_repository() -> MockFileSystem {
    MockFileSystem::new()
        .with_file(
            "/multi_lang_repo/src/main.rs",
            r#"
fn main() {
    println!("Rust code");
}
"#,
        )
        .with_file(
            "/multi_lang_repo/src/app.ts",
            r#"
function main() {
    console.log("TypeScript code");
}
"#,
        )
        .with_file(
            "/multi_lang_repo/src/script.py",
            r#"
def main():
    print("Python code")
"#,
        )
        .with_file(
            "/multi_lang_repo/src/main.go",
            r#"
package main

func main() {
    println("Go code")
}
"#,
        )
}

#[tokio::test]
async fn test_full_analysis_pipeline() {
    let fs = create_test_repository();
    let ctx = MockContext::new();
    let git = MockGit::new();
    let repo_path = Path::new("/test_repo");

    let result = analyze_repository(
        repo_path,
        ctx.clone(),
        fs,
        &git,
        Some(DuplicationConfig {
            min_lines: 5,
            similarity_threshold: 0.9,
        }),
        Some(HotspotConfig { max_hotspots: 10 }),
        None,
    )
    .await;

    assert!(result.is_ok(), "Full analysis should succeed");
    let result = result.unwrap();

    // Verify basic results
    assert!(!result.files.is_empty(), "Should analyze files");
    assert!(result.summary.total_files > 0, "Should have file count");
    assert!(result.summary.total_loc > 0, "Should have LOC count");

    // Verify duplication analysis ran
    assert!(
        result.duplication.is_some(),
        "Duplication analysis should be present"
    );

    // Verify hotspot analysis ran
    assert!(
        result.hotspot_analysis.is_some(),
        "Hotspot analysis should be present"
    );

    // Verify progress events were emitted
    ctx.assert_event_emitted("progress");
    ctx.assert_event_emitted("analysis_complete");
}

#[tokio::test]
async fn test_partial_analysis() {
    let fs = create_test_repository();
    let ctx = MockContext::new();
    let git = MockGit::new();
    let repo_path = Path::new("/test_repo");

    // Run analysis without duplication and hotspots
    let result = analyze_repository(
        repo_path,
        ctx.clone(),
        fs,
        &git,
        None, // No duplication
        None, // No hotspots
        None, // No coverage
    )
    .await;

    assert!(result.is_ok(), "Partial analysis should succeed");
    let result = result.unwrap();

    // Verify basic results
    assert!(!result.files.is_empty(), "Should analyze files");
    assert!(result.summary.total_files > 0, "Should have file count");

    // Verify optional analyses are absent
    assert!(
        result.duplication.is_none(),
        "Duplication analysis should be absent"
    );
    assert!(
        result.coverage_analysis.is_none(),
        "Coverage analysis should be absent"
    );
}

#[tokio::test]
async fn test_error_recovery_malformed_files() {
    let fs = create_malformed_repository();
    let ctx = MockContext::new();
    let git = MockGit::new();
    let repo_path = Path::new("/test_repo");

    let result = analyze_repository(repo_path, ctx, fs, &git, None, None, None).await;

    // Analysis should succeed even with malformed files
    assert!(
        result.is_ok(),
        "Analysis should handle malformed files gracefully"
    );
    let result = result.unwrap();

    // Should have processed all files (parser handles errors gracefully)
    assert!(result.files.len() >= 2, "Should process valid files");

    // Verify we got both Rust files (even the malformed one is processed)
    let rust_files: Vec<_> = result
        .files
        .iter()
        .filter(|f| f.language == "rust")
        .collect();
    assert_eq!(
        rust_files.len(),
        2,
        "Should process both Rust files (parser handles errors)"
    );

    // Verify TypeScript file was also processed
    let ts_files: Vec<_> = result
        .files
        .iter()
        .filter(|f| f.language == "typescript")
        .collect();
    assert_eq!(ts_files.len(), 1, "Should process TypeScript file");
}

#[tokio::test]
async fn test_large_repository() {
    let fs = create_large_repository();
    let ctx = MockContext::new();
    let git = MockGit::new();
    let repo_path = Path::new("/test_repo");

    let start = std::time::Instant::now();

    let result = analyze_repository(repo_path, ctx.clone(), fs, &git, None, None, None).await;

    let duration = start.elapsed();

    assert!(result.is_ok(), "Large repository analysis should succeed");
    let result = result.unwrap();

    // Verify we processed all files
    assert_eq!(result.files.len(), 50, "Should process all 50 files");
    assert_eq!(
        result.summary.total_files, 50,
        "Summary should reflect 50 files"
    );

    // Performance check: should complete in reasonable time (< 5 seconds)
    assert!(
        duration.as_secs() < 5,
        "Large repository analysis should complete in < 5 seconds, took {:?}",
        duration
    );

    // Verify progress reporting
    let progress_events = ctx.get_events_by_name("progress");
    assert!(
        !progress_events.is_empty(),
        "Should emit progress events for large repo"
    );
}

#[tokio::test]
async fn test_empty_repository() {
    let fs = create_empty_repository();
    let ctx = MockContext::new();
    let git = MockGit::new();
    let repo_path = Path::new("/empty_repo");

    let result = analyze_repository(repo_path, ctx, fs, &git, None, None, None).await;

    assert!(
        result.is_ok(),
        "Empty repository should be handled gracefully"
    );
    let result = result.unwrap();

    // Verify empty results
    assert_eq!(result.files.len(), 0, "Should have no files");
    assert_eq!(result.summary.total_files, 0, "Summary should show 0 files");
    assert_eq!(result.summary.total_loc, 0, "Summary should show 0 LOC");
    assert_eq!(
        result.summary.total_functions, 0,
        "Summary should show 0 functions"
    );
}

#[tokio::test]
async fn test_multi_language_repository() {
    let fs = create_multi_language_repository();
    let ctx = MockContext::new();
    let git = MockGit::new();
    let repo_path = Path::new("/multi_lang_repo");

    let result = analyze_repository(repo_path, ctx, fs, &git, None, None, None).await;

    assert!(result.is_ok(), "Multi-language analysis should succeed");
    let result = result.unwrap();

    // Verify we analyzed files from different languages
    let languages: std::collections::HashSet<_> =
        result.files.iter().map(|f| f.language.as_str()).collect();

    assert!(languages.contains("rust"), "Should detect Rust files");
    assert!(
        languages.contains("typescript"),
        "Should detect TypeScript files"
    );
    assert!(languages.contains("python"), "Should detect Python files");
    assert!(languages.contains("go"), "Should detect Go files");

    // Verify file counts per language
    let rust_count = result.files.iter().filter(|f| f.language == "rust").count();
    let ts_count = result
        .files
        .iter()
        .filter(|f| f.language == "typescript")
        .count();
    let py_count = result
        .files
        .iter()
        .filter(|f| f.language == "python")
        .count();
    let go_count = result.files.iter().filter(|f| f.language == "go").count();

    assert_eq!(rust_count, 1, "Should have 1 Rust file");
    assert_eq!(ts_count, 1, "Should have 1 TypeScript file");
    assert_eq!(py_count, 1, "Should have 1 Python file");
    assert_eq!(go_count, 1, "Should have 1 Go file");
}

#[tokio::test]
async fn test_analysis_determinism() {
    let fs = create_test_repository();
    let ctx = MockContext::new();
    let git = MockGit::new();
    let repo_path = Path::new("/test_repo");

    // Run analysis twice
    let result1 = analyze_repository(repo_path, ctx.clone(), fs.clone(), &git, None, None, None)
        .await
        .unwrap();

    let result2 = analyze_repository(repo_path, ctx.clone(), fs.clone(), &git, None, None, None)
        .await
        .unwrap();

    // Results should be deterministic (excluding timestamp)
    assert_eq!(
        result1.files.len(),
        result2.files.len(),
        "File count should be deterministic"
    );
    assert_eq!(
        result1.summary.total_loc, result2.summary.total_loc,
        "LOC count should be deterministic"
    );
    assert_eq!(
        result1.summary.total_functions, result2.summary.total_functions,
        "Function count should be deterministic"
    );
}
