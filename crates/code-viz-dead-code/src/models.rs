//! Core data models for dead code analysis.
//!
//! This module defines the data structures used throughout the dead code
//! detection pipeline, including symbol representations, analysis results,
//! and summary statistics.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// Unique identifier for a symbol (typically file path + line number)
pub type SymbolId = String;

/// A symbol extracted from source code (function, class, variable, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Symbol {
    /// Unique identifier for this symbol
    pub id: SymbolId,

    /// Symbol name (e.g., "handleClick", "UserService")
    pub name: String,

    /// Type of symbol
    pub kind: SymbolKind,

    /// File path where symbol is defined
    pub path: PathBuf,

    /// Starting line number (1-indexed)
    pub line_start: usize,

    /// Ending line number (1-indexed)
    pub line_end: usize,

    /// Whether symbol is exported from its module
    pub is_exported: bool,

    /// Whether symbol is in a test file
    pub is_test: bool,
}

/// Type of symbol
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SymbolKind {
    /// Regular function declaration
    Function,

    /// Arrow function or function expression
    ArrowFunction,

    /// Class declaration
    Class,

    /// Method within a class
    Method,

    /// Variable or constant
    Variable,
}

/// Complete result of dead code analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DeadCodeResult {
    /// Aggregated summary statistics
    pub summary: DeadCodeSummary,

    /// Dead code grouped by file
    pub files: Vec<FileDeadCode>,
}

impl DeadCodeResult {
    /// Filter dead code by minimum confidence score
    pub fn filter_by_confidence(&self, _min_confidence: u8) -> Self {
        todo!("Will implement in task 3.1.1")
    }
}

/// Summary statistics for dead code analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DeadCodeSummary {
    /// Total number of files analyzed
    pub total_files: usize,

    /// Number of files containing dead code
    pub files_with_dead_code: usize,

    /// Total number of dead functions
    pub dead_functions: usize,

    /// Total number of dead classes
    pub dead_classes: usize,

    /// Total lines of dead code
    pub total_dead_loc: usize,

    /// Ratio of dead code to total code (0.0 to 1.0)
    pub dead_code_ratio: f64,
}

/// Dead code found in a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct FileDeadCode {
    /// File path
    pub path: PathBuf,

    /// List of dead symbols in this file
    pub dead_code: Vec<DeadSymbol>,
}

/// A dead (unreachable) symbol with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DeadSymbol {
    /// Symbol name
    pub symbol: String,

    /// Type of symbol
    pub kind: SymbolKind,

    /// Starting line number
    pub line_start: usize,

    /// Ending line number
    pub line_end: usize,

    /// Lines of code in this symbol
    pub loc: usize,

    /// Deletion confidence score (0-100)
    pub confidence: u8,

    /// Reason why this symbol is marked as dead
    pub reason: String,

    /// Last modification time (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<SystemTime>,
}
