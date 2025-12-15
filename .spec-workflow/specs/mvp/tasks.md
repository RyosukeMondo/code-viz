# Tasks Document - MVP

## Phase 0: Skeleton Generation (Day 1-2)

### 0.1 Create Workspace Structure and Configuration

- [x] 0.1.1 Create Workspace Structure and Configuration
  - Files:
    - `Cargo.toml` (workspace root)
    - `.cargo/config.toml` (linker configuration)
    - `Justfile` (task runner recipes)
    - `.gitignore`
    - `README.md` (skeleton)
  - Purpose: Establish workspace foundation with build tooling configured for rapid iteration
  - _Leverage: `.spec-workflow/steering/tech.md` (Mold linker config), `.spec-workflow/steering/structure.md` (naming conventions)_
  - _Requirements: NFR - Performance (Build Speed), NFR - Code Architecture_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: DevOps Engineer specializing in Rust build systems and tooling | Task: Create Cargo workspace root configuration including workspace member declarations (code-viz-core, code-viz-cli), shared dependency definitions (serde, tree-sitter), .cargo/config.toml with mold linker for Linux and appropriate linker for macOS/Windows, Justfile with placeholder recipes (dev, test, check, release), and comprehensive .gitignore for Rust projects | Restrictions: Follow exact linker configuration from tech.md, use workspace dependencies feature for DRY, do not add implementation-specific flags yet, ensure cross-platform compatibility | Leverage: .spec-workflow/steering/tech.md section "Rust Compilation Acceleration", .spec-workflow/steering/structure.md for workspace layout | Requirements: NFR - Build Speed (<3s incremental builds), NFR - Code Architecture (workspace separation) | Success: cargo check passes on workspace root, Justfile recipes are defined (can run just --list), .cargo/config.toml uses correct linker per platform, all paths follow structure.md conventions | Instructions: Edit tasks.md and mark this task as in-progress [-] before starting. After completion, use log-implementation tool with detailed artifacts (all config files created with their purposes), then mark task as complete [x] in tasks.md_

- [x] 0.1.2 Create core library crate skeleton (code-viz-core)
  - Files:
    - `crates/code-viz-core/Cargo.toml`
    - `crates/code-viz-core/src/lib.rs`
    - `crates/code-viz-core/src/models.rs`
    - `crates/code-viz-core/src/scanner.rs`
    - `crates/code-viz-core/src/parser.rs`
    - `crates/code-viz-core/src/metrics.rs`
    - `crates/code-viz-core/src/analyzer.rs`
    - `crates/code-viz-core/src/cache.rs`
  - Purpose: Create analysis engine library crate with complete module structure and type signatures (all functions use todo!() for unimplemented logic)
  - _Leverage: `.spec-workflow/specs/mvp/design.md` (Component signatures), `.spec-workflow/steering/structure.md` (module organization)_
  - _Requirements: Req 2 (Repository Analysis Engine), NFR - Code Architecture_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Rust Systems Developer specializing in library design and API contracts | Task: Create code-viz-core library crate with Cargo.toml declaring all dependencies (tree-sitter 0.20+, serde 1.0+ with derive, rayon 1.8+, thiserror, walkdir, globset), lib.rs with pub mod declarations and re-exports (pub use analyzer::analyze, pub use models::*), and skeleton files (models.rs with FileMetrics/AnalysisResult/Summary/AnalysisConfig structs fully defined per design.md, scanner.rs with scan_directory signature + ScanError enum, parser.rs with LanguageParser trait + TypeScriptParser/JavaScriptParser empty impls, metrics.rs with calculate_metrics signature, analyzer.rs with analyze signature, cache.rs with DiskCache struct + method signatures) - ALL function bodies must be todo!("Description from design.md") | Restrictions: Do NOT implement any logic, only signatures and data structures; use #[allow(dead_code)] at crate level; ensure all types match design.md exactly; all error types use thiserror derive | Leverage: design.md sections "Components and Interfaces" and "Data Models" for exact signatures; structure.md for Rust naming conventions (snake_case functions, PascalCase types) | Requirements: Req 2.1 (File metrics calculation), Req 4 (Multi-language support), NFR - Modularity (trait-based parsers) | Success: cargo check passes in code-viz-core crate with only dead_code warnings, all pub structs/enums/traits defined per design.md, all function signatures compile with correct types, lib.rs exports match public API in design.md | Instructions: Mark task in-progress [-] in tasks.md. After creation, run cargo check to verify. Use log-implementation tool documenting all module files created with their exported types/traits/functions. Mark complete [x] in tasks.md._

- [x] 0.1.3 Create CLI binary crate skeleton (code-viz-cli)
  - Files:
    - `crates/code-viz-cli/Cargo.toml`
    - `crates/code-viz-cli/src/main.rs`
    - `crates/code-viz-cli/src/commands/mod.rs`
    - `crates/code-viz-cli/src/commands/analyze.rs`
    - `crates/code-viz-cli/src/commands/watch.rs`
    - `crates/code-viz-cli/src/commands/diff.rs`
    - `crates/code-viz-cli/src/commands/config.rs`
    - `crates/code-viz-cli/src/output/mod.rs`
    - `crates/code-viz-cli/src/output/json.rs`
    - `crates/code-viz-cli/src/output/csv.rs`
    - `crates/code-viz-cli/src/output/text.rs`
    - `crates/code-viz-cli/src/config_loader.rs`
  - Purpose: Create CLI binary crate with clap argument structure and command modules (all using todo!() for unimplemented business logic)
  - _Leverage: `.spec-workflow/specs/mvp/design.md` (CLI structure), `.spec-workflow/specs/mvp/requirements.md` (CLI commands)_
  - _Requirements: Req 1 (CLI Interface), Req 6 (Export formats), NFR - Code Architecture_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: CLI Developer specializing in Rust command-line applications and clap framework | Task: Create code-viz-cli binary crate with Cargo.toml dependencies (code-viz-core path dependency, clap 4.0+ with derive feature, serde_json, csv, notify 6.0+, env_logger), main.rs with clap Parser/Subcommand derive macros defining CLI structure (Analyze{path, format, exclude, verbose, threshold, output}, Watch{path, format, verbose}, Diff{old, new}, Config subcommand for init), match on commands with todo!() bodies, commands/ directory with mod.rs declaring submodules (pub mod analyze/watch/diff/config) and each command file (analyze.rs, watch.rs, etc.) with pub fn run() signatures taking parsed args and returning Result<(), CommandError> with todo!() bodies, output/ directory with MetricsFormatter trait definition and JsonFormatter/CsvFormatter/TextFormatter structs with empty format() method impls using todo!(), config_loader.rs with function to parse .code-viz.toml using toml crate (signature only, todo!() body) | Restrictions: Do NOT implement command logic or formatters, only argument parsing structure and module organization; ensure clap derives compile; all error types use thiserror; main() should parse args and dispatch to command modules (with todo!() for now) | Leverage: requirements.md Req 1 acceptance criteria for exact CLI flags/commands, design.md "CLI Interface" section for argument structure, structure.md for file organization | Requirements: Req 1.1-1.5 (CLI commands), Req 6.1 (CSV output), NFR - Usability (Shell Integration) | Success: cargo check passes on code-viz-cli, cargo run -- --help displays clap-generated help (even with todo!() bodies), all subcommands parse correctly, commands/ and output/ modules export expected functions/traits | Instructions: Mark in-progress [-]. After creation, verify with cargo run -- --help. Use log-implementation to document CLI structure (Commands enum variants, all command modules, formatter trait). Mark complete [x]._

- [x] 0.1.4 Create test structure skeleton
  - Files:
    - `crates/code-viz-core/tests/integration_test.rs`
    - `crates/code-viz-core/tests/snapshots/.gitkeep`
    - `crates/code-viz-cli/tests/cli_e2e_test.rs`
    - `crates/code-viz-cli/tests/fixtures/.gitkeep`
  - Purpose: Establish test infrastructure with placeholder tests for future E2E and integration testing
  - _Leverage: `.spec-workflow/specs/mvp/design.md` (Testing Strategy)_
  - _Requirements: NFR - Testability, Design Section "Autonomous E2E CLI Testing"_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: QA Engineer specializing in Rust testing frameworks and test infrastructure | Task: Create test directory structure with integration_test.rs in code-viz-core/tests/ containing single #[test] fn test_placeholder() using todo!("Integration tests will be added in Phase 1"), cli_e2e_test.rs in code-viz-cli/tests/ with dependencies on assert_cmd, assert_fs, predicates in Cargo.toml [dev-dependencies], and single #[test] fn e2e_placeholder() using todo!("E2E tests from design.md section 'Autonomous E2E CLI Testing'"), create empty snapshots/ directory for insta crate snapshots (future), create empty fixtures/ directory for test data files | Restrictions: Tests can be #[ignore] or use todo!() - they won't pass yet; ensure test Cargo.toml sections have correct dev-dependencies (assert_cmd = "2.0", assert_fs = "1.0", predicates = "3.0", insta = "1.0" for core crate); do NOT write real test logic yet | Leverage: design.md section "Autonomous End-to-End CLI Testing" for E2E test examples (will be implemented in Phase 4), design.md "Unit Testing" section for integration test patterns | Requirements: NFR - Testability (all components testable), Design requirement for autonomous E2E tests | Success: cargo test runs and shows placeholder/ignored tests, assert_cmd/assert_fs/predicates available in cli test crate, insta available in core test crate, directory structure matches design.md | Instructions: Mark in-progress [-]. Run cargo test to verify structure. Log test infrastructure created (test file locations, dev-dependencies added). Mark complete [x]._

### Verification Checkpoint 0

After completing Phase 0 tasks:
- Run `cargo check` - should pass with only dead_code/unused warnings
- Run `cargo clippy --allow=dead_code` - should pass
- Run `cargo run -p code-viz-cli -- --help` - should display help text
- Verify all imports resolve (no "unresolved import" errors)
- Verify all data structures match design.md
- Commit skeleton with message: "feat: MVP skeleton structure with todo!() placeholders"

## Phase 1: Core Analysis Engine (Week 1)

### 1.1 File Scanner Implementation

- [x] 1.1.1 Implement file discovery with exclusions (scanner.rs)
  - Files: `crates/code-viz-core/src/scanner.rs`
  - Purpose: Implement recursive directory traversal with glob pattern exclusions
  - _Leverage: `walkdir` crate, `globset` crate, design.md scanner algorithm_
  - _Requirements: Req 2.1 (File discovery), Req 5.1 (Default exclusions)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Rust Systems Developer specializing in file I/O and pattern matching | Task: Replace todo!() in scan_directory() function with implementation using walkdir::WalkDir for recursive traversal, globset::GlobSetBuilder to compile exclude patterns, filter by file extensions (.ts, .tsx, .js, .jsx, .rs, .py), skip symlinks and hidden files (except .ts/.js), apply exclusions early (don't descend into excluded directories), return Vec<PathBuf> sorted alphabetically, implement ScanError variants (InvalidPattern for glob compile errors, PermissionDenied wrapping io::Error) | Restrictions: Must handle permission errors gracefully (log warning, continue), skip files >10MB (log warning), use DirEntry::file_type() check before following, ensure paths are relative to root or absolute based on input | Leverage: design.md "Component 1: File Scanner" algorithm, requirements.md Req 5.1 for default exclusion list (node_modules, target, .git, dist, build, __pycache__) | Requirements: Req 2.1 (scan files), Req 5.1 (exclusions), NFR Performance (parallelized traversal if possible with rayon) | Success: cargo test tests::test_scan_with_exclusions passes (create test with temp dir), scans Rust compiler repo in <10s, correctly excludes patterns, handles permission errors without crashing | Instructions: Mark in-progress [-]. Write unit tests in scanner.rs #[cfg(test)] mod. Use log-implementation documenting function implementation + test results. Mark complete [x]._

- [x] 1.1.2 Add scanner unit tests
  - Files: `crates/code-viz-core/src/scanner.rs` (#[cfg(test)] mod tests)
  - Purpose: Test scanner with various exclusion patterns and edge cases
  - _Leverage: `tempfile` crate for test directories_
  - _Requirements: NFR - Testability_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: QA Engineer specializing in Rust unit testing | Task: Add #[cfg(test)] mod tests at end of scanner.rs with test helper using tempfile::tempdir() to create test directory structures, write tests: test_scan_empty_dir (empty dir returns empty vec), test_scan_with_files (finds .rs/.ts files), test_scan_excludes_node_modules (creates node_modules/file.ts, verifies excluded), test_scan_excludes_custom_pattern (uses custom glob like **/*.test.ts), test_scan_permission_denied (if possible on platform, test graceful handling), test_scan_filters_extensions (creates .txt, .rs, .py, verifies only .rs/.py returned) | Restrictions: Tests must be deterministic and fast (<100ms each), use assert_eq! or assert! for verification, clean up temp dirs (tempfile handles automatically) | Leverage: design.md scanner component acceptance criteria, Rust testing best practices (arrange-act-assert) | Requirements: NFR Testability, Req 2.4 (graceful error handling) | Success: cargo test scanner::tests passes all tests, tests cover happy path + exclusions + errors, tests run in <500ms total | Instructions: Mark in-progress [-]. Run cargo test after writing. Log test coverage (6+ tests covering different scenarios). Mark complete [x]._

### 1.2 Tree-sitter Parser Integration

- [x] 1.2.1 Implement TypeScript/JavaScript parser (parser.rs)
  - Files: `crates/code-viz-core/src/parser.rs`
  - Purpose: Implement LanguageParser trait for TypeScript and JavaScript using Tree-sitter
  - _Leverage: `tree-sitter` crate, `tree-sitter-typescript` grammar, design.md parser component_
  - _Requirements: Req 4.1 (TypeScript/JavaScript support)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Compiler Engineer specializing in Tree-sitter and syntax analysis | Task: Replace todo!() in TypeScriptParser and JavaScriptParser impls with real implementations: create tree_sitter::Parser instances with tree_sitter_typescript::language_typescript() and tree_sitter_javascript::language(), implement parse() method calling parser.parse(source, None), handle parse errors by checking if tree.root_node() has error nodes, implement count_functions() using tree-sitter queries (S-expression pattern matching function_declaration, arrow_function, method_definition nodes), cache Parser instances using lazy_static or OnceCell to avoid re-initialization | Restrictions: Must return ParseError::TreeSitterError if parsing fails, do not panic on malformed code, queries must be pre-compiled and reused, parser instances should be thread-safe (use Mutex if necessary) | Leverage: design.md "Component 2: Language Parser" with query examples, tree-sitter docs for query syntax, tree-sitter-typescript examples | Requirements: Req 4.1 (TS/JS support), Req 4.4 (Tree-sitter usage), NFR Performance (sub-second parsing for typical files) | Success: parse() successfully parses valid TypeScript/JavaScript, returns error for syntax errors, count_functions() accurately counts functions in test files (compare to manual count), cargo test parser::tests::test_typescript_parsing passes | Instructions: Mark in-progress [-]. Write tests with sample TS/JS code. Log parser implementation + function counting accuracy. Mark complete [x]._

- [x] 1.2.2 Add parser unit and snapshot tests
  - Files: `crates/code-viz-core/src/parser.rs` (#[cfg(test)] mod tests), `crates/code-viz-core/tests/snapshots/` (insta snapshots)
  - Purpose: Test parsers with real code samples and snapshot expected AST structures
  - _Leverage: `insta` crate for snapshot testing_
  - _Requirements: NFR - Testability_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: QA Engineer specializing in compiler testing and snapshot testing | Task: Add parser tests in #[cfg(test)] mod: test_parse_valid_typescript (parse simple TS file, assert Ok), test_parse_syntax_error (parse invalid code, assert ParseError), test_count_functions_typescript (parse file with 3 functions, assert count == 3), test_count_functions_methods (parse class with methods, count correctly), use insta::assert_debug_snapshot! to snapshot parse tree root nodes for regression testing (test_snapshot_typescript_ast, test_snapshot_javascript_ast), create const test fixtures (SAMPLE_TS_CODE, SAMPLE_JS_CODE) as string literals in test mod | Restrictions: Snapshots must be committed to git, re-run cargo insta review to approve snapshots on first run, tests must not depend on external files (use inline strings), verify snapshots match expected AST structure | Leverage: design.md parser testing examples, insta crate documentation for assert_debug_snapshot usage | Requirements: NFR Testability, Req 4.4 (accurate parsing) | Success: cargo test parser::tests passes, cargo insta test shows snapshots match, function counting is accurate (manually verify count for test fixtures), snapshots stored in tests/snapshots/ directory | Instructions: Mark in-progress [-]. Run cargo insta test and cargo insta review to create snapshots. Log parser tests + snapshot files created. Mark complete [x]._

### 1.3 Metrics Calculation

- [x] 1.3.1 Implement LOC calculation (metrics.rs)
  - Files: `crates/code-viz-core/src/metrics.rs`
  - Purpose: Implement Lines of Code calculation excluding comments and blank lines
  - _Leverage: Tree-sitter comment node detection, design.md metrics algorithm_
  - _Requirements: Req 2.1 (LOC calculation), Req 4.2 (Comment exclusion)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Static Analysis Engineer specializing in code metrics | Task: Replace todo!() in calculate_metrics() with implementation: parse source using LanguageParser::parse(), get comment nodes from AST (use query for comment/line_comment/block_comment nodes depending on language), calculate line ranges covered by comments using node.start_position().row and node.end_position().row, split source into lines (source.lines()), count non-empty lines (line.trim().is_empty() == false) that don't overlap with comment line ranges, set FileMetrics.loc to this count, also populate function_count using parser.count_functions(), size_bytes from source.len(), last_modified from fs::metadata(path).modified(), handle MetricsError for parse failures or filesystem errors | Restrictions: Mixed-line comments (code + comment on same line) count as 1 LOC (code takes precedence), blank lines in multiline comments don't count, must handle edge case of file with only comments (LOC = 0), use HashSet<usize> to track comment line numbers for O(1) lookup | Leverage: design.md "Component 3: Metrics Calculator" algorithm, requirements.md Req 4.2 comment syntax per language | Requirements: Req 2.1 (LOC), Req 4.2 (exclude comments), Req 2.3 (file metadata) | Success: calculate_metrics() returns correct LOC for test files (create fixtures with known LOC counts), handles comment-only files, handles mixed-line comments correctly, populates all FileMetrics fields, cargo test metrics::tests passes | Instructions: Mark in-progress [-]. Create test fixtures (e.g., test_file_10loc.rs with exactly 10 LOC). Log metrics implementation + test results with LOC accuracy. Mark complete [x]._

- [x] 1.3.2 Add metrics unit tests with known LOC values
  - Files: `crates/code-viz-core/src/metrics.rs` (#[cfg(test)] mod tests)
  - Purpose: Test metrics calculation accuracy with various code samples
  - _Leverage: Test fixtures with manually counted LOC_
  - _Requirements: NFR - Testability_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: QA Engineer specializing in code quality metrics validation | Task: Add metrics tests: create const fixtures (RUST_CODE_SAMPLE with 15 LOC including comments, TS_CODE_SAMPLE with 20 LOC), test_rust_loc_calculation (calculate metrics for RUST_CODE_SAMPLE, assert LOC == 15), test_typescript_loc_calculation (assert LOC == 20), test_comments_excluded (file with 10 total lines, 5 comment lines, 2 blank, verify LOC == 3), test_mixed_line_comments (line with code and inline comment counts as 1 LOC), test_multiline_comments_excluded (verify block comments fully excluded), test_function_count (file with 3 functions, verify function_count == 3), use tempfile for file metadata tests (create temp file, call calculate_metrics, verify last_modified is recent) | Restrictions: LOC must be manually verified in test fixtures (add comments documenting expected count), tests must be deterministic, use assert_eq! with helpful messages | Leverage: design.md metrics acceptance criteria, edge cases from requirements.md | Requirements: Req 2.1-2.3 (all metrics accurate), NFR Testability | Success: cargo test metrics::tests passes, all LOC counts verified manually, edge cases covered (comments, blank lines, mixed lines), tests run in <1s | Instructions: Mark in-progress [-]. Run tests. Log test coverage (7+ test scenarios) + LOC validation method. Mark complete [x]._

### 1.4 Analysis Orchestrator

- [x] 1.4.1 Implement analyze() orchestrator function (analyzer.rs)
  - Files: `crates/code-viz-core/src/analyzer.rs`
  - Purpose: Orchestrate scan → parse → metrics pipeline with parallelization
  - _Leverage: `rayon` for parallelism, scanner/parser/metrics modules, design.md analyzer flow_
  - _Requirements: Req 2 (Repository Analysis), NFR Performance_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Systems Architect specializing in high-performance data pipelines and concurrency | Task: Replace todo!() in analyze() with full implementation: call scanner::scan_directory(root, config.exclude_patterns), convert Vec<PathBuf> to rayon::par_iter() for parallel processing, for each file (in parallel): read source with fs::read_to_string(), detect language from extension (.ts -> "typescript"), get parser via get_parser(language), call calculate_metrics(path, source, parser), collect results into Vec<FileMetrics>, calculate Summary (total_files = files.len(), total_loc = files.iter().sum(|f| f.loc), total_functions = sum of function_counts, largest_files = files sorted by LOC descending, take 10), return AnalysisResult with summary, files, timestamp = SystemTime::now(), handle errors by logging warnings and continuing (skip unparseable files), implement AnalysisError variants for scan/parse/io failures | Restrictions: Must use rayon for parallelism (do NOT use std::thread manually), files vector must be sorted by path for deterministic output, errors in individual files should not fail entire analysis (log to eprintln! or use tracing crate), ensure thread safety of parser instances | Leverage: design.md "Component 4: Analysis Orchestrator" flow diagram and algorithm, NFR Performance requirement (<30s for 100K files) | Requirements: Req 2.1-2.5 (analysis engine), NFR Performance (parallelized), NFR Reliability (graceful error handling) | Success: analyze() successfully processes test repository (create fixtures/ with sample files), parallelization works (verify with rayon ThreadPool), errors in one file don't crash analysis, Summary is calculated correctly, cargo test analyzer::tests::test_analyze_repository passes | Instructions: Mark in-progress [-]. Create test repository in fixtures/. Test on real repo (e.g., analyze code-viz itself). Log analyzer implementation + performance metrics (time taken, files/sec). Mark complete [x]._

- [x] 1.4.2 Add integration tests for full analysis pipeline
  - Files: `crates/code-viz-core/tests/integration_test.rs`
  - Purpose: Test end-to-end analysis flow with realistic repository structure
  - _Leverage: `tempfile` for test repos, `insta` for snapshot testing results_
  - _Requirements: NFR - Testability_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Integration Test Engineer specializing in end-to-end testing | Task: Replace todo!() in integration_test.rs with real tests: create test_analyze_sample_repository() that uses tempfile::tempdir() to create test repo with structure (src/main.rs with 50 LOC, src/lib.rs with 30 LOC, tests/test.rs with 20 LOC, node_modules/ignored.js), call code_viz_core::analyze() with default config, assert AnalysisResult.summary.total_files == 3 (node_modules excluded), assert total_loc == 100, assert largest_files[0] is main.rs, use insta::assert_yaml_snapshot!(result) to snapshot full result for regression testing, add test_analyze_with_exclusions() testing custom exclude patterns, test_analyze_handles_parse_errors() with intentionally malformed file (verify analysis continues), test_analyze_empty_directory() (returns empty result) | Restrictions: Tests must create files programmatically (use fs::write), clean up automatically (tempfile does this), snapshots must be reviewed and committed, tests should be fast (<2s each) | Leverage: design.md integration testing section, insta crate for snapshot testing | Requirements: All Req 2.x (analysis engine), NFR Testability | Success: cargo test --test integration_test passes all tests, snapshots are stable and committed, tests cover happy path + exclusions + errors + edge cases | Instructions: Mark in-progress [-]. Run cargo insta test and review snapshots. Log integration tests created + snapshot files. Mark complete [x]._

## Phase 2: CLI Interface (Week 2)

### 2.1 Analyze Command Implementation

- [x] 2.1.1 Implement analyze command logic (commands/analyze.rs)
  - Files: `crates/code-viz-cli/src/commands/analyze.rs`
  - Purpose: Implement analyze command that calls core library and formats output
  - _Leverage: `code-viz-core::analyze`, output formatters, design.md CLI flow_
  - _Requirements: Req 1.2 (analyze command), Req 1.3 (JSON output)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: CLI Developer specializing in command-line interface design and user experience | Task: Replace todo!() in commands/analyze.rs run() function with implementation: parse AnalyzeArgs from clap (path, format, exclude patterns, verbose, threshold, output), create AnalysisConfig with exclude_patterns (merge defaults + custom), call code_viz_core::analyze(&path, &config), get formatter based on format arg (json -> JsonFormatter, csv -> CsvFormatter, text -> TextFormatter), call formatter.format(&result), handle output arg (if Some(path) write to file, else print to stdout), implement threshold checking (if threshold is Some("loc=500"), check if any file.loc > 500, exit with code 3 and print violating files to stderr), handle verbose flag (set up env_logger with debug level if verbose), return AnalyzeError for failures | Restrictions: Must print formatted output to stdout (not stderr unless error), exit codes must match design (0 success, 1 error, 2 bad args, 3 threshold violation), use eprintln! for error messages, respect NO_COLOR env var for colored output | Leverage: design.md CLI command flow, requirements.md Req 1.2-1.5 for exact CLI behavior, Req 6.2 for threshold semantics | Requirements: Req 1.2 (analyze command), Req 1.3 (JSON output), Req 6.2 (thresholds), NFR Usability (exit codes) | Success: cargo run -p code-viz-cli -- analyze tests/fixtures --format json outputs valid JSON, thresholds work (exit code 3 when violated), verbose flag shows debug logs, output flag writes to file correctly | Instructions: Mark in-progress [-]. Test manually with various flags. Log command implementation + integration points with core library. Mark complete [x]._

- [x] 2.1.2 Implement output formatters (output/json.rs, csv.rs, text.rs)
  - Files: `crates/code-viz-cli/src/output/json.rs`, `crates/code-viz-cli/src/output/csv.rs`, `crates/code-viz-cli/src/output/text.rs`
  - Purpose: Implement MetricsFormatter trait for JSON, CSV, and text output
  - _Leverage: `serde_json`, `csv` crate, `prettytable-rs` or manual formatting, design.md formatters_
  - _Requirements: Req 1.3 (JSON), Req 6.1 (CSV), Req 1.2 (text summary)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Data Serialization Engineer specializing in multiple output formats | Task: Replace todo!() in formatters: JsonFormatter::format() use serde_json::to_string_pretty(&result) to serialize entire AnalysisResult, CsvFormatter::format() use csv::Writer to create CSV with headers (path,language,loc,functions,size_bytes), flatten result.files into rows, TextFormatter::format() create human-readable summary with format like "Total Files: 1234\nTotal LOC: 56789\nLargest Files:\n  1. src/main.rs (450 LOC)\n  ..." using Summary data, all return Result<String, FormatterError> wrapping serialization errors | Restrictions: JSON must be valid and parseable (test with serde_json::from_str), CSV must follow RFC 4180, text must be readable and concise, do not use unwrap() - return Err for serialization failures | Leverage: design.md "Component 6: Output Formatters" specifications, requirements.md Req 1.3 for JSON schema, Req 6.1 for CSV format | Requirements: Req 1.3 (JSON output), Req 6.1 (CSV output), Req 1.2 (text summary) | Success: JSON output can be piped to jq and parsed, CSV can be opened in Excel/Google Sheets, text summary is human-readable, cargo test output::tests passes (test each formatter with sample AnalysisResult) | Instructions: Mark in-progress [-]. Test outputs with real data. Log formatters implemented + sample outputs for each format. Mark complete [x]._

### 2.2 Configuration Loading

- [x] 2.2.1 Implement .code-viz.toml config parser (config_loader.rs)
  - Files: `crates/code-viz-cli/src/config_loader.rs`
  - Purpose: Load and parse .code-viz.toml configuration file
  - _Leverage: `toml` crate, design.md config file format_
  - _Requirements: Req 5.2 (config file support)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Configuration Management Engineer specializing in declarative config formats | Task: Implement config_loader with load_config(project_root: &Path) -> Result<ConfigFile, ConfigError> function: check if .code-viz.toml exists in project_root, if yes read file with fs::read_to_string(), parse with toml::from_str::<ConfigFile>(), define ConfigFile struct with serde Deserialize (fields: analysis: Option<AnalysisConfigSection>, output: Option<OutputConfigSection>, cache: Option<CacheConfigSection>), AnalysisConfigSection with exclude: Vec<String>, OutputConfigSection with format: String, CacheConfigSection with enabled: bool, merge config with CLI args (CLI > config > defaults), return ConfigError for parse failures or IO errors | Restrictions: Config file is optional (return default config if not found), invalid TOML should return clear error message, do not panic on missing sections (use Option), validate format field (must be "json", "csv", or "text") | Leverage: design.md config file schema, requirements.md Req 5.2 acceptance criteria (CLI flags override config) | Requirements: Req 5.2 (config file), Req 5.3 (config init), NFR Usability | Success: load_config() correctly parses valid .code-viz.toml, returns default if file missing, validates format field, cargo test config_loader::tests passes (test valid config, missing file, invalid TOML) | Instructions: Mark in-progress [-]. Create sample .code-viz.toml for testing. Log config loader implementation + merge logic. Mark complete [x]._

- [x] 2.2.2 Implement config init command (commands/config.rs)
  - Files: `crates/code-viz-cli/src/commands/config.rs`
  - Purpose: Generate .code-viz.toml template file
  - _Leverage: Template string with commented examples_
  - _Requirements: Req 5.3 (config init)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Developer Experience Engineer specializing in CLI tooling and onboarding | Task: Implement config init command in commands/config.rs: create const TEMPLATE: &str with complete .code-viz.toml template including all sections ([analysis], [output], [cache]) with commented examples (e.g., # exclude = ["**/generated/**", "**/*.test.ts"]), run() function writes TEMPLATE to ./.code-viz.toml using fs::write(), check if file exists first (return error if yes, don't overwrite), print success message "Created .code-viz.toml with default configuration" to stdout, return ConfigError if write fails | Restrictions: Do not overwrite existing config (ask user or return error), template must be valid TOML, include helpful comments explaining each option, use fs::write (not manual File::create) | Leverage: design.md config template example, requirements.md Req 5.3 acceptance criteria | Requirements: Req 5.3 (config init), NFR Usability | Success: code-viz config init creates .code-viz.toml file, file is valid TOML and can be parsed, file contains helpful comments, command fails if file already exists | Instructions: Mark in-progress [-]. Test command manually. Log config template content + init command behavior. Mark complete [x]._

### 2.3 CLI Integration Tests

- [x] 2.3.1 Implement E2E CLI tests (cli_e2e_test.rs)
  - Files: `crates/code-viz-cli/tests/cli_e2e_test.rs`
  - Purpose: Test CLI binary end-to-end with assert_cmd framework
  - _Leverage: `assert_cmd`, `assert_fs`, `predicates`, design.md E2E test scenarios_
  - _Requirements: NFR - Autonomous E2E Testing_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: QA Automation Engineer specializing in CLI testing and end-to-end validation | Task: Replace todo!() in cli_e2e_test.rs with real E2E tests from design.md: test_e2e_analyze_json_output() creates temp dir with 2 test files using assert_fs, runs Command::cargo_bin("code-viz") analyze <temp_path> --format json, asserts success(), asserts stdout contains "\"total_files\":2", test_e2e_threshold_violation() creates file with 600 LOC, runs analyze with --threshold loc=500, asserts failure(), asserts stderr contains "has 600 LOC (limit: 500)", test_e2e_config_file_integration() creates .code-viz.toml in temp dir with exclusions, creates files in tests/ and src/, verifies only src/ files analyzed, test_e2e_analyze_text_output() tests default text format, test_e2e_csv_output() verifies CSV format | Restrictions: Use assert_fs::TempDir for isolation, tests must clean up automatically, use predicates::str::contains for output assertions, tests must be fast (<5s each), do not rely on external files (create fixtures in code) | Leverage: design.md "Autonomous End-to-End CLI Testing" section with 5 test scenarios, assert_cmd/assert_fs documentation | Requirements: All Req 1.x (CLI commands), Req 6.x (exports), NFR - Autonomous E2E Testing | Success: cargo test --test cli_e2e_test passes all tests, tests run in CI without manual intervention, all exit codes verified, stdout/stderr outputs validated | Instructions: Mark in-progress [-]. Run tests locally and in CI. Log E2E test coverage (5+ scenarios) + test execution results. Mark complete [x]._

## Phase 3: Watch Mode (Week 3)

### 3.1 File Watcher Integration

- [x] 3.1.1 Implement watch command with file monitoring (commands/watch.rs)
  - Files: `crates/code-viz-cli/src/commands/watch.rs`
  - Purpose: Monitor file system and re-analyze on changes
  - _Leverage: `notify` crate, debouncing logic, design.md watch component_
  - _Requirements: Req 3 (Real-time monitoring), NFR Performance (100ms updates)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Systems Developer specializing in event-driven programming and file system monitoring | Task: Replace todo!() in commands/watch.rs run() function: perform initial analysis and print results, create notify::RecommendedWatcher, watch path with watcher.watch(&path, RecursiveMode::Recursive), create channel for file events, implement debouncing (collect events for 100ms using std::time::Duration, batch changes), on file change: determine which files changed (filter for .ts/.js/.rs/.py), re-analyze only changed files (incremental), format and print updated metrics (if --format json, print newline-delimited JSON for each update, else print "[timestamp] path: X LOC (+Y)" format), handle Ctrl+C gracefully (listen for SIGINT, print "Stopping watch mode..." and exit), log errors to stderr and continue watching (don't exit on single-file errors) | Restrictions: Must debounce events (max 1 update/sec), incremental analysis only (don't re-scan entire tree), newline-delimited JSON for streaming (each line is valid JSON object), respect verbose flag for debug output, use crossbeam_channel or std::sync::mpsc for event channel | Leverage: design.md "Component 7: Watch Mode" implementation, requirements.md Req 3.1-3.5 acceptance criteria for watch behavior | Requirements: Req 3.1-3.5 (watch mode), NFR Performance (100ms incremental), NFR Reliability (continue on errors) | Success: cargo run -- watch tests/fixtures monitors changes, modifying file triggers re-analysis within 200ms, JSON stream is valid newline-delimited JSON, Ctrl+C stops gracefully, errors don't crash watcher | Instructions: Mark in-progress [-]. Test watch mode by editing files. Log watch implementation + event handling logic + debouncing mechanism. Mark complete [x]._

- [x] 3.1.2 Add watch mode E2E tests (cli_e2e_test.rs)
  - Files: `crates/code-viz-cli/tests/cli_e2e_test.rs`
  - Purpose: Test watch mode with background process and file modifications
  - _Leverage: `assert_cmd` with `.spawn()`, `assert_fs`, design.md watch test scenario_
  - _Requirements: NFR - Autonomous E2E Testing_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: QA Automation Engineer specializing in asynchronous testing and process management | Task: Add test_e2e_watch_mode() to cli_e2e_test.rs: create temp dir with test file, spawn watch command as background process (Command::cargo_bin("code-viz").arg("watch").arg(temp.path()).stdout(Stdio::piped()).spawn()), wait 500ms for initial analysis, modify test file (add new function), wait 500ms, read stdout from process (use BufReader to read lines), parse JSON or text output, verify updated function_count, kill process with child.kill(), verify process exits cleanly, add test_e2e_watch_json_stream() testing newline-delimited JSON format (each line is valid JSON) | Restrictions: Use tokio::time::sleep or std::thread::sleep for waits (keep waits minimal, max 1s total test time), tests must kill background process in all code paths (use defer/drop pattern), stdout must be piped and read asynchronously, handle flaky timing with retry logic if necessary | Leverage: design.md E2E test scenario #2 "Watch Mode Workflow", assert_cmd docs for spawning background processes | Requirements: Req 3.2-3.4 (watch detects changes), NFR - Autonomous E2E Testing | Success: cargo test test_e2e_watch_mode passes reliably, background process is killed even if test fails, watch mode correctly detects file changes, test completes in <3s | Instructions: Mark in-progress [-]. Run test multiple times to verify reliability. Log watch E2E test implementation + background process handling. Mark complete [x]._

## Phase 4: CI/CD Features (Week 4)

### 4.1 Diff Command

- [x] 4.1.1 Implement diff command for comparing reports (commands/diff.rs)
  - Files: `crates/code-viz-cli/src/commands/diff.rs`
  - Purpose: Compare two JSON analysis reports and show differences
  - _Leverage: `serde_json` for parsing reports, design.md diff algorithm_
  - _Requirements: Req 6.4 (diff command)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Data Analysis Engineer specializing in diff algorithms and reporting | Task: Implement commands/diff.rs run() function: parse DiffArgs (old_report: PathBuf, new_report: PathBuf), read both files with fs::read_to_string(), deserialize into AnalysisResult with serde_json::from_str(), compute differences: files_added (in new, not in old), files_deleted (in old, not in new), files_modified (in both, LOC changed), total_loc_delta (new.total_loc - old.total_loc), largest_growth (file with biggest LOC increase), print human-readable diff output: "+12 files added\n-3 files deleted\nTotal LOC: 45,678 → 48,901 (+3,223)\nLargest growth: src/api.ts (+234 LOC)", use colored crate for green/red output (respect NO_COLOR env), return DiffError for parse failures | Restrictions: Must handle case where file was renamed (same content, different path - ignore for MVP), output should be concise (not full file list, just summary), use BTreeMap to match files by path efficiently, colorize output (green for additions, red for deletions, use colored crate) | Leverage: design.md diff command specification, requirements.md Req 6.4 acceptance criteria for output format | Requirements: Req 6.4 (diff command), NFR Usability | Success: cargo run -- diff old.json new.json shows correct diff, handles added/deleted/modified files, output is human-readable, colors work (test with NO_COLOR=1 to disable) | Instructions: Mark in-progress [-]. Test with sample JSON reports. Log diff implementation + output format. Mark complete [x]._

- [x] 4.1.2 Implement baseline comparison for CI (analyze command enhancement)
  - Files: `crates/code-viz-cli/src/commands/analyze.rs` (modify existing)
  - Purpose: Add --baseline flag to fail CI if metrics degrade
  - _Leverage: Diff logic, exit code 3 for threshold violations_
  - _Requirements: Req 6.5 (baseline comparison)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: DevOps Engineer specializing in CI/CD quality gates and automation | Task: Modify analyze command to support --baseline flag: if baseline arg is Some(path), load baseline report from path (deserialize AnalysisResult), compare current result to baseline, check if total_loc increased by >10% (configurable threshold, hardcode 10% for MVP), if exceeded print error "Total LOC increased by 15% (limit: 10%)" to stderr and exit with code 3, else proceed normally, print comparison summary "Total LOC: 45,678 → 48,901 (+7%, within threshold)" to stdout | Restrictions: Baseline comparison only checks total_loc for MVP (not per-file), exit code 3 indicates threshold violation (same as --threshold flag), use same exit code semantics, baseline file must be valid JSON AnalysisResult | Leverage: design.md baseline comparison semantics, requirements.md Req 6.5 acceptance criteria | Requirements: Req 6.5 (baseline comparison), NFR Usability (CI integration) | Success: analyze --baseline old.json exits with code 3 if LOC increased >10%, exits with 0 if within threshold, error message is clear, works in CI pipeline (test with exit code check) | Instructions: Mark in-progress [-]. Test with baseline comparison. Log baseline feature + CI usage example. Mark complete [x]._

### 4.2 Final Polish and Documentation

- [x] 4.2.1 Add comprehensive README with usage examples
  - Files: `README.md` (root, replace skeleton)
  - Purpose: Document installation, usage, and examples
  - _Leverage: All implemented commands, design.md examples_
  - _Requirements: NFR - Usability (Documentation)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Technical Writer specializing in developer documentation and CLI tool guides | Task: Replace README.md skeleton with comprehensive documentation: sections include Project Overview (what is code-viz, why use it), Installation (cargo install --path crates/code-viz-cli or download binary), Quick Start (code-viz analyze ./src, code-viz watch ./src), Commands Reference (analyze with all flags, watch, diff, config init), Configuration (.code-viz.toml example), CI/CD Integration (GitHub Actions example workflow showing analyze with baseline check), Output Formats (JSON/CSV/text examples), Examples (common recipes like "Find files >500 LOC", "Track metrics over time", "Fail CI on code bloat"), Contributing (how to build, test, contribute), License | Restrictions: Use clear headings and code blocks, include actual command outputs (copy from real runs), keep concise (max 500 lines), add badges (build status, license) if applicable, use GIF or screenshot for treemap visualization (mention it's future feature) | Leverage: requirements.md for all CLI commands, design.md for architecture overview, CI/CD examples from Req 6.5 | Requirements: All requirements (documentation of features), NFR Usability | Success: README is comprehensive and clear, covers all commands, includes real examples, can be used for onboarding new users | Instructions: Mark in-progress [-]. Write README incrementally. Log README sections created + examples included. Mark complete [x]._

- [x] 4.2.2 Create GitHub Actions CI workflow
  - Files: `.github/workflows/ci.yml`
  - Purpose: Automate testing and code quality checks in CI
  - _Leverage: GitHub Actions, design.md CI requirements_
  - _Requirements: NFR - Reliability (CI testing)_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: DevOps Engineer specializing in GitHub Actions and CI/CD pipelines | Task: Create .github/workflows/ci.yml with jobs: test (runs on ubuntu-latest, macos-latest, windows-latest), steps include actions/checkout@v4, install Rust (use dtolnay/rust-toolchain@stable), cache cargo dependencies (use Swatinem/rust-cache@v2), run cargo check, cargo clippy -- -D warnings, cargo test --all-features, cargo build --release, lint (runs cargo fmt --check), dogfood (runs cargo run -p code-viz-cli -- analyze . --format json --threshold loc=500, use this to test code-viz on itself), add sccache caching for faster builds (use mozilla/sccache-action@v0.0.3), set RUSTC_WRAPPER=sccache | Restrictions: CI must pass on all 3 platforms, use matrix strategy for OS testing, clippy must pass with zero warnings (-D warnings), dogfood step should analyze code-viz repo itself (validate tool works), cache Cargo.toml dependencies for speed | Leverage: design.md CI integration section, GitHub Actions best practices for Rust | Requirements: NFR Reliability (automated testing), NFR Performance (cached builds) | Success: GitHub Actions workflow runs successfully on PR, all tests pass on 3 platforms, clippy has no warnings, dogfood step analyzes code-viz repo, builds are cached and fast (<5 min total) | Instructions: Mark in-progress [-]. Test workflow locally with act or push to test branch. Log CI workflow created + platforms tested. Mark complete [x]._

- [ ] 4.2.3 Final verification and v0.1.0 release preparation
  - Files: `Cargo.toml` (version bump), `CHANGELOG.md`, Git tag
  - Purpose: Prepare for initial MVP release
  - _Leverage: All implemented features, semantic versioning_
  - _Requirements: All MVP requirements complete_
  - _Prompt: Implement the task for spec mvp, first run spec-workflow-guide to get the workflow guide then implement the task: | Role: Release Manager specializing in versioning and release processes | Task: Prepare for v0.1.0 release: update Cargo.toml version to "0.1.0" in all crates, create CHANGELOG.md with sections (Added: list all features, Fixed: list bugs fixed during development, Known Limitations: list out-of-scope features from requirements.md), run final verification (cargo test --all, cargo clippy, cargo build --release, manual smoke test on real repo), commit with message "chore: prepare v0.1.0 release", create git tag v0.1.0, prepare GitHub Release draft with CHANGELOG content and built binaries (optional, can use cargo-dist for this) | Restrictions: Version must be 0.1.0 (first MVP release), CHANGELOG must follow Keep a Changelog format, git tag must be annotated (git tag -a v0.1.0 -m "MVP release"), do not publish to crates.io yet (wait for post-MVP feedback) | Leverage: semantic versioning spec, requirements.md "Out of Scope" section for Known Limitations | Requirements: All MVP requirements implemented and tested | Success: Version is 0.1.0, CHANGELOG is complete and accurate, git tag created, all tests pass, binary builds successfully, ready for GitHub Release | Instructions: Mark in-progress [-]. Run full verification. Log release checklist completion. Mark complete [x]._

## Verification & Completion

After all tasks complete:
- All requirements from requirements.md are implemented and tested
- All E2E tests pass autonomously (`cargo test`)
- CI/CD pipeline is green on all platforms
- README is comprehensive with examples
- Codebase follows structure.md conventions
- Performance benchmarks meet targets (100K files in <30s)
- Binary size is reasonable (<10MB release build)
- Ready for v0.1.0 MVP release

**Final Deliverable**: Fully functional CLI tool with:
- ✅ Repository analysis (TypeScript/JavaScript support)
- ✅ JSON/CSV/text output formats
- ✅ Watch mode for real-time feedback
- ✅ Configuration file support
- ✅ CI/CD integration (thresholds, diff, baseline)
- ✅ Comprehensive tests (unit, integration, E2E)
- ✅ Complete documentation
