//! Tree node building utilities
//!
//! This module provides functions for creating tree nodes and managing the
//! directory hierarchy during transformation.

use crate::models::{FileMetrics, TreeNode};
use super::path_utils::{get_parent_path, strip_prefix};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Creates an empty root TreeNode for empty file lists
pub fn create_empty_root() -> TreeNode {
    TreeNode {
        id: "/".to_string(),
        name: "root".to_string(),
        path: PathBuf::from("/"),
        loc: 0,
        complexity: 0,
        node_type: "directory".to_string(),
        children: vec![],
        last_modified: SystemTime::now(),
        dead_code_ratio: None,
        language: None,
        size_bytes: None,
        function_count: None,
        coupling: None,
        code_churn: None,
        ai_bloat_index: None,
        cognitive_complexity: None,
        test_coverage: None,
    }
}

/// Creates a root node with a specific project name
pub fn create_root_node(project_name: &str, root_node_path: &Path) -> TreeNode {
    TreeNode {
        id: "/".to_string(),
        name: project_name.to_string(),
        path: root_node_path.to_path_buf(),
        loc: 0,
        complexity: 0,
        node_type: "directory".to_string(),
        children: vec![],
        last_modified: SystemTime::now(),
        dead_code_ratio: None,
        language: None,
        size_bytes: None,
        function_count: None,
        coupling: None,
        code_churn: None,
        ai_bloat_index: None,
        cognitive_complexity: None,
        test_coverage: None,
    }
}

/// Builds initial directory map with root node
pub fn build_directory_map(project_name: &str, root_node_path: &Path) -> HashMap<PathBuf, TreeNode> {
    let mut dir_map = HashMap::new();
    let root_node = create_root_node(project_name, root_node_path);
    dir_map.insert(root_node_path.to_path_buf(), root_node);
    dir_map
}

/// Calculates complexity score from LOC (placeholder: loc/10, capped at 100)
pub fn calculate_complexity(loc: usize) -> u32 {
    ((loc / 10) as u32).min(100)
}

/// Creates a file node from FileMetrics
pub fn create_file_node(file: &FileMetrics, file_path: &Path) -> TreeNode {
    let file_loc = file.loc;
    let file_complexity = calculate_complexity(file_loc);
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    TreeNode {
        id: file_path.to_string_lossy().to_string(),
        name: file_name,
        path: file_path.to_path_buf(),
        loc: file_loc,
        complexity: file_complexity,
        node_type: "file".to_string(),
        children: vec![],
        last_modified: file.last_modified,
        dead_code_ratio: file.dead_code_ratio,
        language: Some(file.language.clone()),
        size_bytes: Some(file.size_bytes),
        function_count: Some(file.function_count),
        coupling: file.coupling.clone(),
        code_churn: file.code_churn.clone(),
        ai_bloat_index: file.ai_bloat_index,
        cognitive_complexity: file.cognitive_complexity.clone(),
        test_coverage: file.test_coverage.clone(),
    }
}

/// Creates a directory node
pub fn create_directory_node(path: &Path) -> TreeNode {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    TreeNode {
        id: path.to_string_lossy().to_string(),
        name,
        path: path.to_path_buf(),
        loc: 0,
        complexity: 0,
        node_type: "directory".to_string(),
        children: vec![],
        last_modified: SystemTime::now(),
        dead_code_ratio: None,
        language: None,
        size_bytes: None,
        function_count: None,
        coupling: None,
        code_churn: None,
        ai_bloat_index: None,
        cognitive_complexity: None,
        test_coverage: None,
    }
}

/// Builds file nodes from metrics and ensures parent directories exist
pub fn build_file_nodes(
    files: Vec<FileMetrics>,
    has_absolute_paths: bool,
    root_path: &Path,
    dir_map: &mut HashMap<PathBuf, TreeNode>,
    root_node_path: &Path,
) -> Vec<(PathBuf, TreeNode)> {
    let mut file_nodes = Vec::new();
    for file in files {
        let file_path = if has_absolute_paths {
            strip_prefix(&file.path, root_path)
        } else {
            file.path.clone()
        };

        let file_node = create_file_node(&file, &file_path);
        file_nodes.push((file_path.clone(), file_node));
        ensure_parent_directories(&file_path, dir_map, root_node_path);
    }
    file_nodes
}

/// Ensures all parent directories exist in the directory map
pub fn ensure_parent_directories(
    file_path: &Path,
    dir_map: &mut HashMap<PathBuf, TreeNode>,
    root_path: &Path,
) {
    let mut current = file_path.to_path_buf();

    // Walk up the directory tree, creating missing directories
    while let Some(parent) = current.parent() {
        if parent.as_os_str().is_empty() || parent == root_path {
            break;
        }

        let parent_buf = parent.to_path_buf();
        if !dir_map.contains_key(&parent_buf) {
            let dir_node = create_directory_node(&parent_buf);
            dir_map.insert(parent_buf.clone(), dir_node);

            // Ensure this directory's parent exists
            ensure_parent_in_tree(&parent_buf, dir_map, root_path);
        }
        current = parent_buf;
    }
}

/// Ensures a directory node is attached to its parent
pub fn ensure_parent_in_tree(
    dir_path: &Path,
    dir_map: &mut HashMap<PathBuf, TreeNode>,
    root_path: &Path,
) {
    // Don't try to attach root to itself
    if dir_path == root_path {
        return;
    }

    let parent_path = get_parent_path(dir_path, root_path);

    // Ensure parent directory exists
    if !dir_map.contains_key(&parent_path) {
        ensure_parent_directories(dir_path, dir_map, root_path);
    }
}

/// Attaches file nodes to their parent directories
pub fn attach_file_nodes_to_parents(
    file_nodes: Vec<(PathBuf, TreeNode)>,
    dir_map: &mut HashMap<PathBuf, TreeNode>,
    root_node_path: &Path,
) {
    for (file_path, file_node) in file_nodes {
        let parent_path = get_parent_path(&file_path, root_node_path);
        if let Some(parent) = dir_map.get_mut(&parent_path) {
            parent.children.push(file_node);
        }
    }
}
