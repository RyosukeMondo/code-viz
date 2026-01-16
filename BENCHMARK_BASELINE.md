# Performance Benchmark Baseline

This document records the baseline performance metrics for the code-viz analysis engine. These benchmarks were established to detect performance regressions during error handling refactoring (Task 34 of error-handling-code-quality-remediation spec).

## Benchmark Date

2026-01-17

## Benchmark Environment

- Platform: Linux
- Rust Version: 1.85+ (release profile)
- Criterion Version: 0.5

## Baseline Results

### Parser Performance

| Benchmark | Time | Notes |
|-----------|------|-------|
| parse_rust_file | ~61-62 µs | Single file parse operation |

### Multi-File Parsing

| File Count | Time | Notes |
|------------|------|-------|
| 10 files | ~1.5 ms | Parse 10 Rust files sequentially |
| 50 files | ~7.4 ms | Parse 50 Rust files sequentially |
| 100 files | ~15 ms | Parse 100 Rust files sequentially |

### Full Analysis (scan + parse)

| File Count | Time | Notes |
|------------|------|-------|
| 10 files | ~105-112 µs | Directory scan with glob patterns |
| 50 files | ~240 µs | Directory scan with glob patterns |

### Duplication Detection

| File Count | Time | Notes |
|------------|------|-------|
| 10 files | ~384 ns | Duplicate code detection |
| 20 files | ~762 ns | Duplicate code detection |

### Hotspot Analysis

| File Count | Time | Notes |
|------------|------|-------|
| 50 files | ~2.6 µs | Git hotspot detection |
| 100 files | ~5.5 µs | Git hotspot detection |
| 200 files | ~9.8 µs | Git hotspot detection |

### Coverage Analysis

| File Count | Time | Notes |
|------------|------|-------|
| 50 files | ~178 ns | Coverage statistics calculation |
| 100 files | ~303 ns | Coverage statistics calculation |
| 200 files | ~486 ns | Coverage statistics calculation |

## Performance Acceptance Criteria

As per Task 34 requirements:
- **Maximum Acceptable Regression**: ±5%
- Any benchmark showing >5% slowdown should trigger investigation
- Benchmarks should be run in CI on every PR

## Running the Benchmarks

To run the benchmarks and compare against this baseline:

```bash
# Quick benchmark (reduced sample size)
cargo bench --bench analysis_benchmarks -- --quick

# Full benchmark (default sample size)
cargo bench --bench analysis_benchmarks

# Compare against baseline
cargo bench --bench analysis_benchmarks -- --save-baseline main
cargo bench --bench analysis_benchmarks -- --baseline main
```

## Benchmark Components

1. **parser/parse_rust_file**: Measures tree-sitter parsing performance for a single file
2. **parse_many_files**: Measures throughput when parsing multiple files sequentially
3. **full_analysis**: Measures end-to-end directory scanning with glob pattern matching
4. **duplication_detection**: Measures code duplication analysis performance
5. **hotspot_analysis**: Measures git hotspot calculation (churn + complexity + size)
6. **coverage_analysis**: Measures test coverage aggregation and reporting

## Notes

- All benchmarks use criterion with default configuration
- Results may vary based on system load and hardware
- Criterion provides statistical analysis including outlier detection
- HTML reports are generated in `target/criterion/` directory
