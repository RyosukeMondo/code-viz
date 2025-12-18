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
#[cfg_attr(feature = "specta", derive(specta::Type))]
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
    ///
    /// Returns a new `DeadCodeResult` containing only dead symbols
    /// with confidence >= `min_confidence`.
    ///
    /// # Arguments
    ///
    /// * `min_confidence` - Minimum confidence score (0-100)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use code_viz_dead_code::DeadCodeResult;
    /// # let result: DeadCodeResult = unimplemented!();
    /// // Only show high-confidence dead code (90% or higher)
    /// let high_confidence = result.filter_by_confidence(90);
    /// ```
    pub fn filter_by_confidence(&self, min_confidence: u8) -> Self {
        let mut filtered_files = Vec::new();
        let mut dead_functions = 0;
        let mut dead_classes = 0;
        let mut total_dead_loc = 0;

        for file in &self.files {
            let filtered_symbols: Vec<DeadSymbol> = file.dead_code
                .iter()
                .filter(|symbol| symbol.confidence >= min_confidence)
                .cloned()
                .collect();

            if !filtered_symbols.is_empty() {
                // Update counters
                for symbol in &filtered_symbols {
                    total_dead_loc += symbol.loc;
                    match symbol.kind {
                        SymbolKind::Function | SymbolKind::ArrowFunction | SymbolKind::Method => {
                            dead_functions += 1;
                        }
                        SymbolKind::Class => {
                            dead_classes += 1;
                        }
                        _ => {}
                    }
                }

                filtered_files.push(FileDeadCode {
                    path: file.path.clone(),
                    dead_code: filtered_symbols,
                });
            }
        }

        let files_with_dead_code = filtered_files.len();

        // Recalculate ratio based on original total
        let dead_code_ratio = if self.summary.total_dead_loc > 0 {
            total_dead_loc as f64 / self.summary.total_dead_loc as f64 * self.summary.dead_code_ratio
        } else {
            0.0
        };

        DeadCodeResult {
            summary: DeadCodeSummary {
                total_files: self.summary.total_files,
                files_with_dead_code,
                dead_functions,
                dead_classes,
                total_dead_loc,
                dead_code_ratio,
            },
            files: filtered_files,
        }
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
