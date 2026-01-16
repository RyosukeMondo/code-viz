use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use code_viz_core::{
    analyzer::process_file_with_fs,
    context::RealFileSystem,
    coverage::calculate_coverage_analysis,
    duplication::DuplicationDetector,
    hotspot::HotspotDetector,
    models::{CodeChurn, FileMetrics, TestCoverage},
    parser::get_parser,
    scanner::scan_directory,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tempfile::TempDir;

fn create_test_file(dir: &Path, name: &str, content: &str) -> PathBuf {
    let file_path = dir.join(name);
    std::fs::write(&file_path, content).unwrap();
    file_path
}

fn create_benchmark_repo(size: usize) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let src_dir = temp_dir.path().join("src");
    std::fs::create_dir(&src_dir).unwrap();

    for i in 0..size {
        let content = format!(
            r#"
pub fn function_{}() -> i32 {{
    let mut sum = 0;
    for i in 0..100 {{
        sum += i;
    }}
    sum
}}

pub fn helper_{}(x: i32, y: i32) -> i32 {{
    x + y + {}
}}

pub struct Data{} {{
    field1: String,
    field2: i32,
    field3: bool,
}}

impl Data{} {{
    pub fn new() -> Self {{
        Self {{
            field1: String::from("test"),
            field2: {},
            field3: true,
        }}
    }}

    pub fn process(&self) -> i32 {{
        self.field2 * 2
    }}
}}
"#,
            i, i, i, i, i, i
        );

        create_test_file(&src_dir, &format!("file_{}.rs", i), &content);
    }

    temp_dir
}

fn bench_parser_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");

    let test_content = r#"
pub fn complex_function(x: i32, y: i32) -> Result<i32, String> {
    if x < 0 || y < 0 {
        return Err("Negative values not allowed".to_string());
    }

    let mut result = 0;
    for i in 0..x {
        for j in 0..y {
            result += i + j;
        }
    }

    Ok(result)
}

pub struct DataProcessor {
    data: Vec<i32>,
    threshold: i32,
}

impl DataProcessor {
    pub fn new(threshold: i32) -> Self {
        Self {
            data: Vec::new(),
            threshold,
        }
    }

    pub fn process(&mut self, values: &[i32]) {
        for &value in values {
            if value > self.threshold {
                self.data.push(value);
            }
        }
    }
}
"#;

    let parser = get_parser("rust").unwrap();

    group.bench_function("parse_rust_file", |b| {
        b.iter(|| {
            let mut tree_parser = tree_sitter::Parser::new();
            tree_parser.set_language(parser.get_language()).unwrap();
            tree_parser.parse(black_box(test_content), None)
        });
    });

    group.finish();
}

fn bench_parse_many_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_many_files");

    for size in [10, 50, 100].iter() {
        let temp_dir = create_benchmark_repo(*size);
        let fs = RealFileSystem;

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let src_dir = temp_dir.path().join("src");
                let mut count = 0;
                for entry in std::fs::read_dir(&src_dir).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                        let _ = process_file_with_fs(&path, &fs);
                        count += 1;
                    }
                }
                black_box(count)
            });
        });
    }

    group.finish();
}

fn bench_full_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_analysis");

    for size in [10, 50].iter() {
        let temp_dir = create_benchmark_repo(*size);
        let src_dir = temp_dir.path().join("src");

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = scan_directory(black_box(&src_dir), black_box(&[]));
                black_box(result)
            });
        });
    }

    group.finish();
}

fn bench_duplication_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("duplication_detection");

    let duplicate_content = r#"
pub fn duplicate_function(x: i32) -> i32 {
    let mut result = 0;
    for i in 0..x {
        result += i * 2;
    }
    result
}
"#;

    for file_count in [10, 20].iter() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        std::fs::create_dir(&src_dir).unwrap();

        let mut files = Vec::new();
        for i in 0..*file_count {
            let path = create_test_file(&src_dir, &format!("dup_{}.rs", i), duplicate_content);
            files.push((path.clone(), duplicate_content.to_string()));
        }

        let mut parsers = HashMap::new();
        parsers.insert("rust".to_string(), get_parser("rust").unwrap());

        group.bench_with_input(
            BenchmarkId::from_parameter(file_count),
            file_count,
            |b, _| {
                let detector = DuplicationDetector::new(3, 0.8);
                b.iter(|| {
                    let result = detector.run(black_box(&files), black_box(&parsers));
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

fn bench_hotspot_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("hotspot_analysis");

    for file_count in [50, 100, 200].iter() {
        let mut files = Vec::new();
        for i in 0..*file_count {
            let metrics = FileMetrics {
                path: PathBuf::from(format!("file_{}.rs", i)),
                language: "rust".to_string(),
                loc: 100 + (i * 10),
                size_bytes: 1024,
                function_count: 5 + (i % 10),
                last_modified: SystemTime::now(),
                dead_function_count: None,
                dead_code_loc: None,
                dead_code_ratio: None,
                code_churn: Some(CodeChurn {
                    added_lines: (i * 20) as usize,
                    deleted_lines: (i * 10) as usize,
                }),
                coupling: None,
                ai_bloat_index: None,
                cognitive_complexity: None,
                test_coverage: None,
            };
            files.push(metrics);
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(file_count),
            file_count,
            |b, _| {
                let detector = HotspotDetector::new(10);
                b.iter(|| {
                    let result = detector.calculate(black_box(&files));
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

fn bench_coverage_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("coverage_analysis");

    for file_count in [50, 100, 200].iter() {
        let mut files = Vec::new();
        for i in 0..*file_count {
            let metrics = FileMetrics {
                path: PathBuf::from(format!("file_{}.rs", i)),
                language: "rust".to_string(),
                loc: 100,
                size_bytes: 1024,
                function_count: 5,
                last_modified: SystemTime::now(),
                dead_function_count: None,
                dead_code_loc: None,
                dead_code_ratio: None,
                code_churn: None,
                coupling: None,
                ai_bloat_index: None,
                cognitive_complexity: None,
                test_coverage: Some(TestCoverage {
                    line_coverage: 80.0,
                    lines_covered: 80,
                    total_lines: 100,
                    branch_coverage: None,
                }),
            };
            files.push(metrics);
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(file_count),
            file_count,
            |b, _| {
                b.iter(|| {
                    let result = calculate_coverage_analysis(black_box(&files));
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parser_performance,
    bench_parse_many_files,
    bench_full_analysis,
    bench_duplication_detection,
    bench_hotspot_analysis,
    bench_coverage_analysis
);
criterion_main!(benches);
