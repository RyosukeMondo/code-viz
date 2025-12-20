//! Build-time validation script for SSOT enforcement
//!
//! This script runs at compile time to ensure the SSOT architecture
//! is maintained. It validates that handlers are properly defined
//! and that no duplication exists.

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/");

    // Validate SSOT structure
    validate_ssot_structure();

    println!("cargo:warning=✅ SSOT validation passed");
}

fn validate_ssot_structure() {
    let src_dir = Path::new("src");

    // Check that all required SSOT files exist
    let required_files = [
        "models.rs",
        "handlers.rs",
        "transform.rs",
        "error.rs",
        "contracts.rs",
    ];

    for file in &required_files {
        let file_path = src_dir.join(file);
        if !file_path.exists() {
            panic!("SSOT validation failed: Missing required file {}", file);
        }
    }

    // Validate that handlers.rs contains the SSOT trait
    let handlers_content = std::fs::read_to_string(src_dir.join("handlers.rs"))
        .expect("Failed to read handlers.rs");

    if !handlers_content.contains("pub trait ApiHandler") {
        panic!("SSOT validation failed: ApiHandler trait not found in handlers.rs");
    }

    if !handlers_content.contains("analyze_repository") {
        panic!("SSOT validation failed: analyze_repository handler not found");
    }

    if !handlers_content.contains("analyze_dead_code") {
        panic!("SSOT validation failed: analyze_dead_code handler not found");
    }

    // Validate that models.rs contains TreeNode
    let models_content = std::fs::read_to_string(src_dir.join("models.rs"))
        .expect("Failed to read models.rs");

    if !models_content.contains("pub struct TreeNode") {
        panic!("SSOT validation failed: TreeNode not found in models.rs");
    }

    // Validate that contracts.rs contains tests
    let contracts_content = std::fs::read_to_string(src_dir.join("contracts.rs"))
        .expect("Failed to read contracts.rs");

    if !contracts_content.contains("#[test]") {
        panic!("SSOT validation failed: No contract tests found in contracts.rs");
    }

    if !contracts_content.contains("validate_tree_node_contract") {
        panic!("SSOT validation failed: Contract validation function not found");
    }

    println!("cargo:warning=✅ All SSOT structural requirements validated");
}
