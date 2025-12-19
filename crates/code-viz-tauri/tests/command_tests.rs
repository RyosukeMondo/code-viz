use code_viz_tauri::commands::analyze_repository_inner;
use code_viz_tauri::context::RealFileSystem;
use code_viz_core::mocks::MockContext;
use std::env;

/// Unit test for analyze_repository_inner using MockContext.
/// This test verifies that the command logic correctly reports progress
/// and returns the analysis results without requiring a full Tauri runtime.
#[tokio::test]
async fn test_analyze_repository_with_mock_context() {
    // 1. Setup MockContext and RealFileSystem
    let ctx = MockContext::new();
    let fs = RealFileSystem::new();

    // 2. Prepare arguments (use current directory for analysis)
    let current_dir = env::current_dir()
        .expect("Failed to get current directory")
        .to_string_lossy()
        .to_string();

    // 3. Execute the command inner function
    let result = analyze_repository_inner(
        ctx.clone(),
        fs,
        current_dir,
        Some("test-request-id".to_string())
    ).await;
    
    // 4. Verify results
    assert!(result.is_ok(), "Command failed: {:?}", result.err());
    let tree = result.unwrap();
    assert!(!tree.name.is_empty());
    
    // 5. Verify event emissions via MockContext
    let events = ctx.get_events();
    
    // Check for progress events
    let progress_events: Vec<_> = events.iter()
        .filter(|(name, _)| name == "progress")
        .collect();
    
    // We expect at least 5 progress reports based on the implementation
    assert!(progress_events.len() >= 5, "Expected at least 5 progress events, got {}", progress_events.len());
    
    // Verify the first progress report
    let first_progress = progress_events.first().unwrap().1.as_object().unwrap();
    let first_percentage = first_progress["percentage"].as_f64().unwrap();
    assert!((first_percentage - 0.1).abs() < 1e-6);
    assert_eq!(first_progress["message"].as_str().unwrap(), "Scanning directory...");
    
    // Verify the final progress report
    let last_progress = progress_events.last().unwrap().1.as_object().unwrap();
    let last_percentage = last_progress["percentage"].as_f64().unwrap();
    assert!((last_percentage - 1.0).abs() < 1e-6);
    assert_eq!(last_progress["message"].as_str().unwrap(), "Analysis complete");
    
    // Use the utility method for a simple assertion
    ctx.assert_event_emitted("progress");
}

/// Verify MockContext handles custom app directory correctly
#[tokio::test]
async fn test_mock_context_app_dir() {
    let custom_path = std::env::temp_dir().join("code-viz-test-dir");
    let ctx = MockContext::new().with_app_dir(custom_path.clone());

    use code_viz_core::traits::AppContext;
    assert_eq!(ctx.get_app_dir(), custom_path);
}
