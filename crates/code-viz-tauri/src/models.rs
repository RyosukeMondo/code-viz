//! Data models for visualization
//!
//! This module defines the TreeNode structure used for hierarchical
//! visualization of code metrics in the frontend.

use serde::{Deserialize, Serialize, Serializer};
use specta::Type;
use std::path::PathBuf;
use std::time::SystemTime;

/// Serialize SystemTime as ISO 8601 string
fn serialize_systemtime<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let datetime: chrono::DateTime<chrono::Utc> = (*time).into();
    serializer.serialize_str(&datetime.to_rfc3339())
}

/// Hierarchical node representing a file or directory in the codebase tree
///
/// This structure extends the core FileMetrics with hierarchical relationships
/// and visualization-specific fields for rendering in the treemap UI.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TreeNode {
    /// Unique identifier for this node (typically the full path)
    pub id: String,

    /// Display name (file/directory name without full path)
    pub name: String,

    /// Full path from repository root
    pub path: PathBuf,

    /// Lines of code (for files) or sum of children (for directories)
    pub loc: usize,

    /// Complexity score (0-100 scale, calculated as loc/10 for MVP)
    /// This is a placeholder metric; future versions may use cyclomatic complexity
    pub complexity: u32,

    /// Node type: "file" or "directory"
    #[serde(rename = "type")]
    pub node_type: String,

    /// Child nodes (empty for files, contains children for directories)
    #[serde(default)]
    pub children: Vec<TreeNode>,

    /// Last modified timestamp (for cache invalidation and sorting)
    #[serde(serialize_with = "serialize_systemtime")]
    pub last_modified: SystemTime,

    /// Dead code ratio (0.0 to 1.0), only present when dead code analysis is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_code_ratio: Option<f64>,
}
