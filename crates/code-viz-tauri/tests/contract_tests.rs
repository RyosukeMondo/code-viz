use thiserror::Error;

/// Errors that can occur during contract validation
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid value for field {0}: {1}")]
    InvalidValue(String, String),
    #[error("Schema mismatch: {0}")]
    SchemaMismatch(String),
}

mod helpers;

/// Tests for Specta schema validation
/// Ensures that the Rust types correctly generate the expected TypeScript schemas
mod specta_schema_tests {
    use code_viz_tauri::models::TreeNode;
    use specta_typescript as ts;

    #[test]
    fn test_validate_tree_node_schema() {
        // Extract schema for TreeNode by converting to TypeScript string
        // We allow BigInt as numbers for this test to match frontend expectations
        let ts_binding = ts::export::<TreeNode>(
            &ts::Typescript::default().bigint(ts::BigIntExportBehavior::Number)
        ).expect("Failed to export TreeNode to TS");

        assert!(ts_binding.contains("id: string"), "Missing 'id' field in TS schema");
        assert!(ts_binding.contains("name: string"), "Missing 'name' field in TS schema");
        assert!(ts_binding.contains("path: string"), "Missing 'path' field in TS schema");
        assert!(ts_binding.contains("loc: number"), "Missing 'loc' field in TS schema");
        assert!(ts_binding.contains("complexity: number"), "Missing 'complexity' field in TS schema");
        assert!(ts_binding.contains("type: string"), "Missing 'type' field in TS schema");
        // children is optional because of #[serde(default)] and Vec
        assert!(ts_binding.contains("children?: TreeNode[]"), "Missing 'children' field in TS schema");
        assert!(ts_binding.contains("lastModified: string"), "Missing 'lastModified' field in TS schema");
    }

    #[test]
    fn test_all_specta_types_coverage() {
        // This test ensures all types marked with #[specta::specta] are captured
        // and verify they can all be serialized to TypeScript.
        let config = ts::Typescript::default().bigint(ts::BigIntExportBehavior::Number);

        // Types from code-viz-tauri::models
        ts::export::<TreeNode>(&config).expect("TreeNode export failed");

        // Types from code-viz-dead-code (used in commands)
        use code_viz_dead_code::{DeadCodeResult, DeadCodeSummary, FileDeadCode, DeadSymbol};
        use code_viz_dead_code::models::SymbolKind;

        ts::export::<DeadCodeResult>(&config).expect("DeadCodeResult export failed");
        ts::export::<DeadCodeSummary>(&config).expect("DeadCodeSummary export failed");
        ts::export::<FileDeadCode>(&config).expect("FileDeadCode export failed");
        ts::export::<DeadSymbol>(&config).expect("DeadSymbol export failed");
        ts::export::<SymbolKind>(&config).expect("SymbolKind export failed");
    }
}

/// Tests for serialization round-trip validation
/// Ensures that data can be correctly serialized and deserialized via IPC
mod serialization_tests {
    use super::helpers::validation_utils;
    use code_viz_tauri::models::TreeNode;
    use serde_json;
    use std::path::PathBuf;

    #[test]
    fn test_tree_node_serialization_round_trip() {
        // Create test subject
        let original = validation_utils::create_test_tree();

        // Serialize to JSON Value
        let json_value = serde_json::to_value(&original).expect("Failed to serialize TreeNode");

        // Deserialize back to Rust
        let deserialized: TreeNode = serde_json::from_value(json_value).expect("Failed to deserialize TreeNode");

        // Verify equality
        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.path, deserialized.path);
        assert_eq!(original.loc, deserialized.loc);
        assert_eq!(original.complexity, deserialized.complexity);
        assert_eq!(original.node_type, deserialized.node_type);
        assert_eq!(original.dead_code_ratio, deserialized.dead_code_ratio);
        assert_eq!(original.children.len(), deserialized.children.len());
    }

    #[test]
    fn test_no_empty_string_paths() {
        // Create valid tree
        let valid_tree = validation_utils::create_test_tree();
        let json_valid = serde_json::to_value(&valid_tree).unwrap();
        
        // This should not panic
        validation_utils::assert_required_fields(&json_valid);

        // Create invalid tree with empty path
        let mut invalid_node = validation_utils::create_test_tree();
        invalid_node.path = PathBuf::from(""); // INVALID
        
        let json_invalid = serde_json::to_value(&invalid_node).unwrap();
        
        // This should fail (using catch_unwind because assert_required_fields panics)
        let result = std::panic::catch_unwind(|| {
            validation_utils::assert_required_fields(&json_invalid);
        });
        
        assert!(result.is_err(), "Validation should have failed for empty path");
    }

    #[test]
    fn test_recursive_children_validation() {
        let tree = validation_utils::create_test_tree();
        let json = serde_json::to_value(&tree).unwrap();

        // Use helper to recursively validate all fields
        validation_utils::assert_required_fields(&json);
        
        // Verify root node from helper data
        assert_eq!(json["name"], "root");
        assert_eq!(json["children"][0]["name"], "src");
        assert!(json["children"][0]["children"].is_array());
    }
}

/// Tests for ECharts compatibility validation
/// Ensures that the JSON output matches ECharts treemap requirements
mod echarts_compatibility_tests {
    use super::helpers::validation_utils;
    use serde_json::Value;

    #[test]
    fn test_echarts_treemap_format() {
        let tree = validation_utils::create_test_tree();
        let json = serde_json::to_value(&tree).expect("Failed to serialize TreeNode");

        // ECharts treemap core requirements (mapped by frontend)
        // Root node validation
        assert!(json["name"].is_string(), "ECharts requires 'name' as string");
        assert!(json["loc"].is_number(), "ECharts requires a value (we use 'loc')");
        assert!(json["children"].is_array(), "Treemap root must have 'children' array");
        
        // Verify that at least one child exists and has required ECharts properties
        let first_child = &json["children"][0];
        assert!(first_child["name"].is_string());
        assert!(first_child["loc"].is_number());
    }

    #[test]
    fn test_all_nodes_have_required_properties() {
        let tree = validation_utils::create_test_tree();
        let json = serde_json::to_value(&tree).expect("Failed to serialize TreeNode");

        // Recursively validate all nodes in the tree
        validate_echarts_properties_recursive(&json, 0);
    }

    fn validate_echarts_properties_recursive(node: &Value, depth: u32) {
        let node_name = node["name"].as_str().unwrap_or("unknown");
        
        // 1. Validate core ECharts treemap properties
        assert!(node.get("name").is_some(), "Node '{}' missing 'name' at depth {}", node_name, depth);
        assert!(node["name"].is_string(), "Node '{}' 'name' must be string", node_name);
        
        assert!(node.get("loc").is_some(), "Node '{}' missing 'loc' (ECharts value) at depth {}", node_name, depth);
        assert!(node["loc"].is_number(), "Node '{}' 'loc' must be number", node_name);

        // 2. Validate our Treemap component's required metadata
        assert!(node.get("path").is_some(), "Node '{}' missing 'path' at depth {}", node_name, depth);
        assert!(node["path"].is_string(), "Node '{}' 'path' must be string", node_name);
        
        assert!(node.get("type").is_some(), "Node '{}' missing 'type' at depth {}", node_name, depth);
        assert!(node["type"].is_string(), "Node '{}' 'type' must be string", node_name);

        // 3. Validate children structure if present
        if let Some(children) = node.get("children") {
            assert!(children.is_array(), "Node '{}' 'children' must be an array", node_name);
            for child in children.as_array().unwrap() {
                validate_echarts_properties_recursive(child, depth + 1);
            }
        }
        
        // Ensure directories have children (even if empty) to satisfy ECharts hierarchy
        if node["type"] == "directory" {
            assert!(node.get("children").is_some(), "Directory node '{}' must have 'children' field", node_name);
        }
    }
}
