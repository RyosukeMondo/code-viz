use code_viz_core::models::AnalysisConfig;
use code_viz_core::analyze;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_analyze_sample_repository() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create file structure
    let src = root.join("src");
    fs::create_dir(&src).unwrap();

    let index_ts = src.join("index.ts");
    let mut f = File::create(&index_ts).unwrap();
    // 3 LOC
    writeln!(f, "function main() {{").unwrap();
    writeln!(f, "    console.log('hello');").unwrap();
    writeln!(f, "}}").unwrap();

    let utils_js = src.join("utils.js");
    let mut f = File::create(&utils_js).unwrap();
    // 1 LOC
    writeln!(f, "export const x = 1;").unwrap();

    // Node modules (should be excluded by default)
    let node_modules = root.join("node_modules");
    fs::create_dir(&node_modules).unwrap();
    let dep_js = node_modules.join("dep.js");
    File::create(&dep_js).unwrap();

    let config = AnalysisConfig::default();
    let result = analyze(root, &config).unwrap();

    assert_eq!(result.summary.total_files, 2);
    assert_eq!(result.summary.total_loc, 4);
    
    // Sort files for snapshot stability (analyze already sorts, but to be sure)
    // Snapshot the result (excluding timestamp)
    
    // Check largest files
    assert_eq!(result.summary.largest_files[0].file_name().unwrap(), "index.ts");
    
    // We can't snapshot the whole result easily because paths are absolute and random (temp dir).
    // We'll snapshot specific fields or normalize paths.
    // For now, let's snapshot the summary (excluding largest_files paths which are absolute) and files list with relative paths.
    
    // Helper to normalize paths for snapshot
    let normalized_files: Vec<String> = result.files.iter()
        .map(|f| {
            let relative = f.path.strip_prefix(root).unwrap_or(&f.path);
            format!("{} ({} LOC, {} funcs)", relative.to_string_lossy(), f.loc, f.function_count)
        })
        .collect();

    insta::assert_debug_snapshot!(normalized_files);
}

#[test]
fn test_analyze_with_exclusions() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    let generated = root.join("generated");
    fs::create_dir(&generated).unwrap();
    File::create(generated.join("gen.ts")).unwrap();
    
    let src = root.join("src");
    fs::create_dir(&src).unwrap();
    File::create(src.join("main.ts")).unwrap();

    let mut config = AnalysisConfig::default();
    config.exclude_patterns.push("**/generated/**".to_string());
    
    let result = analyze(root, &config).unwrap();
    assert_eq!(result.summary.total_files, 1);
    assert_eq!(result.files[0].path.file_name().unwrap(), "main.ts");
}

#[test]
fn test_analyze_handles_parse_errors() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create a malformed file
    let bad_ts = root.join("bad.ts");
    let mut f = File::create(&bad_ts).unwrap();
    writeln!(f, "function oops( {{").unwrap(); // Syntax error, tree-sitter might recover or error?
    // Actually tree-sitter usually recovers.
    // To trigger a ParseError that bubbles up, we'd need get_parser failure or IO error.
    // But analyze() catches errors.
    
    // Let's create a file with unsupported extension if we want to test skipping, 
    // but we want to test "parse error".
    // Tree-sitter is robust.
    // Maybe we can simulate an error by using a file that fails tree-sitter rules?
    // Hard with tree-sitter.
    // But we can verify analysis continues.
    
    let good_ts = root.join("good.ts");
    let mut f = File::create(&good_ts).unwrap();
    writeln!(f, "const x = 1;").unwrap();

    let config = AnalysisConfig::default();
    let result = analyze(root, &config).unwrap();
    
    // Even with syntax error, tree-sitter returns a tree (with error nodes).
    // Our metrics calculation might be weird but it won't fail the `analyze` call.
    // So both files should be present.
    assert_eq!(result.summary.total_files, 2);
}

#[test]
fn test_analyze_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();
    let config = AnalysisConfig::default();
    let result = analyze(root, &config).unwrap();
    assert_eq!(result.summary.total_files, 0);
}
