use anyhow::{anyhow, Result};
use crate::traits::FileSystem;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Mock implementation of FileSystem for unit testing.
/// Uses an in-memory HashMap to store file contents and tracks all read operations.
#[derive(Clone, Default)]
pub struct MockFileSystem {
    files: Arc<Mutex<HashMap<PathBuf, String>>>,
    reads: Arc<Mutex<Vec<PathBuf>>>,
}

impl MockFileSystem {
    /// Create a new MockFileSystem.
    pub fn new() -> Self {
        Self {
            files: Arc::new(Mutex::new(HashMap::new())),
            reads: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a file to the mock filesystem.
    pub fn with_file(self, path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        self.files.lock().unwrap().insert(path.into(), content.into());
        self
    }

    /// Add multiple files to the mock filesystem.
    pub fn with_files(self, files: HashMap<PathBuf, String>) -> Self {
        self.files.lock().unwrap().extend(files);
        self
    }

    /// Get all paths that were read.
    pub fn get_reads(&self) -> Vec<PathBuf> {
        self.reads.lock().unwrap().clone()
    }

    /// Assert that a specific path was read.
    pub fn assert_read(&self, path: &Path) {
        let reads = self.get_reads();
        assert!(
            reads.iter().any(|p| p == path),
            "Expected path '{}' to be read, but it was not. Captured reads: {:?}",
            path.display(),
            reads
        );
    }
}

impl FileSystem for MockFileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        self.reads.lock().unwrap().push(path.to_path_buf());
        self.files.lock().unwrap().get(path)
            .cloned()
            .ok_or_else(|| anyhow!("File not found in mock filesystem: {}", path.display()))
    }

    fn read_dir_recursive(&self, path: &Path) -> Result<Vec<PathBuf>> {
        self.reads.lock().unwrap().push(path.to_path_buf());
        let files = self.files.lock().unwrap();
        let result: Vec<PathBuf> = files.keys()
            .filter(|p| p.starts_with(path))
            .cloned()
            .collect();
        Ok(result)
    }

    fn write(&self, path: &Path, content: &str) -> Result<()> {
        self.files.lock().unwrap().insert(path.to_path_buf(), content.to_string());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.lock().unwrap().contains_key(path)
    }
}
