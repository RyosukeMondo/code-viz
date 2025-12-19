use anyhow::Result;
use std::path::{Path, PathBuf};

/// FileSystem abstracts file I/O operations to enable testing and platform-specific implementations.
pub trait FileSystem: Send + Sync {
    /// Read the entire contents of a file into a string.
    fn read_to_string(&self, path: &Path) -> Result<String>;

    /// Read all files in a directory recursively.
    fn read_dir_recursive(&self, path: &Path) -> Result<Vec<PathBuf>>;

    /// Write content to a file.
    fn write(&self, path: &Path, content: &str) -> Result<()>;

    /// Check if a path exists.
    fn exists(&self, path: &Path) -> bool;
}
