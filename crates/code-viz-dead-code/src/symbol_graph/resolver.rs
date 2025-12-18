//! Import path resolution for symbol graph construction.

use ahash::AHashMap as HashMap;
use std::path::{Path, PathBuf};

/// Resolve an import path relative to the importing file
///
/// Handles:
/// - Relative imports: "./utils" -> "../src/utils.ts"
/// - Package imports: "@/utils" or "~/utils" (TypeScript path aliases)
/// - Extension-less imports: "./utils" could be "./utils.ts" or "./utils/index.ts"
pub(super) fn resolve_import_path(
    importer_path: &Path,
    import_source: &str,
    available_files: &HashMap<PathBuf, bool>,
) -> Option<PathBuf> {
    // Remove quotes from import source
    let import_source = import_source.trim_matches(|c| c == '"' || c == '\'');

    // Skip node_modules and package imports (e.g., "react", "lodash")
    if !import_source.starts_with('.')
        && !import_source.starts_with('/')
        && !import_source.starts_with("@/")
        && !import_source.starts_with("~/")
    {
        return None;
    }

    // Get the directory of the importing file
    let importer_dir = importer_path.parent()?;

    // Handle TypeScript path aliases (@/ and ~/ typically map to src/)
    let import_path_str = if import_source.starts_with("@/") || import_source.starts_with("~/") {
        import_source[2..].to_string()
    } else {
        import_source.to_string()
    };

    // Resolve the path relative to the importer
    let base_path = if import_path_str.starts_with("./") || import_path_str.starts_with("../") {
        importer_dir.join(&import_path_str)
    } else {
        // Assume path alias points to project root (simplified)
        PathBuf::from(&import_path_str)
    };

    // Try to resolve with common extensions
    let extensions = ["", ".ts", ".tsx", ".js", ".jsx"];
    for ext in &extensions {
        let candidate = if ext.is_empty() {
            base_path.clone()
        } else {
            base_path.with_extension(&ext[1..]) // Remove the leading dot
        };

        if available_files.contains_key(&candidate) {
            return Some(candidate);
        }
    }

    // Try index file resolution (import "./dir" -> "./dir/index.ts")
    for ext in &[".ts", ".tsx", ".js", ".jsx"] {
        let index_path = base_path.join(format!("index{}", ext));
        if available_files.contains_key(&index_path) {
            return Some(index_path);
        }
    }

    // Log warning for unresolved import but don't fail
    None
}
