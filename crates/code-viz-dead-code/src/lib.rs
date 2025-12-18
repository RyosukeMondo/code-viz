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
//! let result = analyze_dead_code(Path::new("./src"))?;
//! println!("Found {} dead symbols", result.summary.dead_functions);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![allow(dead_code)]

pub mod symbol_graph;
pub mod reachability;
pub mod confidence;
pub mod entry_points;
pub mod cache;
pub mod models;

// Re-export main types for convenience
pub use models::{
    DeadCodeResult, DeadCodeSummary, FileDeadCode, DeadSymbol,
};

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
/// * `config` - Analysis configuration options
///
/// # Returns
///
/// A `DeadCodeResult` containing the summary and per-file dead code details
///
/// # Example
///
/// ```rust,no_run
/// use code_viz_dead_code::analyze_dead_code;
/// use std::path::Path;
///
/// let result = analyze_dead_code(Path::new("./src"))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn analyze_dead_code(
    _path: &std::path::Path,
) -> Result<DeadCodeResult, String> {
    todo!("Will implement in task 3.1.1")
}
