//! Contract Testing - SSOT Enforcement
//!
//! This module provides compile-time and test-time validation that both
//! Tauri and Web implementations produce identical API contracts.
//!
//! # SSOT Guarantees
//!
//! 1. **Type Safety**: Shared types enforce identical structure
//! 2. **Serialization Contracts**: Tests verify identical JSON output
//! 3. **Handler Contracts**: Trait enforces identical behavior
//! 4. **Snapshot Tests**: Catch any serialization changes

use crate::models::TreeNode;
use code_viz_dead_code::DeadCodeResult;
use serde::{Deserialize, Serialize};

/// Request/Response types that BOTH Tauri and Web must use
///
/// These types define the API contract. Any changes here affect both implementations.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeRequest {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeResponse {
    pub tree: TreeNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeadCodeRequest {
    pub path: String,
    pub min_confidence: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeadCodeResponse {
    pub result: DeadCodeResult,
}

/// Contract validation - ensures both implementations produce identical JSON
#[cfg(test)]
pub mod test_utils {
    use super::*;
    use std::path::PathBuf;
    use std::time::SystemTime;

    pub use crate::transform::flat_to_hierarchy;

    // Re-export RealFileSystem for tests
    pub struct RealFileSystem;
    impl RealFileSystem {
        pub fn new() -> Self {
            Self
        }
    }

    impl code_viz_core::traits::FileSystem for RealFileSystem {
        fn read_dir_recursive(&self, path: &std::path::Path) -> anyhow::Result<Vec<PathBuf>> {
            let mut files = Vec::new();
            if path.is_dir() {
                for entry in std::fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        files.push(path);
                    } else if path.is_dir() {
                        files.extend(self.read_dir_recursive(&path)?);
                    }
                }
            }
            Ok(files)
        }

        fn read_to_string(&self, path: &std::path::Path) -> anyhow::Result<String> {
            Ok(std::fs::read_to_string(path)?)
        }

        fn write(&self, path: &std::path::Path, content: &str) -> anyhow::Result<()> {
            Ok(std::fs::write(path, content)?)
        }

        fn exists(&self, path: &std::path::Path) -> bool {
            path.exists()
        }
    }

    /// Create a test TreeNode with known values for contract testing
    pub fn create_test_tree() -> TreeNode {
        TreeNode {
            id: "test.rs".to_string(),
            name: "test.rs".to_string(),
            path: PathBuf::from("test.rs"),
            loc: 100,
            complexity: 10,
            node_type: "file".to_string(),
            children: vec![],
            last_modified: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1234567890),
            dead_code_ratio: None,
        }
    }

    /// Validate that a TreeNode serializes to valid JSON with correct structure
    pub fn validate_tree_node_contract(tree: &TreeNode) -> Result<(), String> {
        let json = serde_json::to_value(tree)
            .map_err(|e| format!("Failed to serialize TreeNode: {}", e))?;

        // Validate required fields exist and have correct types
        if !json["id"].is_string() {
            return Err(format!("id must be string, got: {:?}", json["id"]));
        }
        if !json["name"].is_string() {
            return Err(format!("name must be string, got: {:?}", json["name"]));
        }
        if !json["path"].is_string() {
            return Err(format!("path must be string, got: {:?}", json["path"]));
        }
        if !json["loc"].is_number() {
            return Err(format!("loc must be number, got: {:?}", json["loc"]));
        }
        if !json["complexity"].is_number() {
            return Err(format!("complexity must be number, got: {:?}", json["complexity"]));
        }
        if !json["type"].is_string() {
            return Err(format!("type must be string, got: {:?}", json["type"]));
        }
        if !json["children"].is_array() {
            return Err(format!("children must be array, got: {:?}", json["children"]));
        }

        // CRITICAL: lastModified must be ISO 8601 string, NOT object
        if !json["lastModified"].is_string() {
            return Err(format!(
                "lastModified must be ISO 8601 string, got: {:?}",
                json["lastModified"]
            ));
        }

        let timestamp = json["lastModified"].as_str().unwrap();
        if !timestamp.contains('T') || !timestamp.ends_with('Z') {
            return Err(format!(
                "lastModified must be ISO 8601 with Z suffix, got: {}",
                timestamp
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;

    #[test]
    fn test_analyze_request_serialization() {
        let request = AnalyzeRequest {
            path: "/test/path".to_string(),
            request_id: Some("test-123".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        insta::assert_snapshot!(json, @r###"{"path":"/test/path","requestId":"test-123"}"###);
    }

    #[test]
    fn test_analyze_response_contract() {
        let tree = create_test_tree();
        let response = AnalyzeResponse { tree: tree.clone() };

        // Validate contract
        validate_tree_node_contract(&response.tree).unwrap();

        // Snapshot test - catches any serialization changes
        let json = serde_json::to_value(&response).unwrap();
        insta::assert_json_snapshot!(json);
    }

    #[test]
    fn test_dead_code_request_serialization() {
        let request = DeadCodeRequest {
            path: "/test/path".to_string(),
            min_confidence: 80,
            request_id: Some("test-456".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        insta::assert_snapshot!(json, @r###"{"path":"/test/path","minConfidence":80,"requestId":"test-456"}"###);
    }

    /// CRITICAL: This test enforces SSOT - if JSON structure changes,
    /// both Tauri and Web must update together
    #[test]
    fn test_tree_node_json_contract() {
        let tree = create_test_tree();

        let json_str = serde_json::to_string_pretty(&tree).unwrap();

        // This snapshot MUST match for both Tauri and Web
        insta::assert_snapshot!(json_str);

        // Validate no raw SystemTime objects leak through
        assert!(
            !json_str.contains("secs_since_epoch"),
            "JSON must not contain raw SystemTime fields"
        );
        assert!(
            json_str.contains("lastModified"),
            "JSON must contain lastModified field"
        );
    }

    #[test]
    fn test_tree_node_roundtrip() {
        let original = create_test_tree();

        // Serialize
        let json_str = serde_json::to_string(&original).unwrap();

        // Deserialize
        let deserialized: TreeNode = serde_json::from_str(&json_str).unwrap();

        // Values should match
        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.loc, deserialized.loc);
        assert_eq!(original.complexity, deserialized.complexity);
    }
}
