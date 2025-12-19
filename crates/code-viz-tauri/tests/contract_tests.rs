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
    // TODO: Implement serialization round-trip validation tests
}

/// Tests for ECharts compatibility validation
/// Ensures that the JSON output matches ECharts treemap requirements
mod echarts_compatibility_tests {
    // TODO: Implement ECharts compatibility validation tests
}
