//! Data models for visualization
//!
//! This module defines the TreeNode structure used for hierarchical
//! visualization of code metrics in the frontend.

use serde::{Deserialize, Serialize, Serializer};
use specta::Type;
use std::path::PathBuf;
use std::time::SystemTime;

/// Serialize SystemTime as ISO 8601 string with Z suffix for UTC
fn serialize_systemtime<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let datetime: chrono::DateTime<chrono::Utc> = (*time).into();
    // Use true parameter to force Z suffix instead of +00:00
    serializer.serialize_str(&datetime.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
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
    #[specta(type = String)]
    pub last_modified: SystemTime,

    /// Dead code ratio (0.0 to 1.0), only present when dead code analysis is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_code_ratio: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test_treenode_serialization_format() {
        // Create a TreeNode with known values
        let node = TreeNode {
            id: "test.ts".to_string(),
            name: "test.ts".to_string(),
            path: PathBuf::from("test.ts"),
            loc: 100,
            complexity: 10,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1234567890),
            dead_code_ratio: None,
        };

        // Debug: Print the actual JSON to see PathBuf serialization
        let json_str = serde_json::to_string_pretty(&node).expect("Failed to serialize");
        println!("Serialized JSON:\n{}", json_str);

        // Serialize to JSON
        let json = serde_json::to_value(&node).expect("Failed to serialize TreeNode");

        // CRITICAL: Verify lastModified is a string, not an object
        assert!(
            json["lastModified"].is_string(),
            "lastModified must be serialized as ISO 8601 string, not object. Got: {:?}",
            json["lastModified"]
        );

        // Verify it's a valid ISO 8601 timestamp
        let timestamp_str = json["lastModified"].as_str().unwrap();
        assert!(
            timestamp_str.ends_with('Z'),
            "Timestamp must be in UTC (end with Z): {}",
            timestamp_str
        );
        assert!(
            timestamp_str.contains('T'),
            "Timestamp must be ISO 8601 format (contain T): {}",
            timestamp_str
        );

        // Verify other fields are correct types
        assert_eq!(json["loc"].as_u64().unwrap(), 100);
        assert_eq!(json["complexity"].as_u64().unwrap(), 10);
        assert_eq!(json["type"].as_str().unwrap(), "file");
        assert_eq!(json["name"].as_str().unwrap(), "test.ts");
    }

    #[test]
    fn test_treenode_with_children_serialization() {
        let child = TreeNode {
            id: "child.ts".to_string(),
            name: "child.ts".to_string(),
            path: PathBuf::from("src/child.ts"),
            loc: 50,
            complexity: 5,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1234567890),
            dead_code_ratio: Some(0.15),
        };

        let parent = TreeNode {
            id: "src".to_string(),
            name: "src".to_string(),
            path: PathBuf::from("src"),
            loc: 50,
            complexity: 5,
            node_type: "directory".to_string(),
            children: vec![child],
            last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1234567890),
            dead_code_ratio: Some(0.15),
        };

        let json = serde_json::to_value(&parent).expect("Failed to serialize");

        // Verify parent timestamp
        assert!(json["lastModified"].is_string());

        // Verify child timestamp (nested)
        assert!(json["children"][0]["lastModified"].is_string());

        // Verify deadCodeRatio is present and correct
        assert_eq!(json["deadCodeRatio"].as_f64().unwrap(), 0.15);
        assert_eq!(json["children"][0]["deadCodeRatio"].as_f64().unwrap(), 0.15);
    }

    #[test]
    fn test_treenode_roundtrip_serialization() {
        let original = TreeNode {
            id: "test.ts".to_string(),
            name: "test.ts".to_string(),
            path: PathBuf::from("test.ts"),
            loc: 100,
            complexity: 10,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1234567890),
            dead_code_ratio: Some(0.25),
        };

        // Serialize
        let json_str = serde_json::to_string(&original).expect("Failed to serialize");

        // Verify JSON contains ISO timestamp string (not object)
        assert!(
            !json_str.contains("secs_since_epoch"),
            "Serialized JSON should not contain raw SystemTime fields: {}",
            json_str
        );
        assert!(
            json_str.contains("lastModified"),
            "Serialized JSON must contain lastModified field"
        );

        // Verify it looks like ISO 8601
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        let timestamp = json["lastModified"].as_str().unwrap();
        assert!(
            timestamp.len() >= 20,
            "ISO 8601 timestamp should be at least 20 chars: {}",
            timestamp
        );
    }

    #[test]
    fn test_dead_code_ratio_optional() {
        let without_dead_code = TreeNode {
            id: "test.ts".to_string(),
            name: "test.ts".to_string(),
            path: PathBuf::from("test.ts"),
            loc: 100,
            complexity: 10,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: SystemTime::now(),
            dead_code_ratio: None,
        };

        let json = serde_json::to_value(&without_dead_code).unwrap();

        // When None, deadCodeRatio should be omitted (not null)
        assert!(
            json.get("deadCodeRatio").is_none(),
            "deadCodeRatio should be omitted when None, not serialized as null"
        );
    }
}
