use anyhow::{Context, Result};
use crate::traits::FileSystem;
use crate::scanner::scan_directory;
use std::fs;
use std::path::{Path, PathBuf};

/// Production implementation of FileSystem that delegates to std::fs and walkdir.
#[derive(Clone, Copy)]
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
        // Use scan_directory which respects .gitignore files
        // No additional exclude patterns (empty array)
        scan_directory(path, &[])
            .map_err(|e| anyhow::anyhow!("Failed to scan directory: {}", e))
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
