use crate::models::FileMetrics;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub struct DiskCache {
    path: PathBuf,
}

impl DiskCache {
    pub fn new(_path: PathBuf) -> Result<Self, CacheError> {
        todo!("Initialize cache directory")
    }

    pub fn get(&self, _file_path: &Path) -> Option<FileMetrics> {
        todo!("Retrieve metrics from cache")
    }

    pub fn set(&self, _metrics: &FileMetrics) -> Result<(), CacheError> {
        todo!("Store metrics in cache")
    }

    pub fn invalidate(&self, _file_path: &Path) -> Result<(), CacheError> {
        todo!("Invalidate cache entry")
    }
}

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Cache I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
}
