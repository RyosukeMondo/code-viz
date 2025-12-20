use globset::{Glob, GlobSetBuilder};
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[tracing::instrument(skip(exclude_patterns), fields(path = %path.display(), pattern_count = exclude_patterns.len()))]
pub fn scan_directory(
    path: &Path,
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>, ScanError> {
    tracing::info!("Starting directory scan");

    if !path.exists() {
        return Err(ScanError::NotFound(path.to_path_buf()));
    }
    if !path.is_dir() {
        return Err(ScanError::NotADirectory(path.to_path_buf()));
    }

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

    // Use ignore::WalkBuilder which respects .gitignore, .ignore, etc.
    let walker = WalkBuilder::new(path)
        .follow_links(false)
        .git_ignore(true) // Respect .gitignore files in git repos
        .git_global(true) // Respect global gitignore
        .git_exclude(true) // Respect .git/info/exclude
        .add_custom_ignore_filename(".gitignore") // Also respect .gitignore in non-git dirs
        .hidden(true) // Skip hidden files/dirs
        .build()
        .filter_map(|result| result.ok()) // Skip errors, log them separately
        .filter(move |entry| {
            let path = entry.path();

            // Allow root directory
            if entry.depth() == 0 {
                return true;
            }

            // Check additional exclude patterns (on top of gitignore)
            let relative_path = path.strip_prefix(&root_path).unwrap_or(path);
            if glob_set.is_match(relative_path) {
                return false;
            }

            true
        });

    let mut files = Vec::new();
    let mut skipped_large = 0;
    let mut skipped_permission = 0;

    for entry in walker {
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Check file size > 10MB
        // Use std::fs::metadata directly since ignore::DirEntry might not have metadata cached
        match std::fs::metadata(path) {
            Ok(metadata) => {
                if metadata.len() > 10 * 1024 * 1024 {
                    tracing::warn!(
                        path = %path.display(),
                        size_mb = metadata.len() / (1024 * 1024),
                        "Skipping large file (>10MB)"
                    );
                    skipped_large += 1;
                    continue;
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    tracing::warn!(path = %path.display(), "Permission denied");
                    skipped_permission += 1;
                } else {
                    tracing::warn!(path = %path.display(), error = %e, "Failed to get metadata");
                }
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

    files.sort();

    tracing::info!(
        files_found = files.len(),
        skipped_large = skipped_large,
        skipped_permission = skipped_permission,
        "Directory scan completed"
    );

    Ok(files)
}

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Path not found: {0}")]
    NotFound(PathBuf),

    #[error("Path is not a directory: {0}")]
    NotADirectory(PathBuf),

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

    #[test]
    fn test_gitignore_respected() {
        use std::io::Write;

        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create .gitignore file
        let gitignore_path = root.join(".gitignore");
        let mut gitignore = File::create(&gitignore_path).unwrap();
        writeln!(gitignore, "node_modules/").unwrap();
        writeln!(gitignore, "*.log").unwrap();
        writeln!(gitignore, "build/").unwrap();
        drop(gitignore);

        // Create directory structure
        let node_modules = root.join("node_modules");
        fs::create_dir(&node_modules).unwrap();
        File::create(node_modules.join("package.js")).unwrap();

        let build = root.join("build");
        fs::create_dir(&build).unwrap();
        File::create(build.join("output.js")).unwrap();

        // These should be scanned
        File::create(root.join("main.ts")).unwrap();
        File::create(root.join("app.js")).unwrap();

        // These should be ignored by .gitignore
        File::create(root.join("debug.log")).unwrap();
        File::create(root.join("error.log")).unwrap();

        // Run scan
        let result = scan_directory(root, &[]).unwrap();

        // Verify only non-ignored files are included
        let file_names: Vec<_> = result.iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect();

        println!("Found files: {:?}", file_names);

        // Should find main.ts and app.js
        assert!(file_names.contains(&"main.ts"), "main.ts should be included");
        assert!(file_names.contains(&"app.js"), "app.js should be included");

        // Should NOT find files in node_modules, build, or .log files
        assert!(!file_names.contains(&"package.js"), "node_modules/package.js should be ignored");
        assert!(!file_names.contains(&"output.js"), "build/output.js should be ignored");
        assert!(!file_names.contains(&"debug.log"), "debug.log should be ignored");
        assert!(!file_names.contains(&"error.log"), "error.log should be ignored");

        assert_eq!(result.len(), 2, "Should only find 2 files (main.ts, app.js)");
    }

    #[test]
    fn test_nested_gitignore() {
        use std::io::Write;

        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Root .gitignore
        let mut root_gitignore = File::create(root.join(".gitignore")).unwrap();
        writeln!(root_gitignore, "*.tmp").unwrap();
        drop(root_gitignore);

        // Create src directory with its own .gitignore
        let src = root.join("src");
        fs::create_dir(&src).unwrap();
        let mut src_gitignore = File::create(src.join(".gitignore")).unwrap();
        writeln!(src_gitignore, "test/").unwrap();
        drop(src_gitignore);

        // Create test directory inside src
        let test = src.join("test");
        fs::create_dir(&test).unwrap();
        File::create(test.join("test.ts")).unwrap();

        // Create files
        File::create(root.join("main.ts")).unwrap();
        File::create(root.join("temp.tmp")).unwrap(); // Ignored by root .gitignore
        File::create(src.join("app.ts")).unwrap();

        let result = scan_directory(root, &[]).unwrap();
        let file_names: Vec<_> = result.iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect();

        println!("Found files: {:?}", file_names);

        // Should find main.ts and app.ts
        assert!(file_names.contains(&"main.ts"));
        assert!(file_names.contains(&"app.ts"));

        // Should NOT find temp.tmp (ignored by root) or test.ts (ignored by src/.gitignore)
        assert!(!file_names.contains(&"temp.tmp"), "temp.tmp should be ignored by root .gitignore");
        assert!(!file_names.contains(&"test.ts"), "test/ dir should be ignored by src/.gitignore");

        assert_eq!(result.len(), 2, "Should only find 2 files");
    }

    #[test]
    #[ignore] // Run with: cargo test -- --ignored
    fn test_real_repo_gitignore() {
        // Test on the actual code-viz repository
        let repo_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();

        println!("Testing on real repo: {}", repo_path.display());

        let result = scan_directory(repo_path, &[]).unwrap();

        println!("Total files found: {}", result.len());

        // Check for node_modules files - should be ZERO
        let node_modules_files: Vec<_> = result.iter()
            .filter(|p| p.to_string_lossy().contains("node_modules"))
            .collect();

        println!("Files in node_modules: {}", node_modules_files.len());
        if !node_modules_files.is_empty() {
            println!("First 5 node_modules files:");
            for f in node_modules_files.iter().take(5) {
                println!("  - {}", f.display());
            }
        }

        // Check for target/ files - should be ZERO
        let target_files: Vec<_> = result.iter()
            .filter(|p| p.to_string_lossy().contains("/target/"))
            .collect();

        println!("Files in target/: {}", target_files.len());

        assert_eq!(node_modules_files.len(), 0, "node_modules should be excluded by .gitignore");
        assert_eq!(target_files.len(), 0, "target/ should be excluded by .gitignore");

        // Reasonable file count for this repo (should be < 500 without node_modules/target)
        assert!(result.len() < 500, "File count too high: {} (node_modules likely included)", result.len());
    }
}