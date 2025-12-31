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

/// AnalysisOptions with specta Type derive for TypeScript generation
///
/// This wraps code_viz_api::AnalysisOptions and adds Tauri-specific annotations.
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisOptions {
    #[serde(default)]
    pub enable_duplicates: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duplicate_lines: Option<usize>,
    #[serde(default)]
    pub enable_hotspots: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hotspots: Option<usize>,
    #[serde(default)]
    pub enable_ai_commits: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_report_path: Option<String>,
}

/// Convert from Tauri AnalysisOptions to API AnalysisOptions
impl From<AnalysisOptions> for code_viz_api::AnalysisOptions {
    fn from(tauri_options: AnalysisOptions) -> Self {
        Self {
            enable_duplicates: tauri_options.enable_duplicates,
            min_duplicate_lines: tauri_options.min_duplicate_lines,
            enable_hotspots: tauri_options.enable_hotspots,
            max_hotspots: tauri_options.max_hotspots,
            enable_ai_commits: tauri_options.enable_ai_commits,
            coverage_report_path: tauri_options.coverage_report_path,
        }
    }
}

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
    // Additional metrics from FileMetrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupling: Option<code_viz_core::models::CouplingMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_churn: Option<code_viz_core::models::CodeChurn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_bloat_index: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognitive_complexity: Option<code_viz_core::models::CognitiveComplexity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_coverage: Option<code_viz_core::models::TestCoverage>,
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
            language: api_node.language,
            size_bytes: api_node.size_bytes,
            function_count: api_node.function_count,
            coupling: api_node.coupling,
            code_churn: api_node.code_churn,
            ai_bloat_index: api_node.ai_bloat_index,
            cognitive_complexity: api_node.cognitive_complexity,
            test_coverage: api_node.test_coverage,
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
            language: None,
            size_bytes: None,
            function_count: None,
            coupling: None,
            code_churn: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
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
            language: None,
            size_bytes: None,
            function_count: None,
            coupling: None,
            code_churn: None,
            ai_bloat_index: None,
            cognitive_complexity: None,
            test_coverage: None,
        };

        // Test-only expect: Serialization failure in tests indicates fixture/setup issues
        let json = serde_json::to_value(&node).expect("Failed to serialize");

        // Verify lastModified is a string (not object)
        assert!(
            json["lastModified"].is_string(),
            "lastModified must be ISO 8601 string"
        );
    }
}
