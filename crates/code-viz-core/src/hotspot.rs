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

    /// Calculate hotspot analysis from file metrics
    /// Requires files to have code_churn data populated
    pub fn calculate(&self, files: &[FileMetrics]) -> HotspotAnalysis {
        // Filter files that have churn data
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

        // Find max values for normalization
        let max_churn = files_with_churn
            .iter()
            .map(|f| {
                let churn = f.code_churn.as_ref().unwrap();
                churn.added_lines + churn.deleted_lines
            })
            .max()
            .unwrap_or(1) as f64;

        let max_complexity = files_with_churn
            .iter()
            .map(|f| f.function_count)
            .max()
            .unwrap_or(1) as f64;

        let max_size = files_with_churn
            .iter()
            .map(|f| f.loc)
            .max()
            .unwrap_or(1) as f64;

        // Calculate hotspot scores
        let mut hotspots: Vec<Hotspot> = files_with_churn
            .iter()
            .map(|file| {
                let churn = file.code_churn.as_ref().unwrap();
                let total_changes = churn.added_lines + churn.deleted_lines;

                // Normalize scores to 0-1 range
                let churn_score = if max_churn > 0.0 {
                    total_changes as f64 / max_churn
                } else {
                    0.0
                };

                let complexity_score = if max_complexity > 0.0 {
                    file.function_count as f64 / max_complexity
                } else {
                    0.0
                };

                let size_score = if max_size > 0.0 {
                    file.loc as f64 / max_size
                } else {
                    0.0
                };

                // Calculate weighted hotspot score
                let hotspot_score = (churn_score * CHURN_WEIGHT)
                    + (complexity_score * COMPLEXITY_WEIGHT)
                    + (size_score * SIZE_WEIGHT);

                Hotspot {
                    path: file.path.clone(),
                    hotspot_score,
                    churn_score,
                    complexity_score,
                    size_score,
                    total_changes,
                }
            })
            .collect();

        // Sort by hotspot score descending
        hotspots.sort_by(|a, b| b.hotspot_score.partial_cmp(&a.hotspot_score).unwrap());

        // Take top N
        hotspots.truncate(self.max_hotspots);

        HotspotAnalysis {
            hotspots,
            total_files_analyzed: files_with_churn.len(),
        }
    }
}

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
}
