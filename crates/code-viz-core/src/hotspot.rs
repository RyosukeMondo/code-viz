use crate::models::{FileMetrics, Hotspot, HotspotAnalysis};

/// Weights for hotspot score calculation
const CHURN_WEIGHT: f64 = 0.4;
const COMPLEXITY_WEIGHT: f64 = 0.4;
const SIZE_WEIGHT: f64 = 0.2;

pub struct HotspotDetector {
    /// Maximum number of hotspots to return
    max_hotspots: usize,
}

impl HotspotDetector {
    pub fn new(max_hotspots: usize) -> Self {
        Self { max_hotspots }
    }

    fn calculate_max_values(files: &[&FileMetrics]) -> (f64, f64, f64) {
        let max_churn = files
            .iter()
            .filter_map(|f| {
                f.code_churn.as_ref().map(|churn| churn.added_lines + churn.deleted_lines)
            })
            .max()
            .unwrap_or(1) as f64;

        let max_complexity = files.iter().map(|f| f.function_count).max().unwrap_or(1) as f64;
        let max_size = files.iter().map(|f| f.loc).max().unwrap_or(1) as f64;

        (max_churn, max_complexity, max_size)
    }

    fn normalize_score(value: f64, max_value: f64) -> f64 {
        if max_value > 0.0 {
            value / max_value
        } else {
            0.0
        }
    }

    fn calculate_hotspot_for_file(
        file: &FileMetrics,
        max_churn: f64,
        max_complexity: f64,
        max_size: f64,
    ) -> Option<Hotspot> {
        let churn = file.code_churn.as_ref()?;
        let total_changes = churn.added_lines + churn.deleted_lines;

        let churn_score = Self::normalize_score(total_changes as f64, max_churn);
        let complexity_score = Self::normalize_score(file.function_count as f64, max_complexity);
        let size_score = Self::normalize_score(file.loc as f64, max_size);

        let hotspot_score = (churn_score * CHURN_WEIGHT)
            + (complexity_score * COMPLEXITY_WEIGHT)
            + (size_score * SIZE_WEIGHT);

        Some(Hotspot {
            path: file.path.clone(),
            hotspot_score,
            churn_score,
            complexity_score,
            size_score,
            total_changes,
        })
    }

    /// Calculate hotspot analysis from file metrics
    /// Requires files to have code_churn data populated
    pub fn calculate(&self, files: &[FileMetrics]) -> HotspotAnalysis {
        let files_with_churn: Vec<&FileMetrics> = files
            .iter()
            .filter(|f| f.code_churn.is_some())
            .collect();

        if files_with_churn.is_empty() {
            return HotspotAnalysis {
                hotspots: Vec::new(),
                total_files_analyzed: 0,
            };
        }

        let (max_churn, max_complexity, max_size) = Self::calculate_max_values(&files_with_churn);

        let mut hotspots: Vec<Hotspot> = files_with_churn
            .iter()
            .filter_map(|file| {
                Self::calculate_hotspot_for_file(file, max_churn, max_complexity, max_size)
            })
            .collect();

        hotspots.sort_by(|a, b| {
            b.hotspot_score
                .partial_cmp(&a.hotspot_score)
                .unwrap_or(std::cmp::Ordering::Less)
        });

        hotspots.truncate(self.max_hotspots);

        HotspotAnalysis {
            hotspots,
            total_files_analyzed: files_with_churn.len(),
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CodeChurn;
    use std::path::PathBuf;
    use std::time::SystemTime;

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
    fn test_hotspot_detection() {
        let files = vec![
            create_test_file("high_churn.rs", 100, 10, 200, 150), // High churn
            create_test_file("complex.rs", 200, 50, 50, 30),      // High complexity
            create_test_file("large.rs", 500, 20, 100, 80),       // Large size
            create_test_file("low.rs", 50, 5, 10, 5),             // Low on all
        ];

        let detector = HotspotDetector::new(3);
        let analysis = detector.calculate(&files);

        assert_eq!(analysis.hotspots.len(), 3);
        assert_eq!(analysis.total_files_analyzed, 4);

        // First hotspot should have highest score
        assert!(analysis.hotspots[0].hotspot_score > analysis.hotspots[1].hotspot_score);
        assert!(analysis.hotspots[1].hotspot_score > analysis.hotspots[2].hotspot_score);
    }

    #[test]
    fn test_no_churn_data() {
        let files = vec![FileMetrics {
            path: PathBuf::from("no_churn.rs"),
            language: "rust".to_string(),
            loc: 100,
            size_bytes: 1024,
            function_count: 10,
            last_modified: SystemTime::now(),
            dead_function_count: None,
            dead_code_loc: None,
            dead_code_ratio: None,
            code_churn: None,
            coupling: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        }];

        let detector = HotspotDetector::new(10);
        let analysis = detector.calculate(&files);

        assert_eq!(analysis.hotspots.len(), 0);
        assert_eq!(analysis.total_files_analyzed, 0);
    }

    #[test]
    fn test_hotspot_limit() {
        let files = vec![
            create_test_file("a.rs", 100, 10, 100, 80),
            create_test_file("b.rs", 110, 11, 110, 85),
            create_test_file("c.rs", 120, 12, 120, 90),
            create_test_file("d.rs", 130, 13, 130, 95),
            create_test_file("e.rs", 140, 14, 140, 100),
        ];

        let detector = HotspotDetector::new(2); // Limit to top 2
        let analysis = detector.calculate(&files);

        assert_eq!(analysis.hotspots.len(), 2);
        assert_eq!(analysis.total_files_analyzed, 5);

        // Should return the top 2 highest scoring files
        assert!(analysis.hotspots[0].hotspot_score > analysis.hotspots[1].hotspot_score);
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
    fn test_zero_churn_file() {
        let files = vec![create_test_file("zero.rs", 100, 10, 0, 0)];

        let detector = HotspotDetector::new(10);
        let analysis = detector.calculate(&files);

        // File with zero churn should still be analyzed if it has other metrics
        assert_eq!(analysis.total_files_analyzed, 1);
    }

    #[test]
    fn test_hotspot_score_calculation() {
        let file = create_test_file("test.rs", 100, 10, 50, 30);

        let detector = HotspotDetector::new(10);
        let analysis = detector.calculate(&vec![file.clone()]);

        assert_eq!(analysis.hotspots.len(), 1);
        let hotspot = &analysis.hotspots[0];

        // Verify score components are calculated
        assert!(hotspot.hotspot_score > 0.0);
        assert_eq!(hotspot.path, PathBuf::from("test.rs"));
        assert_eq!(hotspot.total_changes, 80); // 50 + 30
        assert!(hotspot.churn_score > 0.0);
        assert!(hotspot.complexity_score > 0.0);
        assert!(hotspot.size_score > 0.0);
    }
}
