use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

#[cfg(feature = "specta")]
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "specta", derive(Type))]
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

    /// Cognitive complexity metrics (per-function and file-level aggregation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognitive_complexity: Option<CognitiveComplexity>,

    /// Test coverage metrics (from llvm-cov, tarpaulin, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_coverage: Option<TestCoverage>,
}

/// Represents code churn for a file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "specta", derive(Type))]
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

    /// Git hotspot analysis (only present when enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotspot_analysis: Option<HotspotAnalysis>,

    /// Test coverage analysis (only present when coverage report provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_analysis: Option<CoverageAnalysis>,
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

/// Represents a hotspot file (high churn + complexity + size)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotspot {
    pub path: PathBuf,
    pub hotspot_score: f64,
    pub churn_score: f64,
    pub complexity_score: f64,
    pub size_score: f64,
    pub total_changes: usize,
}

/// Results of git hotspot analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotspotAnalysis {
    /// Top hotspots sorted by score (descending)
    pub hotspots: Vec<Hotspot>,
    /// Total files analyzed for hotspots
    pub total_files_analyzed: usize,
}

/// Cognitive complexity for a single function
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "specta", derive(Type))]
pub struct FunctionComplexity {
    pub name: String,
    pub complexity: usize,
    pub start_line: usize,
    pub end_line: usize,
}

/// Cognitive complexity metrics for a file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "specta", derive(Type))]
pub struct CognitiveComplexity {
    /// Total cognitive complexity for the file
    pub total_complexity: usize,
    /// Average complexity per function
    pub average_complexity: f64,
    /// Maximum complexity among all functions
    pub max_complexity: usize,
    /// Per-function complexity details
    pub functions: Vec<FunctionComplexity>,
}

/// Test coverage metrics for a file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "specta", derive(Type))]
pub struct TestCoverage {
    /// Line coverage percentage (0.0 - 100.0)
    pub line_coverage: f64,
    /// Number of lines covered
    pub lines_covered: usize,
    /// Total number of executable lines
    pub total_lines: usize,
    /// Branch coverage percentage (0.0 - 100.0), if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_coverage: Option<f64>,
}

/// Aggregated test coverage analysis for the entire codebase
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoverageAnalysis {
    /// Overall line coverage percentage
    pub overall_coverage: f64,
    /// Total lines covered across all files
    pub total_lines_covered: usize,
    /// Total executable lines across all files
    pub total_executable_lines: usize,
    /// Files with coverage data
    pub file_count: usize,
}

/// Hierarchical node representing a file or directory in the codebase tree
///
/// This structure is used by the transform module to build hierarchical
/// visualizations from flat file metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(Type))]
pub struct TreeNode {
    /// Unique identifier for this node (typically the full path)
    pub id: String,

    /// Display name (file/directory name without full path)
    pub name: String,

    /// Full path from repository root
    pub path: PathBuf,

    /// Lines of code (for files) or sum of children (for directories)
    pub loc: usize,

    /// Complexity score (0-100 scale, calculated as loc/10)
    pub complexity: u32,

    /// Node type: "file" or "directory"
    pub node_type: String,

    /// Child nodes (empty for files, contains children for directories)
    pub children: Vec<TreeNode>,

    /// Last modified timestamp
    pub last_modified: SystemTime,

    /// Dead code ratio (0.0 to 1.0), only present when dead code analysis is enabled
    pub dead_code_ratio: Option<f64>,

    /// Programming language - only for files
    pub language: Option<String>,

    /// File size in bytes - only for files
    pub size_bytes: Option<u64>,

    /// Number of functions/methods - only for files
    pub function_count: Option<usize>,

    /// Coupling metrics
    pub coupling: Option<CouplingMetrics>,

    /// Code churn metrics
    pub code_churn: Option<CodeChurn>,

    /// AI Bloat Index: (comment_lines / code_lines) * 100
    pub ai_bloat_index: Option<f64>,

    /// Cognitive complexity metrics
    pub cognitive_complexity: Option<CognitiveComplexity>,

    /// Test coverage metrics
    pub test_coverage: Option<TestCoverage>,
}
