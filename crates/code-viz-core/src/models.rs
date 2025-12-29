use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CouplingMetrics {
    /// Number of incoming dependencies (other modules that depend on this one)
    pub afferent_coupling: usize,
    /// Number of outgoing dependencies (modules this one depends on)
    pub efferent_coupling: usize,
    /// Instability metric (Efferent / (Afferent + Efferent))
    /// Ranges from 0 (stable) to 1 (unstable).
    pub instability: f64,
}

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

    /// Code churn metrics (only present when git analysis is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_churn: Option<CodeChurn>,

    /// Dependency coupling metrics (only present when coupling analysis is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupling: Option<CouplingMetrics>,

    /// AI Bloat Index: (comment_lines / code_lines) * 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_bloat_index: Option<f64>,
}

/// Represents code churn for a file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CodeChurn {
    pub added_lines: usize,
    pub deleted_lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Aggregated summary statistics
    pub summary: Summary,

    /// Per-file metrics
    pub files: Vec<FileMetrics>,

    /// When this analysis was performed
    pub timestamp: SystemTime,

    /// Results of code duplication analysis (if enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplication: Option<DuplicationAnalysis>,

    /// AI commit analysis (only present when enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_commit_analysis: Option<AICommitAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CodeLocation {
    pub path: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatePair {
    pub original: CodeLocation,
    pub duplicate: CodeLocation,
    pub similarity: f64,
    pub line_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicationAnalysis {
    pub pairs: Vec<DuplicatePair>,
    pub total_duplicated_loc: usize,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AICommitAnalysis {
    /// Total number of commits scanned
    pub total_commits: usize,

    /// Number of commits identified as AI-generated
    pub ai_generated_count: usize,

    /// Confidence score for each AI-generated commit (SHA, score)
    pub confidence_scores: Vec<(String, u8)>,

    /// List of patterns that were detected
    pub patterns_detected: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetricComparison {
    pub path: PathBuf,
    pub base: Option<FileMetrics>,
    pub head: Option<FileMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchComparison {
    pub files: Vec<FileMetricComparison>,
}
