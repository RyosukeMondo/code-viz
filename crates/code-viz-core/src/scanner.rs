use globset::{Glob, GlobSetBuilder};
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

#[tracing::instrument(skip(exclude_patterns), fields(path = %path.display(), pattern_count = exclude_patterns.len()))]
pub fn scan_directory(
    path: &Path,
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>, ScanError> {
    tracing::info!("Starting directory scan");

    let mut builder = GlobSetBuilder::new();
    for pattern in exclude_patterns {
        builder.add(Glob::new(pattern).map_err(|e| {
            tracing::error!(pattern = %pattern, error = %e, "Invalid glob pattern");
            ScanError::InvalidPattern(e.to_string())
        })?);
    }
    let glob_set = builder
        .build()
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to build glob set");
            ScanError::InvalidPattern(e.to_string())
        })?;

    tracing::debug!("Glob patterns configured");

    let root_path = path.to_path_buf(); // Capture for closure

    let walker = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_entry(move |e| {
            if e.depth() == 0 {
                return true;
            }
            let path = e.path();
            // Skip hidden directories/files, but allow .ts/.js if they are hidden (unlikely but per spec)
            // Standard "skip hidden" logic:
            if is_hidden(e) {
                 // Exception: .ts/.js files
                 if let Some(ext) = path.extension() {
                     let ext_str = ext.to_string_lossy();
                     if ext_str == "ts" || ext_str == "js" {
                         return true; // Keep it
                     }
                 }
                 return false; // Skip hidden
            }

            // Check exclusions
            // globset matches against paths. We should match relative path if possible, or name.
            // Usually exclusions are like "node_modules/**".
            // Let's assume patterns match against the path.

            // Strip root prefix to match against relative patterns
            let relative_path = path.strip_prefix(&root_path).unwrap_or(path);
            !glob_set.is_match(relative_path)
        });

    let mut files = Vec::new();
    let mut skipped_large = 0;
    let mut skipped_permission = 0;

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    continue;
                }

                // Check file size > 10MB
                match entry.metadata() {
                    Ok(metadata) => {
                        if metadata.len() > 10 * 1024 * 1024 {
                            tracing::warn!(path = %path.display(), size_mb = metadata.len() / (1024 * 1024), "Skipping large file (>10MB)");
                            skipped_large += 1;
                            continue;
                        }
                    }
                    Err(e) => {
                         tracing::warn!(path = %path.display(), error = %e, "Failed to get metadata, skipping");
                         continue;
                    }
                }

                // Filter by extension
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy();
                    match ext_str.as_ref() {
                        "ts" | "tsx" | "js" | "jsx" | "rs" | "py" | "go" | "cpp" | "cc" | "cxx" | "hpp" | "h" => {
                             files.push(path.to_path_buf());
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                // Handle permission errors gracefully
                if let Some(io_err) = e.io_error() {
                    if io_err.kind() == std::io::ErrorKind::PermissionDenied {
                        tracing::warn!(error = %e, "Permission denied while scanning");
                        skipped_permission += 1;
                        continue;
                    }
                }
                // Other errors might be strictly IO or loops
                tracing::warn!(error = %e, "Error scanning entry");
            }
        }
    }

    files.sort();

    tracing::info!(
        files_found = files.len(),
        skipped_large = skipped_large,
        skipped_permission = skipped_permission,
        "Directory scan completed"
    );

    Ok(files)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "." && s != "..")
        .unwrap_or(false)
}

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),

    #[error("Access denied: {0}")]
    PermissionDenied(#[source] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_scan_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let result = scan_directory(temp_dir.path(), &[]).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_scan_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        
        File::create(root.join("test.rs")).unwrap();
        File::create(root.join("test.ts")).unwrap();
        File::create(root.join("ignore.txt")).unwrap(); // Should be ignored by extension

        let result = scan_directory(root, &[]).unwrap();
        assert_eq!(result.len(), 2);
        
        let file_names: Vec<_> = result.iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect();
        assert!(file_names.contains(&"test.rs"));
        assert!(file_names.contains(&"test.ts"));
    }

    #[test]
    fn test_scan_excludes_node_modules() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        let node_modules = root.join("node_modules");
        fs::create_dir(&node_modules).unwrap();
        File::create(node_modules.join("dep.ts")).unwrap();
        File::create(root.join("main.ts")).unwrap();

        let result = scan_directory(root, &["**/node_modules/**".to_string()]).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file_name().unwrap().to_str().unwrap(), "main.ts");
    }

    #[test]
    fn test_scan_excludes_custom_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        File::create(root.join("test.test.ts")).unwrap();
        File::create(root.join("main.ts")).unwrap();

        let result = scan_directory(root, &["**/*.test.ts".to_string()]).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file_name().unwrap().to_str().unwrap(), "main.ts");
    }

    #[test]
    fn test_scan_filters_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        File::create(root.join("style.css")).unwrap();
        File::create(root.join("script.js")).unwrap();
        File::create(root.join("readme.md")).unwrap();
        File::create(root.join("app.py")).unwrap();

        let result = scan_directory(root, &[]).unwrap();
        assert_eq!(result.len(), 2);
        let file_names: Vec<_> = result.iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect();
        assert!(file_names.contains(&"script.js"));
        assert!(file_names.contains(&"app.py"));
    }

    #[test]
    fn test_scan_hidden_files() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Hidden dir
        let hidden_dir = root.join(".hidden");
        fs::create_dir(&hidden_dir).unwrap();
        File::create(hidden_dir.join("secret.rs")).unwrap();

        // Hidden file
        File::create(root.join(".config.rs")).unwrap(); // Should be skipped? "skip hidden files"
        File::create(root.join("visible.rs")).unwrap();

        let result = scan_directory(root, &[]).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file_name().unwrap().to_str().unwrap(), "visible.rs");
    }

    #[test]
    #[cfg(unix)]
    fn test_scan_permission_denied() {
        use std::os::unix::fs::PermissionsExt;
        
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        
        let locked_dir = root.join("locked");
        fs::create_dir(&locked_dir).unwrap();
        File::create(locked_dir.join("secret.ts")).unwrap();
        
        // Remove read permissions
        let mut perms = fs::metadata(&locked_dir).unwrap().permissions();
        perms.set_mode(0o000);
        fs::set_permissions(&locked_dir, perms).unwrap();
        
        let result = scan_directory(root, &[]);
        
        // Restore permissions so tempdir cleanup works
        let mut perms = fs::metadata(&locked_dir).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&locked_dir, perms).unwrap();
        
        // It should not fail, just return empty list (or list without secret.ts)
        let files = result.unwrap();
        assert!(files.is_empty());
    }
}