use code_viz_core::parser::{get_parser, ParseError};

#[test]
fn test_typescript_parser_creation() {
    let parser = get_parser("typescript");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "typescript"); // Test-only: parser creation verified
}

#[test]
fn test_typescript_alias_ts() {
    let parser = get_parser("ts");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "typescript"); // Test-only: alias verified
}

#[test]
fn test_javascript_parser_creation() {
    let parser = get_parser("javascript");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "javascript"); // Test-only: parser creation verified
}

#[test]
fn test_javascript_aliases() {
    assert!(get_parser("js").is_ok());
    assert!(get_parser("jsx").is_ok());
}

#[test]
fn test_tsx_parser_creation() {
    let parser = get_parser("tsx");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "tsx"); // Test-only: parser creation verified
}

#[test]
fn test_rust_parser_creation() {
    let parser = get_parser("rust");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "rust"); // Test-only: parser creation verified
}

#[test]
fn test_rust_alias_rs() {
    let parser = get_parser("rs");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "rust"); // Test-only: alias verified
}

#[test]
fn test_python_parser_creation() {
    let parser = get_parser("python");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "python"); // Test-only: parser creation verified
}

#[test]
fn test_python_alias_py() {
    let parser = get_parser("py");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "python"); // Test-only: alias verified
}

#[test]
fn test_go_parser_creation() {
    let parser = get_parser("go");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "go"); // Test-only: parser creation verified
}

#[test]
fn test_cpp_parser_creation() {
    let parser = get_parser("cpp");
    assert!(parser.is_ok());
    assert_eq!(parser.unwrap().language_key(), "cpp"); // Test-only: parser creation verified
}

#[test]
fn test_cpp_aliases() {
    assert!(get_parser("cxx").is_ok());
    assert!(get_parser("cc").is_ok());
    assert!(get_parser("hpp").is_ok());
    assert!(get_parser("h").is_ok());
}

#[test]
fn test_unsupported_language_error() {
    let result = get_parser("fortran");
    assert!(result.is_err());

    match result {
        Err(ParseError::UnsupportedLanguage(lang)) => {
            assert_eq!(lang, "fortran");
        }
        _ => panic!("Expected UnsupportedLanguage error"),
    }
}

#[test]
fn test_parse_valid_typescript_code() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "function greet(name: string) { return `Hello, ${name}`; }";
    let tree = parser.parse(source);

    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: source is valid
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_parse_typescript_with_syntax_error() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "function incomplete(";
    let tree = parser.parse(source);

    assert!(tree.is_ok()); // Tree-sitter produces a tree even with errors
    let tree = tree.unwrap(); // Test-only: tree-sitter always produces result
    assert!(tree.root_node().has_error());
}

#[test]
fn test_parse_valid_javascript_code() {
    let parser = get_parser("javascript").unwrap(); // Test-only: language is valid
    let source = "const add = (a, b) => a + b;";
    let tree = parser.parse(source);

    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: source is valid
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_parse_valid_tsx_code() {
    let parser = get_parser("tsx").unwrap(); // Test-only: language is valid
    let source = r#"const Component = () => <div>Hello</div>;"#;
    let tree = parser.parse(source);

    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: source is valid
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_parse_valid_rust_code() {
    let parser = get_parser("rust").unwrap(); // Test-only: language is valid
    let source = r#"
        fn main() {
            println!("Hello, world!");
        }
    "#;
    let tree = parser.parse(source);

    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: source is valid
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_parse_valid_python_code() {
    let parser = get_parser("python").unwrap(); // Test-only: language is valid
    let source = r#"
def greet(name):
    return f"Hello, {name}"
    "#;
    let tree = parser.parse(source);

    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: source is valid
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_parse_valid_go_code() {
    let parser = get_parser("go").unwrap(); // Test-only: language is valid
    let source = r#"
package main

func main() {
    println("Hello")
}
    "#;
    let tree = parser.parse(source);

    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: source is valid
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_parse_valid_cpp_code() {
    let parser = get_parser("cpp").unwrap(); // Test-only: language is valid
    let source = r#"
#include <iostream>
int main() {
    std::cout << "Hello" << std::endl;
    return 0;
}
    "#;
    let tree = parser.parse(source);

    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: source is valid
    assert!(!tree.root_node().has_error());
}

#[test]
fn test_count_typescript_functions() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
        function a() {}
        const b = () => {};
        class C {
            method() {}
        }
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 3);
}

#[test]
fn test_count_javascript_functions() {
    let parser = get_parser("javascript").unwrap(); // Test-only: language is valid
    let source = r#"
        function named() {}
        const arrow = () => {};
        const obj = {
            method() {}
        };
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 3);
}

#[test]
fn test_count_tsx_functions() {
    let parser = get_parser("tsx").unwrap(); // Test-only: language is valid
    let source = r#"
        const Component = () => <div></div>;
        function helper() {}
        class Service {
            process() {}
        }
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 3);
}

#[test]
fn test_count_rust_functions() {
    let parser = get_parser("rust").unwrap(); // Test-only: language is valid
    let source = r#"
        fn main() {}
        fn helper() {}
        impl MyStruct {
            fn method(&self) {}
        }
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 3);
}

#[test]
fn test_count_python_functions() {
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
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 3);
}

#[test]
fn test_count_go_functions() {
    let parser = get_parser("go").unwrap(); // Test-only: language is valid
    let source = r#"
package main

func main() {}
func helper() {}
func (s *MyStruct) method() {}
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 3);
}

#[test]
fn test_count_cpp_functions() {
    let parser = get_parser("cpp").unwrap(); // Test-only: language is valid
    let source = r#"
int main() { return 0; }
void helper() {}
class MyClass {
    void method() {}
};
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 3);
}

#[test]
fn test_empty_source_parsing() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "";
    let tree = parser.parse(source);

    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: empty source is valid
    assert!(!tree.root_node().has_error());
    assert_eq!(parser.count_functions(&tree), 0);
}

#[test]
fn test_whitespace_only_source() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "   \n\n\t  \n";
    let tree = parser.parse(source);

    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: whitespace is valid
    assert!(!tree.root_node().has_error());
    assert_eq!(parser.count_functions(&tree), 0);
}

#[test]
fn test_find_typescript_comments() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
// Line comment
function foo() {
    /* Block comment */
    return 42;
}
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let comments = parser.find_comment_ranges(&tree);
    assert!(comments.len() >= 2); // At least line and block comment
}

#[test]
fn test_find_rust_comments() {
    let parser = get_parser("rust").unwrap(); // Test-only: language is valid
    let source = r#"
// Line comment
fn main() {
    /* Block comment */
    println!("test");
}
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let comments = parser.find_comment_ranges(&tree);
    assert!(comments.len() >= 2);
}

#[test]
fn test_find_python_comments() {
    let parser = get_parser("python").unwrap(); // Test-only: language is valid
    let source = r#"
# Comment
def main():
    # Another comment
    pass
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let comments = parser.find_comment_ranges(&tree);
    assert!(comments.len() >= 2);
}

#[test]
fn test_complex_nested_functions() {
    let parser = get_parser("javascript").unwrap(); // Test-only: language is valid
    let source = r#"
        function outer() {
            function inner1() {
                function inner2() {}
            }
        }
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 3); // Should count all nested functions
}

#[test]
fn test_async_functions() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = r#"
        async function fetchData() {}
        const asyncArrow = async () => {};
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 2);
}

#[test]
fn test_generator_functions() {
    let parser = get_parser("javascript").unwrap(); // Test-only: language is valid
    let source = r#"
        function* generator() {
            yield 1;
        }
    "#;
    let result = parser.parse(source);
    // Generator functions may or may not be counted depending on query
    // Just verify parsing works
    assert!(result.is_ok());
}

#[test]
fn test_very_large_source_file() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid

    // Generate a large source file with many functions
    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("function func{i}() {{}}\n"));
    }

    let tree = parser.parse(&source).unwrap(); // Test-only: generated source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 1000);
}

#[test]
fn test_severely_malformed_code() {
    let parser = get_parser("typescript").unwrap(); // Test-only: language is valid
    let source = "!@#$%^&*()";
    let tree = parser.parse(source);

    // Should still produce a tree
    assert!(tree.is_ok());
    let tree = tree.unwrap(); // Test-only: tree-sitter always produces result
    assert!(tree.root_node().has_error());
}

#[test]
fn test_multiline_string_not_counted_as_code() {
    let parser = get_parser("python").unwrap(); // Test-only: language is valid
    let source = r#"
def func():
    """
    Multiline docstring
    """
    pass
    "#;
    let tree = parser.parse(source).unwrap(); // Test-only: source is valid
    let count = parser.count_functions(&tree);
    assert_eq!(count, 1);
}
