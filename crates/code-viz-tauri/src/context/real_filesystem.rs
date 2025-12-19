use anyhow::{Context, Result};
use code_viz_core::traits::FileSystem;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Production implementation of FileSystem that delegates to std::fs and walkdir.
pub struct RealFileSystem;

impl RealFileSystem {
    /// Create a new RealFileSystem instance.
    pub fn new() -> Self {
        Self
    }
}

impl FileSystem for RealFileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))
    }

    fn read_dir_recursive(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
        Ok(files)
    }

    fn write(&self, path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
        fs::write(path, content)
            .with_context(|| format!("Failed to write to file: {}", path.display()))
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}
