#![allow(clippy::unwrap_used, clippy::expect_used)]
use code_viz_core::hotspot::HotspotDetector;
use code_viz_core::models::{CodeChurn, FileMetrics};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

fn create_file_with_churn(
    path: &str,
    loc: usize,
    function_count: usize,
    added: usize,
    deleted: usize,
) -> FileMetrics {
    FileMetrics {
        path: PathBuf::from(path),
        language: "rust".to_string(),
        loc,
        size_bytes: 1024,
        function_count,
        last_modified: SystemTime::now(),
        dead_function_count: None,
        dead_code_loc: None,
        dead_code_ratio: None,
        code_churn: Some(CodeChurn {
            added_lines: added,
            deleted_lines: deleted,
        }),
        coupling: None,
        ai_bloat_index: None,
        cognitive_complexity: None,
        test_coverage: None,
    }
}

fn create_file_without_churn(path: &str, loc: usize, function_count: usize) -> FileMetrics {
    FileMetrics {
        path: PathBuf::from(path),
        language: "rust".to_string(),
        loc,
        size_bytes: 1024,
        function_count,
        last_modified: SystemTime::now(),
        dead_function_count: None,
        dead_code_loc: None,
        dead_code_ratio: None,
        code_churn: None,
        coupling: None,
        ai_bloat_index: None,
        cognitive_complexity: None,
        test_coverage: None,
    }
}

#[test]
fn test_basic_hotspot_detection() {
    let files = vec![
        create_file_with_churn("high_churn.rs", 100, 10, 200, 150),
        create_file_with_churn("low_churn.rs", 50, 5, 10, 5),
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 2);
    assert_eq!(analysis.total_files_analyzed, 2);

    // High churn file should rank higher
    assert!(analysis.hotspots[0].hotspot_score > analysis.hotspots[1].hotspot_score);
    assert_eq!(analysis.hotspots[0].path, PathBuf::from("high_churn.rs"));
}

#[test]
fn test_hotspot_limit_enforcement() {
    let files = vec![
        create_file_with_churn("a.rs", 100, 10, 100, 80),
        create_file_with_churn("b.rs", 110, 11, 110, 85),
        create_file_with_churn("c.rs", 120, 12, 120, 90),
        create_file_with_churn("d.rs", 130, 13, 130, 95),
        create_file_with_churn("e.rs", 140, 14, 140, 100),
    ];

    let detector = HotspotDetector::new(3);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 3);
    assert_eq!(analysis.total_files_analyzed, 5);

    // Should return top 3 by score
    for i in 0..2 {
        assert!(analysis.hotspots[i].hotspot_score >= analysis.hotspots[i + 1].hotspot_score);
    }
}

#[test]
fn test_empty_file_list() {
    let files = vec![];
    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 0);
    assert_eq!(analysis.total_files_analyzed, 0);
}

#[test]
fn test_files_without_churn_ignored() {
    let files = vec![
        create_file_without_churn("no_churn.rs", 100, 10),
        create_file_without_churn("also_no_churn.rs", 200, 20),
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 0);
    assert_eq!(analysis.total_files_analyzed, 0);
}

#[test]
fn test_mixed_churn_and_no_churn() {
    let files = vec![
        create_file_with_churn("with_churn.rs", 100, 10, 50, 30),
        create_file_without_churn("no_churn.rs", 100, 10),
        create_file_with_churn("also_with_churn.rs", 120, 12, 60, 40),
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 2);
    assert_eq!(analysis.total_files_analyzed, 2);
}

#[test]
fn test_zero_churn_file() {
    let files = vec![create_file_with_churn("zero.rs", 100, 10, 0, 0)];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.total_files_analyzed, 1);
    assert_eq!(analysis.hotspots.len(), 1);

    let hotspot = &analysis.hotspots[0];
    assert_eq!(hotspot.total_changes, 0);
    assert_eq!(hotspot.churn_score, 0.0);
}

#[test]
fn test_churn_score_calculation() {
    let files = vec![
        create_file_with_churn("a.rs", 100, 10, 100, 0), // 100 total changes
        create_file_with_churn("b.rs", 100, 10, 50, 50), // 100 total changes
        create_file_with_churn("c.rs", 100, 10, 0, 100), // 100 total changes
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    // All files have same churn, should have same churn_score
    assert!(analysis.hotspots.len() == 3);
    let first_churn = analysis.hotspots[0].churn_score;
    for hotspot in &analysis.hotspots {
        assert_eq!(hotspot.churn_score, first_churn);
        assert_eq!(hotspot.total_changes, 100);
    }
}

#[test]
fn test_complexity_score_calculation() {
    let files = vec![
        create_file_with_churn("simple.rs", 50, 5, 10, 10), // Low complexity
        create_file_with_churn("complex.rs", 50, 50, 10, 10), // High complexity
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 2);

    // Find hotspots by path
    let complex = analysis
        .hotspots
        .iter()
        .find(|h| h.path == Path::new("complex.rs"))
        .unwrap(); // Test-only: known fixture

    let simple = analysis
        .hotspots
        .iter()
        .find(|h| h.path == Path::new("simple.rs"))
        .unwrap(); // Test-only: known fixture

    assert!(complex.complexity_score > simple.complexity_score);
}

#[test]
fn test_size_score_calculation() {
    let files = vec![
        create_file_with_churn("small.rs", 50, 10, 10, 10), // Small file
        create_file_with_churn("large.rs", 500, 10, 10, 10), // Large file
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 2);

    let large = analysis
        .hotspots
        .iter()
        .find(|h| h.path == Path::new("large.rs"))
        .unwrap(); // Test-only: known fixture

    let small = analysis
        .hotspots
        .iter()
        .find(|h| h.path == Path::new("small.rs"))
        .unwrap(); // Test-only: known fixture

    assert!(large.size_score > small.size_score);
}

#[test]
fn test_combined_score_weighting() {
    // Create files that excel in different dimensions
    let files = vec![
        create_file_with_churn("high_churn.rs", 100, 10, 1000, 1000), // High churn
        create_file_with_churn("high_complexity.rs", 100, 200, 10, 10), // High complexity
        create_file_with_churn("large_file.rs", 5000, 10, 10, 10),    // Large size
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 3);

    // All should have different scores based on their strengths
    let scores: Vec<f64> = analysis.hotspots.iter().map(|h| h.hotspot_score).collect();
    assert!(scores[0] != scores[1]);
    assert!(scores[1] != scores[2]);

    // High churn should likely rank highest (40% weight)
    assert_eq!(analysis.hotspots[0].path, PathBuf::from("high_churn.rs"));
}

#[test]
fn test_sorting_by_hotspot_score() {
    let files = vec![
        create_file_with_churn("low.rs", 50, 5, 10, 5),
        create_file_with_churn("high.rs", 500, 50, 500, 400),
        create_file_with_churn("medium.rs", 200, 20, 100, 80),
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 3);

    // Verify sorting: each score should be >= next
    for i in 0..analysis.hotspots.len() - 1 {
        assert!(analysis.hotspots[i].hotspot_score >= analysis.hotspots[i + 1].hotspot_score);
    }
}

#[test]
fn test_single_file_analysis() {
    let files = vec![create_file_with_churn("solo.rs", 100, 10, 50, 30)];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 1);
    assert_eq!(analysis.total_files_analyzed, 1);

    let hotspot = &analysis.hotspots[0];
    assert_eq!(hotspot.path, PathBuf::from("solo.rs"));
    assert_eq!(hotspot.total_changes, 80);
}

#[test]
fn test_max_hotspots_zero() {
    let files = vec![
        create_file_with_churn("a.rs", 100, 10, 50, 30),
        create_file_with_churn("b.rs", 100, 10, 50, 30),
    ];

    let detector = HotspotDetector::new(0);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 0);
    assert_eq!(analysis.total_files_analyzed, 2); // Still analyzed
}

#[test]
fn test_max_hotspots_exceeds_file_count() {
    let files = vec![
        create_file_with_churn("a.rs", 100, 10, 50, 30),
        create_file_with_churn("b.rs", 100, 10, 50, 30),
    ];

    let detector = HotspotDetector::new(100);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 2); // Only 2 files available
    assert_eq!(analysis.total_files_analyzed, 2);
}

#[test]
fn test_identical_files() {
    let files = vec![
        create_file_with_churn("a.rs", 100, 10, 50, 30),
        create_file_with_churn("b.rs", 100, 10, 50, 30),
        create_file_with_churn("c.rs", 100, 10, 50, 30),
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 3);

    // All should have identical scores
    let first_score = analysis.hotspots[0].hotspot_score;
    for hotspot in &analysis.hotspots {
        assert_eq!(hotspot.hotspot_score, first_score);
    }
}

#[test]
fn test_very_high_churn() {
    let files = vec![
        create_file_with_churn("extreme.rs", 100, 10, 10000, 10000),
        create_file_with_churn("normal.rs", 100, 10, 10, 10),
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    let extreme = &analysis.hotspots[0];
    assert_eq!(extreme.path, PathBuf::from("extreme.rs"));
    assert_eq!(extreme.total_changes, 20000);
}

#[test]
fn test_score_normalization() {
    let files = vec![
        create_file_with_churn("a.rs", 1000, 100, 1000, 1000),
        create_file_with_churn("b.rs", 1, 1, 1, 1),
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    // Check that scores are normalized (between 0 and 1)
    for hotspot in &analysis.hotspots {
        assert!(hotspot.churn_score >= 0.0 && hotspot.churn_score <= 1.0);
        assert!(hotspot.complexity_score >= 0.0 && hotspot.complexity_score <= 1.0);
        assert!(hotspot.size_score >= 0.0 && hotspot.size_score <= 1.0);
        assert!(hotspot.hotspot_score >= 0.0 && hotspot.hotspot_score <= 1.0);
    }
}

#[test]
fn test_large_dataset() {
    let mut files = vec![];
    for i in 0..1000 {
        files.push(create_file_with_churn(
            &format!("file_{i}.rs"),
            100 + i,
            10 + (i % 50),
            50 + (i % 100),
            30 + (i % 80),
        ));
    }

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 10);
    assert_eq!(analysis.total_files_analyzed, 1000);

    // Verify sorting
    for i in 0..analysis.hotspots.len() - 1 {
        assert!(analysis.hotspots[i].hotspot_score >= analysis.hotspots[i + 1].hotspot_score);
    }
}

#[test]
fn test_all_metrics_present_in_hotspot() {
    let files = vec![create_file_with_churn("test.rs", 200, 25, 100, 75)];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 1);
    let hotspot = &analysis.hotspots[0];

    // Verify all fields are populated
    assert_eq!(hotspot.path, PathBuf::from("test.rs"));
    assert!(hotspot.hotspot_score > 0.0);
    assert!(hotspot.churn_score > 0.0);
    assert!(hotspot.complexity_score > 0.0);
    assert!(hotspot.size_score > 0.0);
    assert_eq!(hotspot.total_changes, 175);
}
