use code_viz_tauri::models::TreeNode;
use serde_json::Value;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

/// Creates a sample TreeNode structure for testing
/// Returns a hierarchy: root -> src -> main.rs, utils.rs
pub fn create_test_tree() -> TreeNode {
    let main_rs = TreeNode {
        id: "src/main.rs".to_string(),
        name: "main.rs".to_string(),
        path: PathBuf::from("src/main.rs"),
        loc: 100,
        complexity: 10,
        node_type: "file".to_string(),
        children: vec![],
        last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1600000000),
        dead_code_ratio: Some(0.1),
    };

    let utils_rs = TreeNode {
        id: "src/utils.rs".to_string(),
        name: "utils.rs".to_string(),
        path: PathBuf::from("src/utils.rs"),
        loc: 50,
        complexity: 5,
        node_type: "file".to_string(),
        children: vec![],
        last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1600000000),
        dead_code_ratio: None,
    };

    let src_dir = TreeNode {
        id: "src".to_string(),
        name: "src".to_string(),
        path: PathBuf::from("src"),
        loc: 150,
        complexity: 15,
        node_type: "directory".to_string(),
        children: vec![main_rs, utils_rs],
        last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1600000000),
        dead_code_ratio: Some(0.05),
    };

    TreeNode {
        id: "root".to_string(),
        name: "root".to_string(),
        path: PathBuf::from("."),
        loc: 150,
        complexity: 15,
        node_type: "directory".to_string(),
        children: vec![src_dir],
        last_modified: UNIX_EPOCH + std::time::Duration::from_secs(1600000000),
        dead_code_ratio: None,
    }
}

/// Recursively asserts that all required fields are present and of correct type in the JSON representation
pub fn assert_required_fields(json: &Value) {
    // Check root node
    assert_node_required_fields(json);

    // Recursively check children if they exist
    if let Some(children) = json.get("children").and_then(|c| c.as_array()) {
        for child in children {
            assert_required_fields(child);
        }
    }
}

fn assert_node_required_fields(node: &Value) {
    // Check presence and type of required fields (camelCase as defined in #[serde(rename_all = "camelCase")])
    assert!(node.get("id").is_some(), "Missing 'id' field");
    assert!(node["id"].is_string(), "'id' must be a string");
    
    assert!(node.get("name").is_some(), "Missing 'name' field");
    assert!(node["name"].is_string(), "'name' must be a string");
    
    assert!(node.get("path").is_some(), "Missing 'path' field");
    // PathBuf serializes to string by default in most cases, but verify it
    assert!(node["path"].is_string(), "'path' must be a string");
    
    // Path must not be empty (regression test requirement)
    let path_str = node["path"].as_str().expect("path should be a string");
    assert!(!path_str.is_empty(), "Path must not be empty string. Node: {}", node["name"]);

    assert!(node.get("loc").is_some(), "Missing 'loc' field");
    assert!(node["loc"].is_number(), "'loc' must be a number");
    
    assert!(node.get("complexity").is_some(), "Missing 'complexity' field");
    assert!(node["complexity"].is_number(), "'complexity' must be a number");
    
    assert!(node.get("type").is_some(), "Missing 'type' field");
    assert!(node["type"].is_string(), "'type' must be a string");
    
    assert!(node.get("children").is_some(), "Missing 'children' field");
    assert!(node["children"].is_array(), "'children' must be an array");
    
    assert!(node.get("lastModified").is_some(), "Missing 'lastModified' field");
    assert!(node["lastModified"].is_string(), "'lastModified' must be a string");

    // deadCodeRatio is optional, but if present it must be a number
    if let Some(ratio) = node.get("deadCodeRatio") {
        assert!(ratio.is_number(), "'deadCodeRatio' must be a number if present");
    }
}
