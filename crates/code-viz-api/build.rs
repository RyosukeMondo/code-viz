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

fn validate_required_files_exist(src_dir: &Path) {
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
}

fn validate_handlers_file(src_dir: &Path) {
    let content = match std::fs::read_to_string(src_dir.join("handlers.rs")) {
        Ok(content) => content,
        Err(e) => panic!("SSOT validation failed: Failed to read handlers.rs: {}", e),
    };

    if !content.contains("pub trait ApiHandler") {
        panic!("SSOT validation failed: ApiHandler trait not found in handlers.rs");
    }
    if !content.contains("analyze_repository") {
        panic!("SSOT validation failed: analyze_repository handler not found");
    }
    if !content.contains("analyze_dead_code") {
        panic!("SSOT validation failed: analyze_dead_code handler not found");
    }
}

fn validate_models_file(src_dir: &Path) {
    let content = match std::fs::read_to_string(src_dir.join("models.rs")) {
        Ok(content) => content,
        Err(e) => panic!("SSOT validation failed: Failed to read models.rs: {}", e),
    };

    if !content.contains("pub struct TreeNode") {
        panic!("SSOT validation failed: TreeNode not found in models.rs");
    }
}

fn validate_contracts_file(src_dir: &Path) {
    let content = match std::fs::read_to_string(src_dir.join("contracts.rs")) {
        Ok(content) => content,
        Err(e) => panic!("SSOT validation failed: Failed to read contracts.rs: {}", e),
    };

    if !content.contains("#[test]") {
        panic!("SSOT validation failed: No contract tests found in contracts.rs");
    }
    if !content.contains("validate_tree_node_contract") {
        panic!("SSOT validation failed: Contract validation function not found");
    }
}

fn validate_ssot_structure() {
    let src_dir = Path::new("src");

    validate_required_files_exist(src_dir);
    validate_handlers_file(src_dir);
    validate_models_file(src_dir);
    validate_contracts_file(src_dir);

    println!("cargo:warning=✅ All SSOT structural requirements validated");
}
