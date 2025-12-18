use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileMetrics {
    /// Relative path from repository root
    pub path: PathBuf,

    /// Programming language ("rust", "typescript", "python", etc.)
    pub language: String,

    /// Lines of code (excluding comments and blank lines)
    pub loc: usize,

    /// File size in bytes
    pub size_bytes: u64,

    /// Number of functions/methods
    pub function_count: usize,

    /// Last modified timestamp (for cache invalidation)
    pub last_modified: SystemTime,

    /// Number of dead functions (only present when dead code analysis enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_function_count: Option<usize>,

    /// Lines of dead code (only present when dead code analysis enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_code_loc: Option<usize>,

    /// Ratio of dead code to total code (only present when dead code analysis enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_code_ratio: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Aggregated summary statistics
    pub summary: Summary,

    /// Per-file metrics
    pub files: Vec<FileMetrics>,

    /// When this analysis was performed
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    /// Total number of files analyzed
    pub total_files: usize,

    /// Total lines of code across all files
    pub total_loc: usize,

    /// Total functions across all files
    pub total_functions: usize,

    /// Top 10 largest files by LOC (sorted descending)
    pub largest_files: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Glob patterns to exclude (e.g., "node_modules/**")
    pub exclude_patterns: Vec<String>,

    /// Whether to use disk cache for unchanged files
    pub use_cache: bool,

    /// Enable dead code analysis (default: false)
    pub enable_dead_code: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            exclude_patterns: vec![
                "node_modules/**".into(),
                "target/**".into(),
                ".git/**".into(),
                "dist/**".into(),
                "build/**".into(),
            ],
            use_cache: true,
            enable_dead_code: false,
        }
    }
}
