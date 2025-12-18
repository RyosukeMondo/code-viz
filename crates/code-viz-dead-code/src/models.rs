//! Core data structures for dead code analysis
//!
//! This module defines the primary data models used throughout the dead code
//! detection pipeline.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Configuration for dead code analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Minimum confidence score to report (0-100)
    pub min_confidence: u8,
    /// Patterns to exclude from analysis
    pub exclude_patterns: Vec<String>,
    /// Enable incremental analysis using cache
    pub enable_cache: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0,
            exclude_patterns: vec!["node_modules/**".to_string(), "dist/**".to_string()],
            enable_cache: true,
        }
    }
}

/// Result of dead code analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadCodeResult {
    /// Summary statistics
    pub summary: DeadCodeSummary,
    /// Dead code grouped by file
    pub files: Vec<FileDeadCode>,
}

/// Summary statistics for dead code analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadCodeSummary {
    /// Total number of dead functions
    pub total_dead_functions: usize,
    /// Total lines of dead code
    pub dead_code_loc: usize,
    /// Percentage of dead code (0.0-1.0)
    pub dead_code_ratio: f64,
    /// Number of files with dead code
    pub files_with_dead_code: usize,
}

/// Dead code information for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDeadCode {
    /// File path
    pub path: PathBuf,
    /// Dead symbols in this file
    pub dead_symbols: Vec<DeadSymbol>,
    /// Total lines of dead code in this file
    pub dead_code_loc: usize,
}

/// A dead symbol (function, class, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadSymbol {
    /// Symbol name
    pub name: String,
    /// Symbol type (function, class, method, etc.)
    pub kind: String,
    /// Starting line number
    pub line_start: usize,
    /// Ending line number
    pub line_end: usize,
    /// Confidence score (0-100)
    pub confidence: u8,
    /// Reason for marking as dead
    pub reason: String,
}

/// Errors that can occur during analysis
#[derive(Debug, Error)]
pub enum AnalysisError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Graph construction error: {0}")]
    Graph(String),

    #[error("Cache error: {0}")]
    Cache(String),
}
