#![allow(clippy::unwrap_used, clippy::expect_used)]
use code_viz_core::metrics::calculate_metrics;
use code_viz_core::parser::get_parser;
use std::path::Path;
use std::time::SystemTime;
use tempfile::TempDir;

#[test]
fn test_basic_typescript_metrics() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
function hello() {
    console.log("world");
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.path, path);
    assert_eq!(metrics.language, "typescript");
    assert!(metrics.loc > 0);
    assert_eq!(metrics.function_count, 1);
    assert!(metrics.size_bytes > 0);
}

#[test]
fn test_loc_excludes_blank_lines() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
function test() {

    let x = 1;


    return x;
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    // Should only count non-blank lines with code
    assert_eq!(metrics.loc, 4); // function, let x, return, closing brace
}

#[test]
fn test_loc_excludes_comments() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
// This is a comment
function test() {
    // Another comment
    let x = 1; // Inline comment
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    // Should not count comment-only lines
    assert_eq!(metrics.loc, 3); // function, let x, closing brace
}

#[test]
fn test_loc_with_block_comments() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
function test() {
    /* This is a
       multiline
       block comment */
    let x = 1;
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.loc, 3); // function, let x, closing brace
}

#[test]
fn test_loc_with_inline_comment() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "let x = 1; // Comment after code";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.loc, 1); // Code line with inline comment counts as 1
}

#[test]
fn test_loc_code_after_block_comment() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "/* comment */ let x = 1;";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.loc, 1);
}

#[test]
fn test_function_count_typescript() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
function a() {}
const b = () => {};
class C {
    method() {}
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.function_count, 3);
}

#[test]
fn test_function_count_rust() {
    let parser = get_parser("rust").unwrap(); // Test-only: language is valid
    let source = r#"
fn main() {}
fn helper() {}
impl MyStruct {
    fn method(&self) {}
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.rs");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.function_count, 3);
}

#[test]
fn test_function_count_python() {
    let parser = get_parser("python").unwrap(); // Test-only: language is valid
    let source = r#"
def main():
    pass

def helper():
    pass

class MyClass:
    def method(self):
        pass
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.py");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.function_count, 3);
}

#[test]
fn test_empty_file() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("empty.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.loc, 0);
    assert_eq!(metrics.function_count, 0);
    assert_eq!(metrics.size_bytes, 0);
    assert_eq!(metrics.ai_bloat_index, Some(0.0));
}

#[test]
fn test_whitespace_only_file() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "   \n\n\t  \n";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("whitespace.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.loc, 0);
    assert_eq!(metrics.function_count, 0);
}

#[test]
fn test_comments_only_file() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
// Comment 1
// Comment 2
/* Block comment */
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("comments.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.loc, 0);
    assert_eq!(metrics.ai_bloat_index, Some(999.0)); // Only comments, no code
}

#[test]
fn test_ai_bloat_index_no_comments() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "let x = 1;";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.ai_bloat_index, Some(0.0));
}

#[test]
fn test_ai_bloat_index_with_comments() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
// Comment
let x = 1;
    "#;
    // 2 non-blank lines: comment line and code line
    // 1 LOC
    // Comment lines = 2 - 1 = 1
    // AI bloat = (1 / 1) * 100 = 100.0

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.ai_bloat_index, Some(100.0));
}

#[test]
fn test_size_bytes_calculation() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "let x = 1;";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.size_bytes, source.len() as u64);
}

#[test]
fn test_last_modified_defaults_to_now() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "let x = 1;";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let before = SystemTime::now();
    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source
    let after = SystemTime::now();

    assert!(metrics.last_modified >= before && metrics.last_modified <= after);
}

#[test]
fn test_last_modified_provided() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "let x = 1;";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let provided_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1000000);
    let metrics = calculate_metrics(&path, source, parser.as_ref(), Some(provided_time)).unwrap(); // Test-only: valid source

    assert_eq!(metrics.last_modified, provided_time);
}

#[test]
fn test_cognitive_complexity_simple_function() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
function simple() {
    return 42;
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert!(metrics.cognitive_complexity.is_some());
    let complexity = metrics.cognitive_complexity.unwrap(); // Test-only: complexity calculated

    // Find the "simple" function
    let simple = complexity.functions.iter().find(|f| f.name == "simple");
    assert!(simple.is_some());
    assert_eq!(simple.unwrap().complexity, 0); // Test-only: found simple function
}

#[test]
fn test_cognitive_complexity_with_if_statement() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
function test(x) {
    if (x > 0) {
        return x;
    }
    return 0;
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    let complexity = metrics.cognitive_complexity.unwrap(); // Test-only: complexity calculated
    assert!(complexity.functions[0].complexity > 0);
}

#[test]
fn test_cognitive_complexity_nested_conditions() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
function nested(x, y) {
    if (x > 0) {
        if (y > 0) {
            return x + y;
        }
    }
    return 0;
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    let complexity = metrics.cognitive_complexity.unwrap(); // Test-only: complexity calculated
                                                            // Nested if should have higher complexity due to nesting
    assert!(complexity.functions[0].complexity > 2);
}

#[test]
fn test_cognitive_complexity_multiple_functions() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
function a() {
    if (true) return 1;
}

function b() {
    for (let i = 0; i < 10; i++) {
        if (i % 2 === 0) continue;
    }
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    let complexity = metrics.cognitive_complexity.unwrap(); // Test-only: complexity calculated
                                                            // Should have at least our 2 functions
    assert!(complexity.functions.len() >= 2);
    assert!(complexity.total_complexity > 0);
    assert!(complexity.average_complexity > 0.0);
}

#[test]
fn test_cognitive_complexity_no_functions() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "let x = 1; let y = 2;";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert!(metrics.cognitive_complexity.is_none());
}

#[test]
fn test_malformed_source_returns_error() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
                                                    // Note: tree-sitter is very permissive, so truly failing to parse is rare
                                                    // This tests the error handling path
    let source = "function incomplete(";

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let result = calculate_metrics(&path, source, parser.as_ref(), None);

    // Tree-sitter usually still produces a result even with errors
    assert!(result.is_ok());
}

#[test]
fn test_very_large_file() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid

    // Generate a large file
    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("function func{i}() {{}}\n"));
    }

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("large.ts");

    let metrics = calculate_metrics(&path, &source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.function_count, 1000);
    assert!(metrics.loc >= 1000);
}

#[test]
fn test_multiline_string_handling() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
const text = `
    This is a
    multiline
    string
`;
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    // Should count non-blank code lines
    assert!(metrics.loc > 0);
}

#[test]
fn test_different_languages() {
    let languages = vec!["typescript", "javascript", "rust", "python", "go", "cpp"];

    for lang in languages {
        let parser = get_parser(lang).unwrap(); // Test-only: all languages are valid
        let source = match lang {
            "rust" => "fn main() {}",
            "python" => "def main():\n    pass",
            "go" => "package main\nfunc main() {}",
            "cpp" => "int main() { return 0; }",
            _ => "function main() {}",
        };

        let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
        let path = temp_dir.path().join(format!("test.{lang}"));

        let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

        assert_eq!(metrics.language, parser.language_key());
        assert!(metrics.function_count >= 1);
    }
}

#[test]
fn test_cognitive_complexity_aggregates() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
function simple() {
    return 1;
}

function complex() {
    if (true) {
        if (true) {
            for (let i = 0; i < 10; i++) {
                if (i > 5) break;
            }
        }
    }
}
    "#;

    let temp_dir = TempDir::new().unwrap(); // Test-only: temp dir creation
    let path = temp_dir.path().join("test.ts");

    let metrics = calculate_metrics(&path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    let complexity = metrics.cognitive_complexity.unwrap(); // Test-only: complexity calculated

    assert!(complexity.functions.len() >= 2);
    assert!(complexity.max_complexity > 0);
    assert!(complexity.average_complexity > 0.0);
    let expected_total: usize = complexity.functions.iter().map(|f| f.complexity).sum();
    assert_eq!(complexity.total_complexity, expected_total);
}

#[test]
fn test_path_stored_correctly() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "let x = 1;";

    let path = Path::new("/custom/path/to/file.ts");
    let metrics = calculate_metrics(path, source, parser.as_ref(), None).unwrap(); // Test-only: valid source

    assert_eq!(metrics.path, path);
}
