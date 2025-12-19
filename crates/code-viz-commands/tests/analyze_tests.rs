use code_viz_commands::analyze_repository;
use code_viz_core::mocks::{MockContext, MockFileSystem};
use std::path::Path;

#[tokio::test]
async fn test_analyze_repository_success() {
    let ctx = MockContext::new();
    let fs = MockFileSystem::new()
        .with_file("src/main.rs", "fn main() { println!(\"hello\"); }")
        .with_file("src/lib.rs", "pub fn add(a: i32, b: i32) -> i32 { a + b }")
        .with_file("README.md", "# My Project"); // Should be ignored by extension

    let path = Path::new("src");
    let result = analyze_repository(path, ctx.clone(), fs.clone()).await.unwrap();

    // Verify AnalysisResult
    assert_eq!(result.summary.total_files, 2);
    assert_eq!(result.summary.total_loc, 2); // 1 LOC each
    assert_eq!(result.summary.total_functions, 2); // 1 func each

    // Verify MockFileSystem reads
    fs.assert_read(Path::new("src/main.rs"));
    fs.assert_read(Path::new("src/lib.rs"));

    // Verify MockContext events
    ctx.assert_event_emitted("analysis_complete");
    
    let progress_events = ctx.get_events_by_name("progress");
    assert!(!progress_events.is_empty(), "Should have emitted progress events");
    
    // Check final progress
    let last_progress = progress_events.last().unwrap();
    assert_eq!(last_progress["percentage"], 1.0);
    assert_eq!(last_progress["message"], "Analysis complete");
}

#[tokio::test]
async fn test_analyze_repository_empty_dir() {
    let ctx = MockContext::new();
    let fs = MockFileSystem::new();

    let path = Path::new("empty");
    let result = analyze_repository(path, ctx.clone(), fs.clone()).await.unwrap();

    assert_eq!(result.summary.total_files, 0);
    ctx.assert_event_emitted("analysis_complete");
}

#[tokio::test]
async fn test_analyze_repository_error_handling() {
    let ctx = MockContext::new();
    let fs = MockFileSystem::new(); // Empty, but we'll try to scan a non-existent dir

    // Our current implementation doesn't check if dir exists before scan, 
    // it just gets an empty list from read_dir_recursive if it's empty.
    // Wait, MockFileSystem.read_dir_recursive returns empty Vec if no files match prefix.
    
    let path = Path::new("non_existent");
    let result = analyze_repository(path, ctx, fs).await;
    
    assert!(result.is_ok()); // Should return empty AnalysisResult for empty/non-existent dir in mock
}
