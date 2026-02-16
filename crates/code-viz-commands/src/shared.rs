use anyhow::{Context, Result};
use code_viz_core::traits::FileSystem;
use std::path::{Path, PathBuf};

/// Scans a directory for supported file types and returns a filtered list.
pub fn scan_and_filter_files(fs: &impl FileSystem, path: &Path) -> Result<Vec<PathBuf>> {
    let all_files = fs
        .read_dir_recursive(path)
        .with_context(|| format!("Failed to scan directory: {}", path.display()))?;

    let supported_files: Vec<PathBuf> = all_files
        .into_iter()
        .filter(|p| {
            if let Some(ext) = p.extension() {
                let ext_str = ext.to_string_lossy();
                matches!(
                    ext_str.as_ref(),
                    "ts" | "tsx"
                        | "js"
                        | "jsx"
                        | "rs"
                        | "py"
                        | "go"
                        | "cpp"
                        | "cc"
                        | "cxx"
                        | "hpp"
                        | "h"
                )
            } else {
                false
            }
        })
        .collect();

    Ok(supported_files)
}
