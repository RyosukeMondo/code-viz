//! Tauri wrapper library for code-viz analysis engine
//!
//! This crate provides Tauri command wrappers and data models for exposing
//! the code-viz-core analysis functionality to the frontend via IPC.

// Public modules
pub mod commands;
pub mod models;
pub mod transform;

// Re-export commonly used types
pub use commands::*;
pub use models::*;
pub use transform::*;
