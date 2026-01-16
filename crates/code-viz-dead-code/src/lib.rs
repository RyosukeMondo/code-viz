//! Dead Code Detection and Analysis Engine
//!
//! This crate provides semantic reachability analysis using symbol graphs to identify
//! unused functions, classes, and modules across a codebase. It builds a symbol graph
//! from import/export relationships and performs depth-first traversal from entry points
//! to identify unreachable (dead) code.
//!
//! # Architecture
//!
//! The analysis pipeline consists of the following stages:
//!
//! 1. **Symbol Extraction**: Parse source files using Tree-sitter to extract symbols
//!    (functions, classes, imports, exports)
//! 2. **Graph Building**: Construct a bidirectional dependency graph from symbols
//! 3. **Entry Point Detection**: Identify entry points (main files, exports, tests)
//! 4. **Reachability Analysis**: DFS traversal from entry points to mark reachable symbols
//! 5. **Dead Code Identification**: Symbols not reached = dead code
//! 6. **Confidence Scoring**: Calculate deletion confidence based on heuristics
//!
//! # Example
//!
//! ```rust,no_run
//! use code_viz_dead_code::analyze_dead_code;
//! use std::path::Path;
//!
//! // Analyze a TypeScript project for dead code
//! let result = analyze_dead_code(Path::new("./src"), None)?;
//! println!("Found {} dead symbols", result.summary.dead_functions);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![allow(dead_code)]

pub mod analyzer;
pub mod cache;
pub mod confidence;
pub mod entry_points;
pub mod graph_manager;
pub mod models;
pub mod orchestrator;
pub mod reachability;
pub mod result_aggregator;
pub mod symbol_graph;

// Re-export main types for convenience
pub use models::{DeadCodeResult, DeadCodeSummary, DeadSymbol, FileDeadCode};

pub use analyzer::ReachabilityAnalyzer as DeadCodeAnalyzer;
pub use cache::{CacheError, SymbolGraphCache};
pub use confidence::ConfidenceCalculator;
pub use entry_points::detect_entry_points;
pub use graph_manager::GraphManager;
pub use orchestrator::DeadCodeOrchestrator;
pub use reachability::{ReachabilityAnalyzer, ReachabilityError};
pub use result_aggregator::ResultAggregator;
pub use symbol_graph::{GraphError, SymbolGraph, SymbolGraphBuilder};

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Configuration options for dead code analysis
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Patterns to exclude from analysis (glob patterns)
    pub exclude_patterns: Vec<String>,

    /// Enable caching for incremental analysis
    pub enable_cache: bool,

    /// Cache directory path (defaults to .code-viz/cache)
    pub cache_dir: Option<PathBuf>,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            exclude_patterns: vec![
                "node_modules/**".to_string(),
                "dist/**".to_string(),
                "build/**".to_string(),
                ".git/**".to_string(),
            ],
            enable_cache: true,
            cache_dir: None,
        }
    }
}

/// Error type for analysis operations
#[derive(Debug, Error)]
pub enum AnalysisError {
    /// Failed to scan directory
    #[error("Directory scan failed: {0}")]
    ScanError(#[from] code_viz_core::scanner::ScanError),

    /// Symbol graph construction failed
    #[error("Symbol graph construction failed: {0}")]
    GraphError(#[from] symbol_graph::GraphError),

    /// Reachability analysis failed
    #[error("Reachability analysis failed: {0}")]
    ReachabilityError(#[from] reachability::ReachabilityError),

    /// Cache operation failed
    #[error("Cache operation failed: {0}")]
    CacheError(#[from] cache::CacheError),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// No entry points found
    #[error("No entry points found in the codebase")]
    NoEntryPoints,
}

/// Main entry point for dead code analysis
///
/// This function orchestrates the full analysis pipeline:
/// 1. Scans the directory for source files
/// 2. Builds or loads cached symbol graph
/// 3. Detects entry points
/// 4. Performs reachability analysis
/// 5. Calculates confidence scores
/// 6. Returns aggregated results
///
/// # Arguments
///
/// * `path` - Root directory to analyze
/// * `config` - Analysis configuration options (uses defaults if None)
///
/// # Returns
///
/// A `DeadCodeResult` containing the summary and per-file dead code details
///
/// # Example
///
/// ```rust,no_run
/// use code_viz_dead_code::{analyze_dead_code, AnalysisConfig};
/// use std::path::Path;
///
/// let result = analyze_dead_code(Path::new("./src"), None)?;
/// println!("Found {} dead symbols", result.summary.dead_functions);
/// # Ok::<(), code_viz_dead_code::AnalysisError>(())
/// ```
#[tracing::instrument(skip(config), fields(path = %path.display()))]
pub fn analyze_dead_code(
    path: &Path,
    config: Option<AnalysisConfig>,
) -> Result<DeadCodeResult, AnalysisError> {
    let config = config.unwrap_or_default();
    let orchestrator = orchestrator::DeadCodeOrchestrator::new(path, config);
    orchestrator.analyze()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    //! Integration tests for dead code analysis
    //!
    //! Note: unwrap() is acceptable in test code because test panics indicate
    //! test fixture or assertion failures, which is the expected behavior.

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn setup_test_files(src_dir: &std::path::Path) {
        // Create main.ts (entry point)
        fs::write(
            src_dir.join("main.ts"),
            r#"
import { usedFunction } from './used';

function main() {
    usedFunction();
}

main();
"#,
        )
        .unwrap();

        // Create used.ts (partially used)
        fs::write(
            src_dir.join("used.ts"),
            r#"
export function usedFunction() {
    console.log('I am used');
}

export function deadFunction() {
    console.log('I am never called');
}
"#,
        )
        .unwrap();

        // Create dead.ts (completely unused)
        fs::write(
            src_dir.join("dead.ts"),
            r#"
export function completelyUnused() {
    console.log('Nobody uses me');
}

export class UnusedClass {
    method() {
        console.log('Never called');
    }
}
"#,
        )
        .unwrap();
    }

    #[test]
    fn test_analyze_dead_code_integration() {
        // Create temporary directory with test files
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();

        setup_test_files(&src_dir);

        // Run analysis
        let result = analyze_dead_code(&src_dir, None).unwrap();

        // Verify results
        assert!(result.summary.total_files > 0, "Should analyze files");
        assert!(
            result.summary.files_with_dead_code > 0,
            "Should find dead code"
        );
        assert!(
            result.summary.dead_functions >= 2,
            "Should find at least 2 dead functions"
        );
    }

    #[test]
    fn test_filter_by_confidence() {
        let result = DeadCodeResult {
            summary: DeadCodeSummary {
                total_files: 2,
                files_with_dead_code: 2,
                dead_functions: 3,
                dead_classes: 0,
                total_dead_loc: 30,
                dead_code_ratio: 0.5,
            },
            files: vec![FileDeadCode {
                path: PathBuf::from("test.ts"),
                dead_code: vec![
                    DeadSymbol {
                        symbol: "highConfidence".to_string(),
                        kind: models::SymbolKind::Function,
                        line_start: 1,
                        line_end: 10,
                        loc: 10,
                        confidence: 95,
                        reason: "Test".to_string(),
                        last_modified: None,
                    },
                    DeadSymbol {
                        symbol: "lowConfidence".to_string(),
                        kind: models::SymbolKind::Function,
                        line_start: 11,
                        line_end: 20,
                        loc: 10,
                        confidence: 50,
                        reason: "Test".to_string(),
                        last_modified: None,
                    },
                ],
            }],
        };

        let filtered = result.filter_by_confidence(80);

        assert_eq!(filtered.summary.dead_functions, 1);
        assert_eq!(filtered.summary.total_dead_loc, 10);
        assert_eq!(filtered.files.len(), 1);
        assert_eq!(filtered.files[0].dead_code.len(), 1);
        assert_eq!(filtered.files[0].dead_code[0].symbol, "highConfidence");
    }
}
