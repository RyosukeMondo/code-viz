//! Tauri-specific type wrappers with specta support
//!
//! This module re-exports types from code-viz-api and adds Tauri-specific
//! features like TypeScript type generation via specta.

use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;
use std::time::SystemTime;

// Re-export the serialization functions from code-viz-api
pub use code_viz_api::models::{serialize_systemtime, deserialize_systemtime};

/// TreeNode with specta Type derive for TypeScript generation
///
/// This wraps code_viz_api::TreeNode and adds Tauri-specific annotations.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TreeNode {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub loc: usize,
    pub complexity: u32,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(default)]
    pub children: Vec<TreeNode>,
    #[serde(serialize_with = "serialize_systemtime", deserialize_with = "deserialize_systemtime")]
    #[specta(type = String)]
    pub last_modified: SystemTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_code_ratio: Option<f64>,
}

/// Convert from code_viz_api::TreeNode to Tauri TreeNode
impl From<code_viz_api::TreeNode> for TreeNode {
    fn from(api_node: code_viz_api::TreeNode) -> Self {
        Self {
            id: api_node.id,
            name: api_node.name,
            path: api_node.path,
            loc: api_node.loc,
            complexity: api_node.complexity,
            node_type: api_node.node_type,
            children: api_node.children.into_iter().map(Into::into).collect(),
            last_modified: api_node.last_modified,
            dead_code_ratio: api_node.dead_code_ratio,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test_conversion_from_api_tree_node() {
        let api_node = code_viz_api::TreeNode {
            id: "test.rs".to_string(),
            name: "test.rs".to_string(),
            path: PathBuf::from("test.rs"),
            loc: 100,
            complexity: 10,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1234567890),
            dead_code_ratio: None,
        };

        let tauri_node: TreeNode = api_node.into();

        assert_eq!(tauri_node.id, "test.rs");
        assert_eq!(tauri_node.loc, 100);
        assert_eq!(tauri_node.complexity, 10);
    }

    #[test]
    fn test_tauri_node_serialization() {
        let node = TreeNode {
            id: "test.rs".to_string(),
            name: "test.rs".to_string(),
            path: PathBuf::from("test.rs"),
            loc: 100,
            complexity: 10,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1234567890),
            dead_code_ratio: None,
        };

        let json = serde_json::to_value(&node).expect("Failed to serialize");

        // Verify lastModified is a string (not object)
        assert!(
            json["lastModified"].is_string(),
            "lastModified must be ISO 8601 string"
        );
    }
}
