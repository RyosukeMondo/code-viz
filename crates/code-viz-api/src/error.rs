//! Shared error types for API handlers
//!
//! This module defines the error types used across both Tauri and Web implementations.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("Dead code analysis failed: {0}")]
    DeadCodeFailed(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ApiError {
    /// Convert to a user-friendly error message string
    pub fn to_user_message(&self) -> String {
        match self {
            ApiError::AnalysisFailed(msg) => format!("Analysis failed: {}", msg),
            ApiError::DeadCodeFailed(msg) => format!("Dead code analysis failed: {}", msg),
            ApiError::InvalidPath(msg) => format!("Invalid path: {}", msg),
            ApiError::Io(e) => format!("File system error: {}", e),
            ApiError::Internal(e) => format!("Internal error: {}", e),
        }
    }
}
