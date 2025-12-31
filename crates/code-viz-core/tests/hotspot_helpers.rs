use code_viz_core::models::{CodeChurn, FileMetrics};
use code_viz_core::hotspot::HotspotDetector;
use std::path::PathBuf;
use std::time::SystemTime;

/// Test helper for creating FileMetrics
fn create_test_file(
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

#[test]
fn test_normalize_score_with_zero_max() {
    // The normalize_score function should return 0.0 when max_value is 0
    // to avoid division by zero
    let detector = HotspotDetector::new(10);

    // Create a file with no churn to test zero max values
    let files = vec![
        FileMetrics {
            path: PathBuf::from("test.rs"),
            language: "rust".to_string(),
            loc: 0,
            size_bytes: 0,
            function_count: 0,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            code_churn: Some(CodeChurn {
                added_lines: 0,
                deleted_lines: 0,
            }),
            coupling: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        }
    ];

    let analysis = detector.calculate(&files);

    // Should handle zero values gracefully
    assert_eq!(analysis.hotspots.len(), 1);
    assert_eq!(analysis.hotspots[0].hotspot_score, 0.0);
}

#[test]
fn test_hotspot_score_calculation() {
    // Test that hotspot score is calculated correctly with weighted components
    let files = vec![
        create_test_file("high_churn.rs", 100, 10, 200, 150), // High churn (350 total)
        create_test_file("baseline.rs", 100, 10, 100, 50),    // Baseline (150 total)
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 2);

    // high_churn.rs should have higher score than baseline.rs
    let high_churn_hotspot = analysis.hotspots.iter()
        .find(|h| h.path == PathBuf::from("high_churn.rs"))
        .expect("high_churn.rs should be in results");

    let baseline_hotspot = analysis.hotspots.iter()
        .find(|h| h.path == PathBuf::from("baseline.rs"))
        .expect("baseline.rs should be in results");

    assert!(high_churn_hotspot.hotspot_score > baseline_hotspot.hotspot_score);
    assert!(high_churn_hotspot.churn_score > baseline_hotspot.churn_score);
}

#[test]
fn test_max_values_calculation() {
    // Test that max values are correctly identified across files
    let files = vec![
        create_test_file("max_churn.rs", 50, 5, 1000, 500),      // Max churn: 1500
        create_test_file("max_complexity.rs", 50, 100, 10, 10),  // Max complexity: 100
        create_test_file("max_size.rs", 1000, 10, 50, 30),       // Max size: 1000
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    // Verify normalization is working correctly
    let max_churn_hotspot = analysis.hotspots.iter()
        .find(|h| h.path == PathBuf::from("max_churn.rs"))
        .expect("max_churn.rs should be in results");

    // Churn score should be 1.0 since it has max churn
    assert_eq!(max_churn_hotspot.churn_score, 1.0);

    let max_complexity_hotspot = analysis.hotspots.iter()
        .find(|h| h.path == PathBuf::from("max_complexity.rs"))
        .expect("max_complexity.rs should be in results");

    // Complexity score should be 1.0 since it has max complexity
    assert_eq!(max_complexity_hotspot.complexity_score, 1.0);

    let max_size_hotspot = analysis.hotspots.iter()
        .find(|h| h.path == PathBuf::from("max_size.rs"))
        .expect("max_size.rs should be in results");

    // Size score should be 1.0 since it has max size
    assert_eq!(max_size_hotspot.size_score, 1.0);
}

#[test]
fn test_hotspot_truncation() {
    // Test that max_hotspots limit is respected
    let files: Vec<FileMetrics> = (0..20)
        .map(|i| create_test_file(&format!("file{}.rs", i), 100, 10, i * 10, i * 5))
        .collect();

    let max_hotspots = 5;
    let detector = HotspotDetector::new(max_hotspots);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), max_hotspots);
    assert_eq!(analysis.total_files_analyzed, 20);
}

#[test]
fn test_hotspots_sorted_by_score() {
    // Test that hotspots are sorted in descending order by score
    let files = vec![
        create_test_file("low.rs", 50, 5, 10, 5),
        create_test_file("medium.rs", 100, 20, 100, 80),
        create_test_file("high.rs", 200, 50, 500, 300),
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 3);

    // Verify descending order
    for i in 0..analysis.hotspots.len() - 1 {
        assert!(
            analysis.hotspots[i].hotspot_score >= analysis.hotspots[i + 1].hotspot_score,
            "Hotspots should be sorted in descending order"
        );
    }
}

#[test]
fn test_files_without_churn_filtered() {
    // Test that files without churn data are filtered out
    let files = vec![
        create_test_file("with_churn.rs", 100, 10, 50, 30),
        FileMetrics {
            path: PathBuf::from("without_churn.rs"),
            language: "rust".to_string(),
            loc: 100,
            size_bytes: 1024,
            function_count: 10,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            code_churn: None, // No churn data
            coupling: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        },
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    // Only file with churn should be included
    assert_eq!(analysis.hotspots.len(), 1);
    assert_eq!(analysis.total_files_analyzed, 1);
    assert_eq!(analysis.hotspots[0].path, PathBuf::from("with_churn.rs"));
}

#[test]
fn test_weighted_score_components() {
    // Test that score components are weighted correctly
    // CHURN_WEIGHT = 0.4, COMPLEXITY_WEIGHT = 0.4, SIZE_WEIGHT = 0.2

    let files = vec![
        FileMetrics {
            path: PathBuf::from("test.rs"),
            language: "rust".to_string(),
            loc: 100,       // This is max, so size_score = 1.0
            size_bytes: 1024,
            function_count: 50,  // This is max, so complexity_score = 1.0
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            code_churn: Some(CodeChurn {
                added_lines: 100,
                deleted_lines: 50,  // Total 150, this is max, so churn_score = 1.0
            }),
            coupling: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        }
    ];

    let detector = HotspotDetector::new(10);
    let analysis = detector.calculate(&files);

    assert_eq!(analysis.hotspots.len(), 1);
    let hotspot = &analysis.hotspots[0];

    // All scores should be 1.0 since this is the only file
    assert_eq!(hotspot.churn_score, 1.0);
    assert_eq!(hotspot.complexity_score, 1.0);
    assert_eq!(hotspot.size_score, 1.0);

    // Weighted total: 1.0 * 0.4 + 1.0 * 0.4 + 1.0 * 0.2 = 1.0
    assert_eq!(hotspot.hotspot_score, 1.0);
}
