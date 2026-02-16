//! Path handling utilities for tree transformation
//!
//! This module provides functions for working with file paths during the
//! transformation from flat file lists to hierarchical trees.

use crate::models::FileMetrics;
use std::path::{Path, PathBuf};

/// Finds the common root directory from a list of file paths
///
/// This function identifies the deepest common directory that contains all files.
/// Used to convert absolute filesystem paths to project-relative paths.
///
/// # Arguments
/// * `files` - Vector of file metrics with paths
///
/// # Returns
/// The common root directory path
///
/// # Examples
/// ```
/// // Files: /home/user/project/src/main.rs, /home/user/project/tests/test.rs
/// // Returns: /home/user/project
/// ```
pub fn find_common_root(files: &[FileMetrics]) -> PathBuf {
    if files.is_empty() {
        return PathBuf::from("/");
    }

    // Get the parent directories of all files
    let parents: Vec<PathBuf> = files
        .iter()
        .filter_map(|f| f.path.parent())
        .map(|p| p.to_path_buf())
        .collect();

    if parents.is_empty() {
        return PathBuf::from("/");
    }

    // Start with the first parent as the candidate
    let mut common = parents[0].clone();

    // Find the common ancestor of all parent directories
    for parent in parents.iter().skip(1) {
        // Walk up the tree until we find a common ancestor
        while !parent.starts_with(&common) && common != Path::new("/") {
            common = common.parent().unwrap_or(Path::new("/")).to_path_buf();
        }
    }

    // If all files have the same parent directory AND they're in a subdirectory
    // (not at root level), go up one more level to get the project root.
    // For example, if all files are in "src/", we want the parent of "src/" as root.
    let all_same_parent = parents.iter().all(|p| p == &common);
    if all_same_parent && parents.len() > 1 {
        // Multiple files in the same directory suggests it's a subdirectory
        common = common.parent().unwrap_or(&common).to_path_buf();
    }

    common
}

/// Strips a prefix from a path, returning a relative path
///
/// If the path doesn't start with the prefix, returns the path as-is.
///
/// # Arguments
/// * `path` - The absolute path to strip
/// * `prefix` - The prefix to remove
///
/// # Returns
/// A relative path with the prefix removed
pub fn strip_prefix(path: &Path, prefix: &Path) -> PathBuf {
    path.strip_prefix(prefix).unwrap_or(path).to_path_buf()
}

/// Gets the parent path of a given path, defaulting to root if no parent
///
/// # Arguments
/// * `path` - The path to get the parent of
/// * `root_path` - The root path to use as default
///
/// # Returns
/// The parent path, or root_path if no parent exists
pub fn get_parent_path(path: &Path, root_path: &Path) -> PathBuf {
    path.parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| root_path.to_path_buf())
}
