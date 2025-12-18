//! Symbol graph caching using sled embedded database.
//!
//! This module handles persisting and loading symbol graphs to/from
//! disk for incremental analysis. The cache stores symbol graphs
//! with file hashes for invalidation.

use crate::symbol_graph::SymbolGraph;
use ahash::AHashMap as HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use thiserror::Error;

/// Error type for cache operations
#[derive(Debug, Error)]
pub enum CacheError {
    /// Failed to open sled database
    #[error("Failed to open cache database: {0}")]
    DatabaseOpen(String),

    /// Failed to serialize/deserialize data
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Cache is corrupted
    #[error("Cache is corrupted and will be rebuilt")]
    Corrupted,
}

/// Current cache schema version
const CACHE_VERSION: u32 = 1;

/// Cached symbol graph with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedSymbolGraph {
    /// Schema version for migration
    pub version: u32,

    /// When this cache was created
    pub timestamp: SystemTime,

    /// The cached symbol graph
    pub graph: SymbolGraph,

    /// File hashes for invalidation (file path -> hash)
    pub file_hashes: HashMap<PathBuf, u64>,
}

/// Symbol graph cache using sled embedded database
pub struct SymbolGraphCache {
    /// Sled database handle
    db: sled::Db,
    /// Cache directory path
    cache_dir: PathBuf,
}

impl SymbolGraphCache {
    /// Create a new symbol graph cache
    ///
    /// # Arguments
    /// * `cache_dir` - Directory where cache database will be stored
    ///
    /// # Returns
    /// New cache instance or error if database cannot be opened
    pub fn new(cache_dir: &Path) -> Result<Self, CacheError> {
        // Create cache directory if it doesn't exist
        fs::create_dir_all(cache_dir)?;

        let db_path = cache_dir.join("symbols.db");

        // Try to open sled database, delete and recreate if corrupted
        let db = match sled::open(&db_path) {
            Ok(db) => db,
            Err(e) => {
                tracing::warn!("Cache database corrupted, rebuilding: {}", e);
                // Delete corrupted database
                let _ = fs::remove_dir_all(&db_path);
                // Try to open again
                sled::open(&db_path).map_err(|e| {
                    CacheError::DatabaseOpen(format!("Failed to open cache after cleanup: {}", e))
                })?
            }
        };

        Ok(Self {
            db,
            cache_dir: cache_dir.to_path_buf(),
        })
    }

    /// Save symbol graph to cache
    ///
    /// # Arguments
    /// * `graph` - The symbol graph to save
    ///
    /// # Returns
    /// Ok if saved successfully
    pub fn save(&self, graph: &SymbolGraph) -> Result<(), CacheError> {
        // Calculate file hashes from the graph
        let mut file_hashes = HashMap::new();
        for path in graph.exports.keys() {
            if let Ok(metadata) = fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    // Use modification time as hash (simple but effective)
                    let hash = modified
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    file_hashes.insert(path.clone(), hash);
                }
            }
        }

        let cached = CachedSymbolGraph {
            version: CACHE_VERSION,
            timestamp: SystemTime::now(),
            graph: graph.clone(),
            file_hashes,
        };

        // Serialize with bincode
        let bytes =
            bincode::serialize(&cached).map_err(|e| CacheError::Serialization(e.to_string()))?;

        // Store in sled with a known key
        self.db
            .insert(b"symbol_graph", bytes)
            .map_err(|e| CacheError::DatabaseOpen(e.to_string()))?;

        // Flush to ensure data is persisted
        self.db
            .flush()
            .map_err(|e| CacheError::DatabaseOpen(e.to_string()))?;

        Ok(())
    }

    /// Load symbol graph from cache
    ///
    /// # Returns
    /// Cached symbol graph if available and valid, None otherwise
    pub fn load(&self) -> Result<Option<SymbolGraph>, CacheError> {
        // Try to get from database
        let value = match self.db.get(b"symbol_graph") {
            Ok(Some(v)) => v,
            Ok(None) => return Ok(None),
            Err(e) => {
                tracing::warn!("Failed to read from cache: {}", e);
                return Ok(None);
            }
        };

        // Deserialize
        let cached: CachedSymbolGraph = match bincode::deserialize(&value) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Failed to deserialize cache, will rebuild: {}", e);
                // Clear corrupted cache
                let _ = self.db.remove(b"symbol_graph");
                return Ok(None);
            }
        };

        // Check version
        if cached.version != CACHE_VERSION {
            tracing::info!(
                "Cache version mismatch (got {}, expected {}), rebuilding",
                cached.version,
                CACHE_VERSION
            );
            let _ = self.db.remove(b"symbol_graph");
            return Ok(None);
        }

        Ok(Some(cached.graph))
    }

    /// Check if cache is stale and invalidate if needed
    ///
    /// Compares file modification times with cached hashes.
    ///
    /// # Arguments
    /// * `files` - List of files to check
    ///
    /// # Returns
    /// True if cache was invalidated (is stale)
    pub fn invalidate_if_stale(&self, files: &[PathBuf]) -> Result<bool, CacheError> {
        // Get cached data
        let value = match self.db.get(b"symbol_graph") {
            Ok(Some(v)) => v,
            Ok(None) => return Ok(true), // No cache, consider it stale
            Err(_) => return Ok(true),   // Error reading, consider it stale
        };

        let cached: CachedSymbolGraph = match bincode::deserialize(&value) {
            Ok(c) => c,
            Err(_) => {
                // Corrupted cache, invalidate
                let _ = self.db.remove(b"symbol_graph");
                return Ok(true);
            }
        };

        // Check version first
        if cached.version != CACHE_VERSION {
            let _ = self.db.remove(b"symbol_graph");
            return Ok(true);
        }

        // Check if any file has been modified
        for file in files {
            if let Ok(metadata) = fs::metadata(file) {
                if let Ok(modified) = metadata.modified() {
                    let current_hash = modified
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    // Compare with cached hash
                    match cached.file_hashes.get(file) {
                        Some(&cached_hash) if cached_hash == current_hash => {
                            // File hasn't changed
                            continue;
                        }
                        _ => {
                            // File changed or not in cache - invalidate
                            tracing::debug!("File changed, invalidating cache: {:?}", file);
                            let _ = self.db.remove(b"symbol_graph");
                            return Ok(true);
                        }
                    }
                }
            } else {
                // File doesn't exist anymore - invalidate
                tracing::debug!("File missing, invalidating cache: {:?}", file);
                let _ = self.db.remove(b"symbol_graph");
                return Ok(true);
            }
        }

        // Also check if there are files in cache that are no longer being analyzed
        // This happens when files are deleted from the project
        let current_files: std::collections::HashSet<_> = files.iter().collect();
        for cached_file in cached.file_hashes.keys() {
            if !current_files.contains(cached_file) {
                tracing::debug!(
                    "File removed from analysis, invalidating cache: {:?}",
                    cached_file
                );
                let _ = self.db.remove(b"symbol_graph");
                return Ok(true);
            }
        }

        // Cache is valid
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Symbol, SymbolId, SymbolKind};
    use ahash::AHashMap as HashMap;
    use tempfile::TempDir;

    fn create_test_graph() -> SymbolGraph {
        let mut symbols = HashMap::new();
        let mut imports = HashMap::new();
        let mut exports = HashMap::new();

        let symbol_id: SymbolId = "test.ts:1".to_string();
        let symbol = Symbol {
            id: symbol_id.clone(),
            name: "testFunction".to_string(),
            kind: SymbolKind::Function,
            path: PathBuf::from("test.ts"),
            line_start: 1,
            line_end: 5,
            is_exported: true,
            is_test: false,
        };

        symbols.insert(symbol_id.clone(), symbol);
        imports.insert(symbol_id.clone(), vec![]);
        exports.insert(PathBuf::from("test.ts"), vec![symbol_id]);

        SymbolGraph {
            symbols,
            imports,
            exports,
        }
    }

    #[test]
    fn test_cache_new_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");

        let _cache = SymbolGraphCache::new(&cache_dir).unwrap();
        assert!(cache_dir.exists());
    }

    #[test]
    fn test_cache_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");

        let cache = SymbolGraphCache::new(&cache_dir).unwrap();
        let graph = create_test_graph();

        // Save graph
        cache.save(&graph).unwrap();

        // Load graph
        let loaded = cache.load().unwrap();
        assert!(loaded.is_some());

        let loaded_graph = loaded.unwrap();
        assert_eq!(loaded_graph.symbols.len(), graph.symbols.len());
        assert_eq!(loaded_graph.imports.len(), graph.imports.len());
        assert_eq!(loaded_graph.exports.len(), graph.exports.len());

        // Verify symbol data
        let symbol_id = "test.ts:1";
        assert!(loaded_graph.symbols.contains_key(symbol_id));
        let loaded_symbol = &loaded_graph.symbols[symbol_id];
        assert_eq!(loaded_symbol.name, "testFunction");
        assert_eq!(loaded_symbol.kind, SymbolKind::Function);
    }

    #[test]
    fn test_cache_load_empty() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");

        let cache = SymbolGraphCache::new(&cache_dir).unwrap();

        // Load from empty cache
        let loaded = cache.load().unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_cache_invalidate_empty() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");

        let cache = SymbolGraphCache::new(&cache_dir).unwrap();

        // Invalidate with no cache should return true (stale)
        let is_stale = cache
            .invalidate_if_stale(&[PathBuf::from("test.ts")])
            .unwrap();
        assert!(is_stale);
    }

    #[test]
    fn test_cache_invalidate_after_save() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");

        let cache = SymbolGraphCache::new(&cache_dir).unwrap();
        let graph = create_test_graph();

        // Save graph
        cache.save(&graph).unwrap();

        // Check if stale with same files (should not be stale)
        let files: Vec<PathBuf> = graph.exports.keys().cloned().collect();
        let is_stale = cache.invalidate_if_stale(&files).unwrap();

        // Note: This will be true because the files in the graph don't exist on disk
        // In a real scenario with actual files, this would be false
        assert!(is_stale);
    }

    #[test]
    fn test_cache_version_mismatch() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");

        let cache = SymbolGraphCache::new(&cache_dir).unwrap();

        // Create a cached graph with wrong version
        let graph = create_test_graph();
        let cached = CachedSymbolGraph {
            version: CACHE_VERSION + 1, // Wrong version
            timestamp: SystemTime::now(),
            graph,
            file_hashes: HashMap::new(),
        };

        let bytes = bincode::serialize(&cached).unwrap();
        cache.db.insert(b"symbol_graph", bytes).unwrap();

        // Load should return None due to version mismatch
        let loaded = cache.load().unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_cache_corrupted_data() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");

        let cache = SymbolGraphCache::new(&cache_dir).unwrap();

        // Insert corrupted data
        cache.db.insert(b"symbol_graph", b"corrupted data").unwrap();

        // Load should return None and clear corrupted data
        let loaded = cache.load().unwrap();
        assert!(loaded.is_none());

        // Cache should be cleared
        assert!(cache.db.get(b"symbol_graph").unwrap().is_none());
    }

    #[test]
    fn test_cache_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");

        let graph = create_test_graph();

        // Save in one cache instance
        {
            let cache = SymbolGraphCache::new(&cache_dir).unwrap();
            cache.save(&graph).unwrap();
        }

        // Load in another cache instance
        {
            let cache = SymbolGraphCache::new(&cache_dir).unwrap();
            let loaded = cache.load().unwrap();
            assert!(loaded.is_some());

            let loaded_graph = loaded.unwrap();
            assert_eq!(loaded_graph.symbols.len(), 1);
        }
    }
}
