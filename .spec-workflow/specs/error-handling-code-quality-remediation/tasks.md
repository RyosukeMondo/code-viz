# Tasks Document

## Error Handling and Code Quality Remediation

**Spec Name**: error-handling-code-quality-remediation
**Total Tasks**: 45 tasks across 7 phases
**Estimated Duration**: 8 weeks

---

## Phase 1: Foundation (Week 1)

### Build error handling infrastructure and development tooling

- [x] 1. Create unified error type hierarchy
  - **File**: `crates/code-viz-core/src/error.rs`
  - **Purpose**: Establish centralized error types to replace 358 unwrap() calls with proper error propagation
  - **Requirements**: FR1, FR2, NFR1
  - **Leverage**: Existing thiserror pattern in codebase (if any)
  - **Prompt**: **Role**: Rust Systems Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create a comprehensive error type hierarchy in `crates/code-viz-core/src/error.rs` that consolidates all error handling across the code-viz codebase.

    **Error Types to Define**:
    - `CodeVizError` enum with variants: ParseError, CoverageDataMissing, AnalysisFailed, FileSystem, Git
    - ParseError should include: language (String), path (PathBuf), line (usize), source (boxed error)
    - Each variant must have `#[error("...")]` attribute with descriptive message format
    - Implement `From<std::io::Error>` for automatic conversion
    - Add `context()` method to wrap errors with additional context

    **Dependencies**:
    - Add `thiserror = "2.0"` to `code-viz-core/Cargo.toml`
    - Use `#[derive(Error, Debug)]` macro

    **Documentation**:
    - Add rustdoc comments explaining when to use each error variant
    - Include usage examples showing error propagation with ?
    - Document error message format conventions

  | **Restrictions**: File size ≤200 lines, no dependencies beyond thiserror, errors must be Send + Sync, all variants must have Display implementation via thiserror
  | **Success**: ✅ `error.rs` compiles without warnings, ✅ All error variants have descriptive messages, ✅ rustdoc examples compile and run, ✅ Error types implement std::error::Error
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (error type definitions, From implementations, public API functions), (4) Mark this task as complete [x] in tasks.md

- [x] 2. Create error conversion helpers
  - **File**: `crates/code-viz-core/src/error_ext.rs`
  - **Purpose**: Provide ergonomic helpers to reduce error mapping duplication (currently 87 map_err occurrences)
  - **Requirements**: FR2, NFR1
  - **Leverage**: Task 1 error types
  - **Prompt**: **Role**: Rust Developer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create error conversion utilities to simplify error handling throughout the codebase.

    **ResultExt Trait**:
    - Define `ResultExt<T>` trait with `with_context` method
    - Implement for `Result<T, E> where E: Into<CodeVizError>`
    - Context closure should be `FnOnce() -> String` for lazy evaluation

    **Conversion Macros** (optional but recommended):
    - `io_error!` macro for IoError conversion
    - `parse_error!` macro with file and language context
    - Each macro should reduce 3-5 lines to 1 line

    **Tests**:
    - Unit test showing Result conversion with context
    - Test demonstrating macro usage
    - Compile-fail test for invalid conversions

  | **Restrictions**: File size ≤150 lines, no external dependencies beyond std, macros must have hygiene (use $crate::), trait methods must be inline
  | **Success**: ✅ ResultExt trait compiles and works with ?, ✅ At least 2 helper macros defined, ✅ Unit tests pass, ✅ Documentation with usage examples
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (trait definitions, macro implementations, test utilities), (4) Mark this task as complete [x] in tasks.md

- [x] 3. Add error handling dependencies
  - **File**: `Cargo.toml` (workspace root)
  - **Purpose**: Add required error handling crates to workspace dependencies
  - **Requirements**: Design Section 6.1
  - **Prompt**: **Role**: DevOps Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Add error handling dependencies to the workspace Cargo.toml.

    **Workspace Dependencies to Add**:
    - `thiserror = "2.0"` for error derive macros
    - `anyhow = "1.0"` for context-aware errors (CLI layer)

    **Update Each Crate**:
    - `code-viz-core`: Add `thiserror` with workspace = true
    - `code-viz-cli`: Add both `thiserror` and `anyhow` with workspace = true
    - `code-viz-api`, `code-viz-tauri`: Add `thiserror` with workspace = true

    **Verification**:
    - Run `cargo check --workspace` to ensure dependencies resolve
    - Verify no version conflicts

  | **Restrictions**: Use exact versions specified, workspace dependencies only, no additional deps without approval
  | **Success**: ✅ cargo check passes, ✅ All crates can access thiserror, ✅ No dependency conflicts
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (dependency additions, version specifications), (4) Mark this task as complete [x] in tasks.md

- [x] 4. Create error handling migration guide
  - **File**: `MIGRATION.md` (project root)
  - **Purpose**: Document error handling patterns for contributors converting unwrap() calls
  - **Requirements**: NFR4
  - **Prompt**: **Role**: Technical Writer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create comprehensive migration guide for error handling refactoring.

    **Guide Sections**:
    - **Overview**: Explain the error type hierarchy
    - **Common Patterns**: Show before/after for typical unwrap() scenarios
    - **Parser Errors**: How to handle tree-sitter parser failures
    - **Coverage Errors**: How to handle missing coverage data gracefully
    - **File I/O**: Pattern for file operation error handling
    - **Testing**: How to test error paths

    **Code Examples** (at least 5):
    - Parser unwrap → Result with ParseError
    - Coverage unwrap → unwrap_or with default
    - File read unwrap → ? with context
    - map_err duplication → ResultExt usage
    - Test error scenario example

    **Anti-Patterns to Avoid**:
    - Using unwrap() "because we know it won't fail"
    - Silently ignoring errors
    - Generic error messages without context

  | **Restrictions**: File size ≤300 lines, all code examples must compile, use real codebase examples where possible
  | **Success**: ✅ All examples compile, ✅ Covers all common scenarios, ✅ Clear anti-patterns section, ✅ Links to tasks.md and design.md
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (documentation sections, code examples), (4) Mark this task as complete [x] in tasks.md

- [x] 5. Setup pre-commit hooks for metrics
  - **Files**: `.husky/pre-commit`, `scripts/check-metrics.sh`
  - **Purpose**: Prevent regression by blocking commits that violate code quality metrics
  - **Requirements**: NFR1, NFR5
  - **Prompt**: **Role**: DevOps Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Setup automated pre-commit checks to enforce code quality metrics.

    **Install Husky** (if not already present):
    - Add husky to package.json devDependencies
    - Run `npx husky install`

    **Create check-metrics.sh Script**:
    - Check 1: File sizes (fail if any .rs or .tsx file >500 lines excluding comments)
    - Check 2: Function sizes (warn if function >50 lines, fail if >100)
    - Check 3: unwrap() in staged files (fail if found in production code, allow in tests)
    - Check 4: TypeScript any usage (warn if found, provide guidance)
    - Script must be executable: `chmod +x scripts/check-metrics.sh`

    **Pre-commit Hook**:
    - Run check-metrics.sh
    - Run `cargo clippy -- -D warnings` on staged Rust files
    - Run `npm run lint` on staged TypeScript files
    - Exit 1 if any check fails

    **Developer Experience**:
    - Clear error messages explaining what failed
    - Suggest fix commands where possible
    - Allow bypass with --no-verify (documented in CONTRIBUTING.md)

  | **Restrictions**: Script must be fast (<5 seconds), only check staged files, clear error output, cross-platform compatible (bash)
  | **Success**: ✅ Hook blocks commits with unwrap(), ✅ Hook blocks >500 line files, ✅ Hook runs clippy and lint, ✅ Clear failure messages
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (shell scripts, hook configurations, check implementations), (4) Mark this task as complete [x] in tasks.md

---

## Phase 2: Critical Error Elimination (Week 2)

### Replace all 358 unwrap() calls with proper error handling

- [x] 6. Fix code-viz-core unwrap() calls
  - **Files**: All production .rs files in `crates/code-viz-core/src/`
  - **Purpose**: Eliminate 197 unwrap() calls causing crash risk in core analysis logic
  - **Requirements**: FR1, NFR1
  - **Leverage**: Tasks 1-2 (error types and helpers)
  - **Prompt**: **Role**: Rust Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Replace all unwrap() and expect() calls in code-viz-core production code with proper error handling.

    **Priority Files** (by impact):
    1. `coupling.rs` - Parser unwrap() at lines 34-36 (CRITICAL)
    2. `coverage.rs` - Coverage unwrap() at lines 179-244 (CRITICAL)
    3. `duplication.rs` - Analysis unwrap() calls
    4. `parser.rs` - Tree-sitter unwrap() calls
    5. `metrics.rs`, `hotspot.rs`, `scanner.rs` - Remaining unwrap() calls

    **Conversion Patterns**:
    - Parser unwrap → `ok_or_else(|| ParseError { ... })?`
    - Coverage unwrap → `as_ref().map(|c| c.value).unwrap_or(default)`
    - File I/O unwrap → `.with_context(|| format!("reading {}", path.display()))?`
    - Collection unwrap → `.ok_or(AnalysisFailed { ... })?`

    **Error Handling Strategy**:
    - Critical failures: Return error (parser failures)
    - Non-critical: Use defaults (missing coverage)
    - Log warnings for degraded functionality

    **Testing**:
    - Add error scenario tests for each converted unwrap
    - Test parser failure recovery
    - Test missing coverage graceful handling

  | **Restrictions**: Zero unwrap() in production code (tests OK with comment), must use CodeVizError types, preserve existing behavior, all errors must have context
  | **Success**: ✅ 0 unwrap() in production .rs files, ✅ Error tests added for critical paths, ✅ Existing tests still pass, ✅ Graceful degradation for non-critical errors
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (error handling conversions, test cases, graceful fallback logic), (4) Mark this task as complete [x] in tasks.md

- [x] 7. Fix code-viz-dead-code unwrap() calls
  - **Files**: All .rs files in `crates/code-viz-dead-code/src/`
  - **Purpose**: Eliminate 72 unwrap() calls in dead code analysis
  - **Requirements**: FR1, NFR1
  - **Leverage**: Tasks 1-2, Task 6 patterns
  - **Prompt**: **Role**: Rust Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Replace all unwrap() calls in code-viz-dead-code crate with proper error handling following patterns from task 6.

    **Priority Files**:
    1. `lib.rs:140` - Graph building unwrap() calls
    2. `symbol_graph/builder.rs` - Parser unwrap() at lines 172-180
    3. `confidence.rs` - Test code unwrap() (add comments only)
    4. Other module files with unwrap()

    **Dead Code Specific Errors**:
    - Define `DeadCodeError` enum wrapping `CodeVizError`
    - Add variants: GraphBuildFailed, SymbolNotFound, EntryPointMissing
    - Use context to explain which symbol/file caused error

    **Symbol Graph Errors**:
    - Parser failures → Skip file with warning, continue analysis
    - Missing exports → Default to no exports, log warning
    - Type resolution failures → Mark symbol as UnknownType

    **Testing**:
    - Test graph building with malformed files
    - Test missing symbol scenarios
    - Verify partial analysis results on errors

  | **Restrictions**: Zero unwrap() in production, test unwrap() must have `// Test-only unwrap: [reason]` comment, errors must be actionable
  | **Success**: ✅ 0 production unwrap(), ✅ Test unwrap() annotated, ✅ DeadCodeError properly wraps core errors, ✅ Partial results on non-critical failures
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (DeadCodeError definitions, error recovery logic, test additions), (4) Mark this task as complete [x] in tasks.md

- [x] 8. Fix code-viz-cli unwrap() calls
  - **Files**: All .rs files in `crates/code-viz-cli/src/`
  - **Purpose**: Eliminate 54 unwrap() calls in CLI layer
  - **Requirements**: FR1, FR2, NFR1
  - **Leverage**: Tasks 1-2, anyhow for CLI error context
  - **Prompt**: **Role**: Rust CLI Developer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Replace all unwrap() calls in CLI code with user-friendly error handling.

    **CLI Error Strategy**:
    - Define `CliError` wrapping `CodeVizError`
    - Variants: InvalidArgs, CommandFailed, ConfigError
    - Implement Display to provide user-friendly messages
    - Use anyhow::Context for error chains

    **Priority Files**:
    1. `commands/analyze.rs` - File I/O unwrap()
    2. `commands/dead_code.rs` - Result unwrap()
    3. `main.rs` - Argument parsing unwrap()
    4. `config_loader.rs` - Config parsing unwrap()

    **User-Facing Error Messages**:
    - Parser errors → "Failed to parse [file]. Is this a valid [language] file?"
    - Missing file → "File not found: [path]. Check the path and try again."
    - Analysis failed → "Analysis failed: [context]. See --verbose for details."

    **Logging Integration**:
    - ERROR level: Analysis failures
    - WARN level: Degraded functionality
    - Use --verbose flag to show full error chains

  | **Restrictions**: All errors must have user-friendly Display, no panic!() in CLI code, preserve exit codes, verbose mode shows technical details
  | **Success**: ✅ 0 unwrap() in CLI, ✅ User-friendly error messages, ✅ --verbose shows full context, ✅ Proper exit codes (0 success, 1 error)
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (CliError definitions, Display implementations, error message templates), (4) Mark this task as complete [x] in tasks.md

- [x] 9. Fix code-viz-api unwrap() calls
  - **Files**: All .rs files in `crates/code-viz-api/src/`
  - **Purpose**: Eliminate 21 unwrap() calls in API layer
  - **Requirements**: FR1, FR2
  - **Leverage**: Tasks 1-2, HTTP error status codes
  - **Prompt**: **Role**: Rust Backend Developer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Replace all unwrap() calls in API code with proper HTTP error responses.

    **API Error Strategy**:
    - Define `ApiError` wrapping `CodeVizError`
    - Map errors to HTTP status codes:
      - ParseError, AnalysisFailed → 500 Internal Server Error
      - InvalidArgs, BadRequest → 400 Bad Request
      - FileNotFound → 404 Not Found
    - JSON error response format: `{ "error": "message", "details": "..." }`

    **Priority Files**:
    1. `transform.rs` - Tree building unwrap()
    2. API endpoint handlers - Request parsing unwrap()
    3. Response serialization unwrap()

    **Error Response Format**:
    ```json
    {
      "error": "Analysis failed",
      "message": "Failed to parse src/main.rs",
      "details": "Unexpected token at line 42",
      "path": "/api/analyze"
    }
    ```

    **Testing**:
    - Test error responses have correct status codes
    - Test error JSON is well-formed
    - Test sensitive info not leaked in errors

  | **Restrictions**: Zero unwrap() in API code, errors must map to HTTP status, no stack traces in production responses, JSON must be valid
  | **Success**: ✅ 0 unwrap() in API, ✅ Proper HTTP status codes, ✅ Structured JSON errors, ✅ No sensitive data in errors
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (ApiError definitions, HTTP mappings, error response structures), (4) Mark this task as complete [x] in tasks.md

- [x] 10. Fix code-viz-tauri unwrap() calls
  - **Files**: All .rs files in `crates/code-viz-tauri/src/`
  - **Purpose**: Eliminate 16 unwrap() calls in Tauri layer
  - **Requirements**: FR1, FR2
  - **Leverage**: Tasks 1-2, Tauri error handling
  - **Prompt**: **Role**: Tauri Developer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Replace all unwrap() calls in Tauri code with proper IPC error handling.

    **Tauri Error Strategy**:
    - Define `TauriError` wrapping `CodeVizError`
    - Implement `Into<tauri::InvokeError>` for automatic conversion
    - Error messages sent to frontend via IPC

    **Priority Files**:
    1. `transform.rs` - Tree building unwrap() (note: will be refactored in task 21)
    2. Command handlers - IPC unwrap()
    3. File system operations

    **Frontend Error Integration**:
    - Errors propagate to JavaScript as rejected Promises
    - Include error code for programmatic handling
    - User-friendly message in error.message

    **Testing**:
    - Test Tauri commands handle errors gracefully
    - Test frontend receives structured errors
    - Integration test for error scenarios

  | **Restrictions**: Zero unwrap() in Tauri code, errors must serialize to JSON, frontend must receive actionable errors
  | **Success**: ✅ 0 unwrap() in Tauri, ✅ Errors propagate to frontend, ✅ Structured error format, ✅ Integration tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (TauriError definitions, IPC error handling, frontend integration), (4) Mark this task as complete [x] in tasks.md

- [x] 11. Add error scenario tests
  - **Files**: Test files across all crates
  - **Purpose**: Ensure error paths are tested and work correctly
  - **Requirements**: FR1, NFR1
  - **Leverage**: All error types from tasks 6-10
  - **Prompt**: **Role**: QA Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create comprehensive error scenario tests validating all error paths.

    **Test Coverage Required**:
    - Parser failures (malformed input)
    - Missing files (FileNotFound scenarios)
    - Missing coverage data (graceful degradation)
    - Network errors (if applicable)
    - Permission errors
    - Invalid arguments

    **Test Structure** (per crate):
    ```rust
    #[cfg(test)]
    mod error_tests {
        #[test]
        fn test_parser_error_on_malformed_input() { ... }

        #[test]
        fn test_missing_coverage_uses_default() { ... }

        #[test]
        fn test_file_not_found_returns_error() { ... }
    }
    ```

    **Negative Testing**:
    - Verify errors contain expected information
    - Test error message formatting
    - Test error source chains (cause tracking)

    **Integration Tests**:
    - Full pipeline with error injection
    - Recovery from partial failures
    - Error aggregation for batch operations

  | **Restrictions**: All error paths must be tested, use assert_matches! for error variants, no test code duplicating production logic
  | **Success**: ✅ All error variants tested, ✅ Error messages validated, ✅ Integration tests for error recovery, ✅ Tests pass consistently
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (test cases, error fixtures, assertion helpers), (4) Mark this task as complete [x] in tasks.md

- [x] 12. Validate CI blocks new unwrap() calls
  - **Files**: `.github/workflows/ci.yml`, `scripts/check-unwrap.sh`
  - **Purpose**: Ensure CI prevents introduction of new unwrap() in production code
  - **Requirements**: NFR1
  - **Leverage**: Pre-commit hook from task 5
  - **Prompt**: **Role**: DevOps Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Add CI checks to block PRs containing unwrap() in production code.

    **CI Workflow Addition**:
    ```yaml
    - name: Check for unwrap() in production code
      run: |
        ./scripts/check-unwrap.sh
        if [ $? -ne 0 ]; then
          echo "❌ Found unwrap() in production code"
          exit 1
        fi
    ```

    **check-unwrap.sh Script**:
    - Find all .rs files excluding tests/ directories
    - Grep for unwrap() and expect()
    - Exclude test files (path contains /tests/ or ends in _test.rs)
    - Exclude test modules (#[cfg(test)])
    - Report file:line for each violation
    - Exit 1 if any found

    **CI Failure Message**:
    - List all files with unwrap()
    - Link to MIGRATION.md for guidance
    - Suggest proper error handling pattern

    **Testing**:
    - Create test PR with unwrap() → verify CI fails
    - Create test PR without unwrap() → verify CI passes

  | **Restrictions**: Must catch unwrap() and expect(), must allow test code unwrap(), fast execution (<30 seconds), clear error messages
  | **Success**: ✅ CI fails on unwrap() in production, ✅ CI passes on test unwrap(), ✅ Clear failure message with fix guidance, ✅ Test PRs validate behavior
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (CI workflow steps, shell scripts, test validations), (4) Mark this task as complete [x] in tasks.md

---

## Phase 3: Function Size Reduction (Week 3)

### Break down all functions >50 lines

- [x] 13. Refactor analyze.rs:52 (CLI)
  - **File**: `crates/code-viz-cli/src/commands/analyze.rs`
  - **Purpose**: Reduce run() function from 136 → <40 lines
  - **Requirements**: FR3, NFR1
  - **Leverage**: Task 6 error handling
  - **Prompt**: **Role**: Rust Refactoring Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor the 136-line `run()` function into focused helper functions following Single Responsibility Principle.

    **Current Function**: `analyze.rs:52-188` (136 lines)

    **Extraction Plan**:
    1. Create `AnalysisConfig::from_args()` - parameter building (20 lines)
    2. Create `AnalysisOrchestrator` struct with methods:
       - `run_base_analysis()` - core analysis (15 lines)
       - `enrich_with_duplication()` - duplication if enabled (12 lines)
       - `enrich_with_hotspots()` - hotspots if enabled (12 lines)
       - `enrich_with_coverage()` - coverage if enabled (12 lines)
       - `enrich_with_dead_code()` - dead code if enabled (15 lines)
       - `enrich_with_ai_commits()` - AI commits if enabled (15 lines)
       - `apply_baseline()` - baseline comparison if present (12 lines)
    3. Main `run()` function → orchestration (30 lines)

    **New Structure**:
    ```rust
    pub async fn run(args: AnalyzeArgs) -> Result<(), CliError> {
        let config = AnalysisConfig::from_args(args)?;
        let orchestrator = AnalysisOrchestrator::new(config)?;
        let result = orchestrator.execute().await?;
        output_result(&result, &config.output)?;
        Ok(())
    }
    ```

    **Testing**:
    - Preserve all existing test behavior
    - Add tests for each extracted function
    - Verify orchestration order unchanged

  | **Restrictions**: Main function ≤40 lines, each helper ≤20 lines, preserve exact behavior, all tests pass
  | **Success**: ✅ run() ≤40 lines, ✅ All helpers ≤20 lines, ✅ Existing tests pass, ✅ New helper tests added
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (AnalysisConfig struct, AnalysisOrchestrator class and methods, helper function extractions), (4) Mark this task as complete [x] in tasks.md

- [x] 14. Refactor builder.rs:160 (dead-code)
  - **File**: `crates/code-viz-dead-code/src/symbol_graph/builder.rs`
  - **Purpose**: Reduce build_graph() from 138 → <50 lines
  - **Requirements**: FR3, NFR1
  - **Leverage**: Task 7 error handling
  - **Prompt**: **Role**: Rust Refactoring Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor the 138-line symbol graph builder into focused functions.

    **Current Function**: `builder.rs:160-298` (138 lines)

    **Extraction Plan**:
    1. `extract_symbols_from_file()` - Parse and extract (25 lines)
    2. `build_dependency_edges()` - Create edges (20 lines)
    3. `resolve_symbol_types()` - Type resolution (25 lines)
    4. `handle_exports()` - Export handling (15 lines)
    5. Main `build_graph()` - Orchestration (40 lines)

    **Error Handling**:
    - Parser failures → skip file, log warning, continue
    - Missing symbols → create UnknownSymbol node
    - Type resolution failures → mark as UnknownType

    **Performance**:
    - Ensure no performance regression (benchmark if needed)
    - Preserve parallel processing if present

  | **Restrictions**: Main function ≤50 lines, helpers ≤25 lines, no behavior changes, handle errors gracefully
  | **Success**: ✅ build_graph() ≤50 lines, ✅ Helpers well-named and focused, ✅ Tests pass, ✅ Errors handled properly
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (extracted symbol functions, dependency edge builders, error recovery logic), (4) Mark this task as complete [x] in tasks.md

- [x] 15. Refactor transform.rs:128 (API/Tauri)
  - **Files**: `crates/code-viz-api/src/transform.rs`, `crates/code-viz-tauri/src/transform.rs`
  - **Purpose**: Reduce flat_to_hierarchy() from 130 → <50 lines (NOTE: Full deduplication in task 21)
  - **Requirements**: FR3, FR5
  - **Leverage**: Task 9-10 error handling
  - **Prompt**: **Role**: Rust Refactoring Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor the duplicated transform logic to reduce function size (preparation for task 21 which will fully deduplicate).

    **Current Functions**:
    - `api/transform.rs:128-258` (130 lines)
    - `tauri/transform.rs:128-258` (130 lines DUPLICATE)

    **Extraction Plan** (apply to both files identically):
    1. `build_tree_nodes()` - Create node structure (25 lines)
    2. `populate_file_metrics()` - Add metrics to leaves (20 lines)
    3. `aggregate_directory_metrics()` - Roll up metrics (30 lines)
    4. Main `flat_to_hierarchy()` - Orchestration (45 lines)

    **NOTE**: This task reduces function size but keeps duplication. Task 21 will move this logic to shared core module.

    **Testing**:
    - Ensure both API and Tauri produce identical output
    - Add property-based tests for tree structure validity

  | **Restrictions**: Both files must remain identical, main function ≤50 lines, helpers ≤30 lines, preserve behavior exactly
  | **Success**: ✅ flat_to_hierarchy() ≤50 lines in both files, ✅ Helpers identical across files, ✅ Tests pass, ✅ Output unchanged
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (tree node builders, metric aggregators, helper extractions), (4) Mark this task as complete [x] in tasks.md

- [x] 16. Refactor duplication.rs:28
  - **File**: `crates/code-viz-core/src/duplication.rs`
  - **Purpose**: Reduce analyze_duplication() from 191 → <50 lines
  - **Requirements**: FR3, NFR1
  - **Leverage**: Task 6 error handling
  - **Prompt**: **Role**: Rust Refactoring Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor the 191-line duplication analysis function into focused components.

    **Current Function**: `duplication.rs:28-219` (191 lines)

    **Extraction Plan**:
    1. `DuplicationConfig::from_options()` - Build config (15 lines)
    2. `chunk_file_content()` - Create text chunks (20 lines)
    3. `compute_hashes()` - Hash chunks (15 lines)
    4. `find_duplicate_chunks()` - Identify duplicates (25 lines)
    5. `group_duplicates()` - Group by hash (20 lines)
    6. `filter_by_threshold()` - Apply min size filter (15 lines)
    7. `build_duplication_report()` - Create result (20 lines)
    8. Main `analyze_duplication()` - Orchestration (45 lines)

    **Algorithm Preservation**:
    - Ensure chunking strategy unchanged
    - Hash algorithm must remain identical
    - Threshold filtering logic preserved

  | **Restrictions**: Main function ≤50 lines, algorithm unchanged, performance ±5%, all tests pass
  | **Success**: ✅ analyze_duplication() ≤50 lines, ✅ Each helper single purpose, ✅ Algorithm unchanged, ✅ Tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (DuplicationConfig, chunking functions, hash computation, report builders), (4) Mark this task as complete [x] in tasks.md

- [x] 17. Refactor dead_code.rs:34 (CLI formatter)
  - **File**: `crates/code-viz-cli/src/output/dead_code.rs`
  - **Purpose**: Reduce formatting function from 175 → <50 lines
  - **Requirements**: FR3, NFR1
  - **Leverage**: Task 8 CLI error handling
  - **Prompt**: **Role**: Rust Refactoring Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor the 175-line dead code formatter into format-specific functions.

    **Current Function**: `dead_code.rs:34-209` (175 lines)

    **Extraction Plan**:
    1. Create `DeadCodeFormatter` trait:
       ```rust
       trait DeadCodeFormatter {
           fn format(&self, result: &DeadCodeResult) -> String;
       }
       ```
    2. Implement formatters:
       - `JsonFormatter` - JSON output (30 lines)
       - `TextFormatter` - Human-readable (35 lines)
       - `CsvFormatter` - CSV output (25 lines)
       - `QuietFormatter` - Minimal output (15 lines)
    3. Main function - Dispatch to formatter (20 lines)

    **Output Compatibility**:
    - JSON format must be identical
    - Text format whitespace may differ slightly (document changes)
    - CSV format unchanged

  | **Restrictions**: Main function ≤20 lines, each formatter ≤35 lines, output format compatible, extensible design
  | **Success**: ✅ Main function ≤20 lines, ✅ Each formatter focused, ✅ Output unchanged or documented, ✅ Easy to add new formats
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (DeadCodeFormatter trait, formatter implementations, dispatch logic), (4) Mark this task as complete [x] in tasks.md

- [x] 18. Refactor remaining 33 functions >50 lines
  - **Files**: Various files across codebase
  - **Purpose**: Ensure all functions ≤50 lines
  - **Requirements**: FR3, NFR1
  - **Leverage**: Patterns from tasks 13-17
  - **Prompt**: **Role**: Rust Refactoring Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor the remaining 33 functions exceeding 50 lines following established patterns.

    **Target Functions** (from audit):
    - `commands/analyze.rs:34` (135 lines) - Similar to task 13
    - `scanner.rs:7` (117 lines) - File scanning logic
    - `lib.rs:140` (dead-code) (161 lines) - Symbol analysis
    - Plus 30 more functions 50-100 lines

    **General Refactoring Strategy**:
    1. Identify single responsibility violations
    2. Extract parameter building to configs
    3. Extract loops to iterator helpers
    4. Extract error handling to dedicated functions
    5. Use orchestrator pattern for multi-step processes

    **Prioritization**:
    - Functions >100 lines first (8 remaining)
    - Functions 75-100 lines (12 functions)
    - Functions 50-75 lines (13 functions)

    **Quality Gates**:
    - All functions ≤50 lines
    - Cyclomatic complexity ≤10
    - Clear, descriptive names
    - Single responsibility

  | **Restrictions**: Every function ≤50 lines, no God functions, behavior preserved, all tests pass
  | **Success**: ✅ Zero functions >50 lines, ✅ Complexity metrics met, ✅ Clear naming, ✅ All tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (refactored functions by file, extracted helpers, orchestration patterns), (4) Mark this task as complete [x] in tasks.md

- [x] 19. Add tests for extracted helpers
  - **Files**: Test files for modules modified in tasks 13-18
  - **Purpose**: Ensure all extracted helper functions have unit tests
  - **Requirements**: FR3, NFR1
  - **Leverage**: Error scenario tests from task 11
  - **Prompt**: **Role**: QA Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create unit tests for all helper functions extracted during function size refactoring.

    **Test Coverage Requirements**:
    - Every extracted helper function has ≥1 test
    - Complex helpers (>20 lines) have ≥3 tests (happy path, edge case, error case)
    - Config builders tested with valid and invalid inputs
    - Orchestrators tested with mocks for dependencies

    **Test Organization**:
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        mod config_builder_tests { ... }
        mod analysis_orchestrator_tests { ... }
        mod helper_function_tests { ... }
    }
    ```

    **Testing Helpers to Create**:
    - Mock data builders for common test fixtures
    - Assertion helpers for complex structs
    - Property-based test generators

  | **Restrictions**: Every helper tested, clear test names, no test code duplication, use fixtures where appropriate
  | **Success**: ✅ All helpers have tests, ✅ Complex functions have edge case tests, ✅ Mocks used where needed, ✅ Coverage increased
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (test suites, mock helpers, assertion utilities), (4) Mark this task as complete [x] in tasks.md

- [x] 20. Validate complexity metrics
  - **Purpose**: Verify all functions meet cyclomatic complexity ≤10 and other quality metrics
  - **Requirements**: NFR1, NFR5
  - **Leverage**: cargo-clippy complexity lints
  - **Prompt**: **Role**: QA Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Validate that all refactored functions meet complexity and quality targets.

    **Metrics to Check**:
    1. **Cyclomatic Complexity**: Run `cargo clippy -- -W clippy::cognitive_complexity`
       - Target: ≤10 for all functions
       - Fail if any function >10
    2. **Function Size**: Automated check from task 5
       - Verify: All functions ≤50 lines
    3. **Import Counts**: Check module dependencies
       - Target: ≤8 imports per module
       - List violations
    4. **Nested Depth**: Check indentation levels
       - Target: ≤4 levels of nesting

    **Validation Script**:
    ```bash
    #!/bin/bash
    cargo clippy -- -W clippy::cognitive_complexity \
                     -W clippy::too_many_lines \
                     -W clippy::too_many_arguments

    # Run custom metrics
    ./scripts/check-metrics.sh

    # Generate complexity report
    cargo-complexity --all --json > complexity-report.json
    ```

    **Remediation**:
    - If any function fails metrics, refactor further
    - Document any necessary exceptions (with justification)

  | **Restrictions**: Must be automated, clear pass/fail, actionable failure messages, run in CI
  | **Success**: ✅ All functions complexity ≤10, ✅ All functions ≤50 lines, ✅ Metrics automated in CI, ✅ Report generated
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (validation scripts, complexity reports, CI integration), (4) Mark this task as complete [x] in tasks.md

---

## Phase 4: Module Extraction (Week 4)

### Fix file size and code duplication issues

- [x] 21. Create code-viz-core transform module
  - **Files**: `crates/code-viz-core/src/transform/` (new module)
  - **Purpose**: Extract shared transform logic to eliminate 210 duplicate lines
  - **Requirements**: FR5, NFR1
  - **Leverage**: Refactored transform.rs from task 15
  - **Prompt**: **Role**: Rust Systems Architect | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create shared transform module in code-viz-core to eliminate API/Tauri duplication.

    **Module Structure**:
    ```
    crates/code-viz-core/src/transform/
    ├── mod.rs (public API, 50 lines)
    ├── tree_builder.rs (node creation, 180 lines)
    ├── metric_aggregator.rs (metric calculations, 200 lines)
    └── path_utils.rs (path handling, 80 lines)
    ```

    **Public API** (mod.rs):
    ```rust
    pub struct TreeTransformConfig { ... }
    pub fn transform_flat_to_hierarchy(
        files: Vec<FileMetrics>,
        config: TreeTransformConfig,
    ) -> Result<TreeNode, TransformError> { ... }
    ```

    **Migration Strategy**:
    - Copy current API transform.rs logic to core module
    - Adapt to use core types (FileMetrics, TreeNode)
    - Add comprehensive rustdoc
    - Create extensive tests

    **Testing**:
    - Property-based tests for tree validity
    - Test metric aggregation correctness
    - Test path handling edge cases

  | **Restrictions**: Each file ≤200 lines, public API minimal, well-documented, comprehensive tests, no behavior changes
  | **Success**: ✅ Module compiles independently, ✅ All files <200 lines, ✅ Public API clear, ✅ Tests pass, ✅ Rustdoc complete
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (tree_builder functions, metric_aggregator logic, path_utils helpers, public API), (4) Mark this task as complete [x] in tasks.md

- [x] 22. Migrate API transform to use core
  - **File**: `crates/code-viz-api/src/transform.rs`
  - **Purpose**: Replace duplicate logic with calls to core module (578 → ~60 lines)
  - **Requirements**: FR5, NFR1
  - **Leverage**: Task 21 core transform module
  - **Prompt**: **Role**: Rust Developer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor API transform.rs to use shared core transform module.

    **New Implementation** (~60 lines):
    ```rust
    use code_viz_core::transform::{transform_flat_to_hierarchy, TreeTransformConfig};

    pub fn flat_to_hierarchy_api(
        files: Vec<ApiFileMetrics>,
    ) -> Result<ApiTreeNode, String> {
        // Convert API types to core types
        let core_files: Vec<FileMetrics> = files
            .into_iter()
            .map(Into::into)
            .collect();

        // Use core transform
        let config = TreeTransformConfig::default();
        let core_tree = transform_flat_to_hierarchy(core_files, config)
            .map_err(|e| e.to_string())?;

        // Convert core types back to API types
        Ok(core_tree.into())
    }
    ```

    **Type Conversions**:
    - Implement `From<ApiFileMetrics> for FileMetrics`
    - Implement `From<TreeNode> for ApiTreeNode`
    - Ensure zero-copy conversions where possible

    **Testing**:
    - Existing API tests must pass unchanged
    - Add conversion tests
    - Compare output with pre-refactor baseline (golden master)

  | **Restrictions**: File ≤60 lines, behavior identical, all API tests pass, efficient type conversions
  | **Success**: ✅ transform.rs ≤60 lines, ✅ Uses core module, ✅ Output identical to baseline, ✅ Tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (type conversions, adapter functions, integration updates), (4) Mark this task as complete [x] in tasks.md

- [x] 23. Migrate Tauri transform to use core
  - **File**: `crates/code-viz-tauri/src/transform.rs`
  - **Purpose**: Replace duplicate logic with calls to core module (578 → ~60 lines)
  - **Requirements**: FR5, NFR1
  - **Leverage**: Task 21 core module, Task 22 API migration pattern
  - **Prompt**: **Role**: Rust Developer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor Tauri transform.rs to use shared core transform module following the pattern from task 22.

    **New Implementation** (~60 lines):
    ```rust
    use code_viz_core::transform::{transform_flat_to_hierarchy, TreeTransformConfig};

    pub fn flat_to_hierarchy_tauri(
        files: Vec<TauriFileMetrics>,
    ) -> Result<TauriTreeNode, String> {
        let core_files = files.into_iter().map(Into::into).collect();
        let config = TreeTransformConfig::default();
        let core_tree = transform_flat_to_hierarchy(core_files, config)
            .map_err(|e| e.to_string())?;
        Ok(core_tree.into())
    }
    ```

    **Type Conversions**:
    - Implement `From<TauriFileMetrics> for FileMetrics`
    - Implement `From<TreeNode> for TauriTreeNode`
    - Ensure TypeScript bindings still work (specta derives)

    **Testing**:
    - Existing Tauri tests pass
    - Frontend integration unchanged
    - TypeScript types match

  | **Restrictions**: File ≤60 lines, Tauri commands work, frontend unchanged, specta bindings valid
  | **Success**: ✅ transform.rs ≤60 lines, ✅ Uses core module, ✅ Frontend works, ✅ TypeScript bindings correct
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (type conversions, Tauri command updates, specta integration), (4) Mark this task as complete [x] in tasks.md

- [x] 24. Validate duplication eliminated
  - **Purpose**: Verify 210 duplicate lines removed and both API/Tauri use shared code
  - **Requirements**: FR5
  - **Leverage**: Tasks 21-23 completed modules
  - **Prompt**: **Role**: QA Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Validate that transform code duplication has been eliminated.

    **Validation Steps**:
    1. Run `cargo-dup` or similar tool to detect duplication
    2. Manually compare API and Tauri transform.rs
       - Should be <60 lines each
       - Should only contain type conversions and core calls
       - No business logic duplication
    3. Run both API and Tauri with same input, verify identical output
    4. Check core module is properly shared (only one implementation)

    **Metrics**:
    - Before: 578 lines × 2 files = 1156 lines total
    - After: 510 lines (core) + 60 (API) + 60 (Tauri) = 630 lines total
    - **Savings**: 526 lines eliminated (45% reduction)

    **Integration Testing**:
    - Run full analysis through both API and Tauri
    - Compare tree structures (must be identical)
    - Verify metrics aggregation matches

  | **Restrictions**: Must be automated, clear metrics, regression tests in place
  | **Success**: ✅ No duplication detected, ✅ Both use core module, ✅ Output identical, ✅ 45% code reduction achieved
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (validation scripts, comparison tests, metrics reports), (4) Mark this task as complete [x] in tasks.md

- [x] 25. Split large test files
  - **Files**: `Treemap.test.tsx`, `treeTransform.test.ts`
  - **Purpose**: Reduce test file sizes from 867, 503 → <300 lines each
  - **Requirements**: FR4, NFR1
  - **Leverage**: Test organization patterns from task 19
  - **Prompt**: **Role**: Test Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Split large test files into focused test suites.

    **Treemap.test.tsx** (867 → ~250 lines × 4 files):
    ```
    src/components/visualizations/__tests__/
    ├── Treemap.interaction.test.tsx (250 lines)
    │   - Click handling
    │   - Hover interactions
    │   - Drill-down behavior
    ├── Treemap.rendering.test.tsx (280 lines)
    │   - Visual rendering
    │   - Color schemes
    │   - Layout calculations
    ├── Treemap.data.test.tsx (200 lines)
    │   - Data transformation
    │   - Metric calculations
    │   - Tree structure
    └── Treemap.accessibility.test.tsx (150 lines)
        - ARIA attributes
        - Keyboard navigation
        - Screen reader support
    ```

    **treeTransform.test.ts** (503 → ~250 lines × 2 files):
    ```
    src/utils/__tests__/
    ├── treeTransform.structure.test.ts (270 lines)
    │   - Tree building logic
    │   - Path handling
    │   - Node creation
    └── treeTransform.metrics.test.ts (250 lines)
        - Metric aggregation
        - Rollup calculations
        - Edge cases
    ```

    **Shared Test Utilities**:
    - Create `__fixtures__/` directory for test data
    - Extract common setup to `test-utils.ts`
    - Reuse mock data builders

  | **Restrictions**: All test files <300 lines, no test duplication, shared fixtures, all tests pass
  | **Success**: ✅ All test files <300 lines, ✅ Logical organization, ✅ Shared utilities extracted, ✅ All tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (test suite splits, fixture files, test utilities), (4) Mark this task as complete [x] in tasks.md

---

## Phase 5: Test Coverage (Weeks 5-6)

### Achieve 80%+ coverage for all code

- [x] 26. Add TypeScript component tests (12 files)
  - **Files**: Test files for 12 untested components
  - **Purpose**: Create comprehensive tests for all untested UI components
  - **Requirements**: FR6, NFR1
  - **Leverage**: Testing patterns from task 25
  - **Prompt**: **Role**: Frontend Test Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create comprehensive test files for all 12 untested TypeScript components.

    **Components to Test**:
    1. TreeView.tsx → TreeView.test.tsx
    2. ErrorBoundary.tsx → ErrorBoundary.test.tsx
    3. DataDebugger.tsx → DataDebugger.test.tsx
    4. AnalysisSettings.tsx → AnalysisSettings.test.tsx
    5. LoadingSkeleton.tsx → LoadingSkeleton.test.tsx
    6. ProgressBar.tsx → ProgressBar.test.tsx
    7. Sunburst.tsx → Sunburst.test.tsx
    8. CirclePacking.tsx → CirclePacking.test.tsx
    9-12. Remaining 4 untested components

    **Test Structure** (per component):
    ```tsx
    import { render, screen, fireEvent } from '@testing-library/react';
    import { ComponentName } from '../ComponentName';

    describe('ComponentName', () => {
      describe('rendering', () => {
        test('renders with required props', () => { ... });
        test('renders with optional props', () => { ... });
      });

      describe('interactions', () => {
        test('handles user clicks', () => { ... });
        test('updates on prop changes', () => { ... });
      });

      describe('edge cases', () => {
        test('handles empty data', () => { ... });
        test('handles errors gracefully', () => { ... });
      });
    });
    ```

    **Coverage Targets**:
    - Each component ≥80% line coverage
    - All props tested (with and without)
    - All event handlers tested
    - Error states tested

  | **Restrictions**: Use @testing-library/react, no snapshot tests, coverage ≥80% per file, async code properly awaited
  | **Success**: ✅ All 12 components have tests, ✅ Each file ≥80% coverage, ✅ User interactions tested, ✅ All tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (test files by component, test utilities, mock data builders), (4) Mark this task as complete [x] in tasks.md

- [x] 27. Add Rust critical module tests
  - **Files**: Test files for coupling, parser, hotspot, metrics modules
  - **Purpose**: Test core analysis logic currently at 0% coverage
  - **Requirements**: FR7, NFR1
  - **Leverage**: Error scenario tests from task 11
  - **Prompt**: **Role**: Rust Test Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create comprehensive test coverage for critical Rust analysis modules.

    **Modules to Test**:
    1. **coupling.rs** (0% → 90%)
       - Dependency graph building
       - Import detection across languages
       - Coupling metrics calculation
    2. **parser.rs** (0% → 90%)
       - Tree-sitter integration
       - Multi-language parsing
       - Error recovery
    3. **hotspot.rs** (0% → 80%)
       - Hotspot detection algorithm
       - Commit history analysis
       - Scoring logic
    4. **metrics.rs** (~20% → 90%)
       - LOC calculations
       - Complexity metrics
       - Function counting

    **Test Files to Create**:
    ```
    crates/code-viz-core/tests/
    ├── coupling_tests.rs
    ├── parser_tests.rs
    ├── hotspot_tests.rs
    ├── metrics_tests.rs
    └── fixtures/
        ├── sample_rust.rs
        ├── sample_typescript.ts
        ├── sample_python.py
        └── malformed_samples/
    ```

    **Testing Approach**:
    - Unit tests for each function
    - Integration tests with real fixtures
    - Property-based tests for parsing
    - Error scenario tests

  | **Restrictions**: Coverage ≥80% for each module (≥90% for coupling/parser), use real fixtures, test error paths
  | **Success**: ✅ coupling.rs ≥90%, ✅ parser.rs ≥90%, ✅ hotspot.rs ≥80%, ✅ metrics.rs ≥90%, ✅ All tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (test modules, fixture files, property-based tests), (4) Mark this task as complete [x] in tasks.md

- [x] 28. Add integration tests for analysis pipeline
  - **Files**: `crates/code-viz-core/tests/integration_tests.rs`
  - **Purpose**: Test full end-to-end analysis workflows
  - **Requirements**: FR7, NFR2
  - **Prompt**: **Role**: Integration Test Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create end-to-end integration tests for the complete analysis pipeline.

    **Test Scenarios**:
    1. **Full Analysis**: All features enabled
    2. **Partial Analysis**: Selected features only
    3. **Error Recovery**: Malformed files in repository
    4. **Large Repository**: Performance under load
    5. **Empty Repository**: Graceful handling
    6. **Multi-Language**: Mixed language codebases

    **Test Structure**:
    ```rust
    #[test]
    fn test_full_analysis_pipeline() {
        let repo = create_test_repository();
        let config = AnalysisConfig::all_features();

        let result = analyze_repository(&repo, config).unwrap();

        assert!(result.files.len() > 0);
        assert!(result.duplication_data.is_some());
        assert!(result.hotspots.is_some());
        assert!(result.coverage.is_some());
    }
    ```

    **Test Fixtures**:
    - Create realistic test repositories
    - Include edge cases (empty files, binary files, etc.)
    - Multi-language samples

  | **Restrictions**: Tests must be deterministic, use isolated test dirs, clean up after tests, performance benchmarks included
  | **Success**: ✅ End-to-end tests pass, ✅ Error recovery tested, ✅ Performance acceptable, ✅ Multi-language tested
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (integration test cases, test repository fixtures, performance benchmarks), (4) Mark this task as complete [x] in tasks.md

- [x] 29. Add negative/error scenario tests
  - **Files**: Test modules across all crates
  - **Purpose**: Ensure error handling works correctly in all failure modes
  - **Requirements**: FR1, FR7, NFR1
  - **Leverage**: Error types from tasks 6-10
  - **Prompt**: **Role**: QA Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create comprehensive negative test scenarios validating error handling.

    **Error Scenarios to Test**:
    1. **File System Errors**
       - Missing files
       - Permission denied
       - Disk full
       - Invalid paths
    2. **Parser Errors**
       - Malformed syntax
       - Unsupported language
       - Binary files
       - Empty files
    3. **Configuration Errors**
       - Invalid config values
       - Missing required fields
       - Conflicting options
    4. **Analysis Errors**
       - Circular dependencies
       - Missing coverage data
       - Timeout scenarios
    5. **Resource Errors**
       - Out of memory
       - Too many open files

    **Test Pattern**:
    ```rust
    #[test]
    fn test_parser_error_on_malformed_rust() {
        let malformed = "fn incomplete(";
        let result = parse_rust_code(malformed);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_matches!(err, CodeVizError::ParseError { .. });
        assert!(err.to_string().contains("malformed"));
    }
    ```

  | **Restrictions**: All error paths tested, use assert_matches!, error messages validated, no test code duplicating production
  | **Success**: ✅ All error variants tested, ✅ Error messages checked, ✅ Graceful degradation validated, ✅ Tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (error scenario tests, assertion helpers, failure mode coverage), (4) Mark this task as complete [x] in tasks.md

- [x] 30. Add edge case tests
  - **Files**: Test modules for edge case scenarios
  - **Purpose**: Test boundary conditions and unusual inputs
  - **Requirements**: FR7, NFR1
  - **Prompt**: **Role**: QA Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create edge case tests for boundary conditions and unusual scenarios.

    **Edge Cases to Test**:
    1. **Empty Inputs**
       - Empty repository
       - Empty files
       - No coverage data
    2. **Extreme Values**
       - Very large files (100K+ lines)
       - Very deep directory nesting
       - Thousands of small files
    3. **Special Characters**
       - Unicode in filenames
       - Spaces in paths
       - Special characters in code
    4. **Circular Dependencies**
       - Module A imports B imports A
       - Transitive circular deps
    5. **Platform-Specific**
       - Windows paths (\\) on Linux
       - Symlinks
       - Case sensitivity

    **Property-Based Tests**:
    - Use `proptest` for random input generation
    - Invariant: Parser never panics on any input
    - Invariant: Tree structure always valid

  | **Restrictions**: Use proptest for random testing, no panic!() on any input, tests isolated and deterministic
  | **Success**: ✅ All edge cases tested, ✅ Property-based tests added, ✅ No panics on extreme inputs, ✅ Tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (edge case tests, property-based tests, invariant checks), (4) Mark this task as complete [x] in tasks.md

- [x] 31. Run coverage reports
  - **Purpose**: Generate coverage reports to identify gaps
  - **Requirements**: NFR1
  - **Leverage**: Tests from tasks 26-30
  - **Prompt**: **Role**: QA Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Generate comprehensive coverage reports for Rust and TypeScript code.

    **Rust Coverage** (cargo-tarpaulin):
    ```bash
    cargo tarpaulin --out Html --out Xml \
      --exclude-files 'tests/*' \
      --exclude-files '*_test.rs' \
      --all-features \
      --workspace
    ```

    **TypeScript Coverage** (vitest):
    ```bash
    npm run test:coverage -- \
      --coverage.reporter=html \
      --coverage.reporter=json \
      --coverage.all
    ```

    **Report Generation**:
    - HTML reports for manual review
    - JSON/XML for automated processing
    - Per-file coverage breakdown
    - Identify files <80% coverage

    **Coverage Dashboard**:
    - Overall percentage
    - Per-crate breakdown (Rust)
    - Per-component breakdown (TypeScript)
    - Trend over time (if historical data available)

  | **Restrictions**: Exclude test files from coverage, generate machine-readable reports, identify gaps programmatically
  | **Success**: ✅ Coverage reports generated, ✅ Per-file breakdown available, ✅ Gaps identified, ✅ Reports uploaded to CI
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (coverage scripts, report configurations, gap analysis), (4) Mark this task as complete [x] in tasks.md

- [x] 32. Fix coverage gaps below 80%
  - **Purpose**: Add tests to bring all modules above 80% threshold
  - **Requirements**: NFR1
  - **Leverage**: Coverage reports from task 31
  - **Prompt**: **Role**: Test Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Add tests to address all coverage gaps identified in task 31 reports.

    **Process**:
    1. Review coverage report from task 31
    2. Identify all files/modules <80% coverage
    3. For each gap:
       - Analyze uncovered lines
       - Determine if code is dead (can be removed)
       - Write tests to cover live code
    4. Re-run coverage to verify

    **Priority**:
    - Critical paths first (core analysis logic)
    - User-facing components second
    - Utilities and helpers third

    **Iterative Approach**:
    - Fix highest-impact gaps first
    - Run coverage after each batch
    - Stop when all targets met

  | **Restrictions**: All modules ≥80% (≥90% for critical), dead code removed not just tested, tests must be meaningful
  | **Success**: ✅ All modules ≥80% coverage, ✅ Critical paths ≥90%, ✅ Dead code removed, ✅ Coverage gates pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (added test cases, dead code removals, coverage improvements), (4) Mark this task as complete [x] in tasks.md

- [x] 33. Add property-based tests for parsers
  - **Files**: `crates/code-viz-core/tests/parser_property_tests.rs`
  - **Purpose**: Validate parser robustness with random inputs
  - **Requirements**: FR7, NFR1
  - **Leverage**: proptest library
  - **Prompt**: **Role**: Test Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create property-based tests to validate parser correctness and robustness.

    **Properties to Test**:
    1. **No Panics**: Parser never panics on any input
    2. **Idempotency**: Parsing twice produces same result
    3. **Valid Output**: Parsed tree always valid structure
    4. **Error Handling**: Malformed input returns Err, never panics

    **Proptest Implementation**:
    ```rust
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn parser_never_panics(code in "\\PC*") {
            let result = parse_rust_code(&code);
            // Should return Ok or Err, never panic
            assert!(result.is_ok() || result.is_err());
        }

        #[test]
        fn tree_structure_valid(code in valid_rust_code()) {
            if let Ok(tree) = parse_rust_code(&code) {
                assert_valid_tree_structure(&tree);
            }
        }
    }
    ```

    **Input Generators**:
    - Random UTF-8 strings
    - Valid syntax with random content
    - Intentionally malformed syntax

  | **Restrictions**: Use proptest crate, run 1000+ iterations, no panics allowed, tests must be deterministic with seed
  | **Success**: ✅ No panics on random input, ✅ 1000+ test cases pass, ✅ Properties validated, ✅ Fast execution
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (property-based tests, input generators, invariant checks), (4) Mark this task as complete [x] in tasks.md

- [x] 34. Add performance regression tests
  - **Files**: `benches/analysis_benchmarks.rs`
  - **Purpose**: Ensure refactoring doesn't degrade performance
  - **Requirements**: NFR2
  - **Leverage**: criterion crate for benchmarking
  - **Prompt**: **Role**: Performance Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Create performance benchmarks to detect regressions.

    **Benchmarks to Create**:
    1. **Full Analysis**: Time to analyze medium repository (~1K files)
    2. **Parser Performance**: Parse 1000 files
    3. **Duplication Detection**: Large codebase duplication scan
    4. **Hotspot Analysis**: Git history processing
    5. **Coverage Analysis**: Coverage data processing

    **Criterion Setup**:
    ```rust
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn bench_full_analysis(c: &mut Criterion) {
        let repo = load_benchmark_repository();
        c.bench_function("full_analysis", |b| {
            b.iter(|| analyze_repository(black_box(&repo)))
        });
    }

    criterion_group!(benches, bench_full_analysis);
    criterion_main!(benches);
    ```

    **Baseline**:
    - Run benchmarks before refactoring
    - Save baseline results
    - Compare after refactoring
    - Fail if >5% slower

  | **Restrictions**: Use criterion, deterministic benchmarks, baseline comparison, CI integration, ±5% acceptable variance
  | **Success**: ✅ Benchmarks created, ✅ Baseline established, ✅ No >5% regression, ✅ Runs in CI
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (benchmark suites, baseline measurements, CI integration), (4) Mark this task as complete [x] in tasks.md

- [x] 35. Add CI coverage gates
  - **Files**: `.github/workflows/coverage.yml`
  - **Purpose**: Block PRs that reduce coverage below 80%
  - **Requirements**: NFR1
  - **Leverage**: Coverage scripts from task 31
  - **Prompt**: **Role**: DevOps Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Setup CI to enforce coverage thresholds.

    **CI Workflow**:
    ```yaml
    name: Coverage

    on: [pull_request]

    jobs:
      rust-coverage:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4
          - name: Install tarpaulin
            run: cargo install cargo-tarpaulin
          - name: Run coverage
            run: cargo tarpaulin --fail-under 80 --out Xml
          - name: Upload to codecov
            uses: codecov/codecov-action@v4

      typescript-coverage:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4
          - name: Install deps
            run: npm ci
          - name: Run tests with coverage
            run: npm run test:coverage -- --coverage --coverageThreshold='{"global":{"lines":80}}'
    ```

    **Failure Handling**:
    - Clear message showing which files are below threshold
    - Link to coverage report
    - Block merge until fixed

  | **Restrictions**: Must run on every PR, fail if coverage <80%, upload reports, fast execution
  | **Success**: ✅ Coverage runs in CI, ✅ Fails below 80%, ✅ Reports uploaded, ✅ Clear failure messages
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (CI workflows, coverage configurations, gate implementations), (4) Mark this task as complete [x] in tasks.md

---

## Phase 6: Architecture Improvements (Week 7)

### Fix SOLID violations and improve extensibility

- [x] 36. Split metrics.rs into focused modules
  - **File**: `crates/code-viz-core/src/metrics.rs` → multiple modules
  - **Purpose**: Fix God object violating Single Responsibility Principle
  - **Requirements**: FR8, NFR5
  - **Leverage**: Module extraction patterns from Phase 4
  - **Prompt**: **Role**: Rust Architect | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Split metrics.rs into three focused modules.

    **Module Structure**:
    ```
    crates/code-viz-core/src/metrics/
    ├── mod.rs (public API, 40 lines)
    ├── loc_calculator.rs (LOC counting logic, 120 lines)
    ├── function_counter.rs (function detection, 100 lines)
    └── complexity_analyzer.rs (complexity metrics, 150 lines)
    ```

    **Responsibility Separation**:
    - `loc_calculator`: Lines of code counting (exclude comments, blanks)
    - `function_counter`: Function/method detection across languages
    - `complexity_analyzer`: Cyclomatic complexity, nesting depth

    **Public API** (mod.rs):
    ```rust
    pub use loc_calculator::calculate_loc;
    pub use function_counter::count_functions;
    pub use complexity_analyzer::analyze_complexity;

    pub struct FileMetrics {
        pub loc: LocMetrics,
        pub functions: FunctionMetrics,
        pub complexity: ComplexityMetrics,
    }

    pub fn analyze_file_metrics(file: &File) -> Result<FileMetrics> {
        Ok(FileMetrics {
            loc: calculate_loc(file)?,
            functions: count_functions(file)?,
            complexity: analyze_complexity(file)?,
        })
    }
    ```

  | **Restrictions**: Each module ≤150 lines, single responsibility, well-documented, comprehensive tests, behavior unchanged
  | **Success**: ✅ 3 focused modules created, ✅ Each ≤150 lines, ✅ Clear responsibilities, ✅ Tests pass, ✅ Public API clean
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (loc_calculator functions, function_counter logic, complexity_analyzer methods, public API), (4) Mark this task as complete [x] in tasks.md

- [x] 37. Split dead_code lib.rs into orchestrator pattern
  - **File**: `crates/code-viz-dead-code/src/lib.rs:140` → multiple modules
  - **Purpose**: Fix 161-line function God object
  - **Requirements**: FR8, NFR5
  - **Leverage**: Orchestrator pattern from task 13
  - **Prompt**: **Role**: Rust Architect | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor lib.rs:140 into orchestrator pattern with focused modules.

    **Module Structure**:
    ```
    crates/code-viz-dead-code/src/
    ├── lib.rs (40 lines - public API)
    ├── analyzer.rs (reachability analysis, 100 lines)
    ├── graph_manager.rs (graph loading/building, 90 lines)
    ├── result_aggregator.rs (result collection, 80 lines)
    └── orchestrator.rs (coordination, 60 lines)
    ```

    **Orchestrator Pattern**:
    ```rust
    // orchestrator.rs
    pub struct DeadCodeOrchestrator {
        graph_manager: GraphManager,
        analyzer: ReachabilityAnalyzer,
        aggregator: ResultAggregator,
    }

    impl DeadCodeOrchestrator {
        pub fn analyze(&mut self, entry_points: &[Symbol]) -> Result<DeadCodeResult> {
            let graph = self.graph_manager.load_or_build()?;
            let reachable = self.analyzer.find_reachable(&graph, entry_points)?;
            let result = self.aggregator.aggregate(&graph, &reachable)?;
            Ok(result)
        }
    }
    ```

  | **Restrictions**: Each module single responsibility, orchestrator ≤60 lines, behavior unchanged, tests pass
  | **Success**: ✅ 4 focused modules created, ✅ Orchestrator pattern implemented, ✅ Tests pass, ✅ Clear separation
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (analyzer module, graph_manager class, result_aggregator functions, orchestrator implementation), (4) Mark this task as complete [x] in tasks.md

- [x] 38. Create language plugin system
  - **Files**: `crates/code-viz-core/src/language/` (new module)
  - **Purpose**: Abstract hardcoded language support for extensibility
  - **Requirements**: FR9, NFR5
  - **Leverage**: Existing language-specific queries in coupling.rs
  - **Prompt**: **Role**: Rust Systems Architect | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Design and implement a language plugin system for extensible language support.

    **Module Structure**:
    ```
    crates/code-viz-core/src/language/
    ├── mod.rs (public API, 60 lines)
    ├── provider.rs (LanguageProvider trait, 40 lines)
    ├── registry.rs (LanguageRegistry, 80 lines)
    ├── plugins/
    │   ├── rust.rs (Rust plugin, 100 lines)
    │   ├── typescript.rs (TypeScript plugin, 100 lines)
    │   └── python.rs (Python plugin, 100 lines)
    └── queries/ (move hardcoded queries here)
        ├── rust.scm
        ├── typescript.scm
        └── python.scm
    ```

    **LanguageProvider Trait**:
    ```rust
    pub trait LanguageProvider: Send + Sync {
        fn name(&self) -> &str;
        fn file_extensions(&self) -> &[&str];
        fn tree_sitter_language(&self) -> tree_sitter::Language;
        fn coupling_query(&self) -> &str;
        fn symbol_query(&self) -> &str;
    }
    ```

    **LanguageRegistry**:
    ```rust
    pub struct LanguageRegistry {
        providers: HashMap<String, Box<dyn LanguageProvider>>,
    }

    impl LanguageRegistry {
        pub fn new() -> Self {
            let mut registry = Self::default();
            registry.register(Box::new(RustLanguageProvider));
            registry.register(Box::new(TypeScriptLanguageProvider));
            registry.register(Box::new(PythonLanguageProvider));
            registry
        }

        pub fn get_by_extension(&self, ext: &str) -> Option<&dyn LanguageProvider>;
    }
    ```

    **Migration**:
    - Move hardcoded queries from coupling.rs to query files
    - Replace language detection switch with registry lookup
    - Update all language-specific code to use providers

  | **Restrictions**: Trait-based design, queries in separate files, registry for runtime lookup, zero core changes to add language
  | **Success**: ✅ LanguageProvider trait defined, ✅ Registry implemented, ✅ 3 languages migrated, ✅ Easy to add languages
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (LanguageProvider trait, LanguageRegistry class, language plugin implementations, query files), (4) Mark this task as complete [x] in tasks.md

- [ ] 39. Migrate coupling.rs to use language registry
  - **File**: `crates/code-viz-core/src/coupling.rs`
  - **Purpose**: Remove hardcoded language queries using plugin system from task 38
  - **Requirements**: FR9, NFR5
  - **Leverage**: Language registry from task 38
  - **Prompt**: **Role**: Rust Developer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Refactor coupling.rs to use the language registry instead of hardcoded queries.

    **Before** (lines 8-27 - hardcoded):
    ```rust
    const TYPESCRIPT_QUERY: &str = "...";
    const RUST_QUERY: &str = "...";
    const PYTHON_QUERY: &str = "...";

    let query_str = match language {
        Language::TypeScript => TYPESCRIPT_QUERY,
        Language::Rust => RUST_QUERY,
        Language::Python => PYTHON_QUERY,
    };
    ```

    **After** (using registry):
    ```rust
    use crate::language::LanguageRegistry;

    let registry = LanguageRegistry::new();
    let provider = registry
        .get_by_extension(&file_extension)
        .ok_or(CodeVizError::UnsupportedLanguage)?;

    let query_str = provider.coupling_query();
    let language = provider.tree_sitter_language();
    ```

    **Benefits**:
    - Adding new language = new plugin file, zero coupling.rs changes
    - Queries maintainable separately
    - Language detection centralized

  | **Restrictions**: Behavior unchanged, no hardcoded language logic, extensible design, all tests pass
  | **Success**: ✅ No hardcoded queries in coupling.rs, ✅ Uses language registry, ✅ Tests pass, ✅ Same output
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (registry integration, language detection refactor, query loading), (4) Mark this task as complete [x] in tasks.md

- [ ] 40. Reduce TypeScript any usage
  - **Files**: TypeScript files across frontend
  - **Purpose**: Improve type safety by replacing 82 any usages with proper types
  - **Requirements**: FR10, NFR1
  - **Leverage**: TypeScript bindings from specta
  - **Prompt**: **Role**: TypeScript Developer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Replace any types with proper TypeScript interfaces.

    **Priority Files**:
    1. **Treemap.tsx:95** - ECharts params
    2. **Sunburst.tsx** - Chart event handlers
    3. **CirclePacking.tsx** - Visualization props
    4. Component props across codebase

    **Type Definitions to Create**:
    ```typescript
    // types/echarts.ts
    interface EChartsClickParams {
      componentType: string;
      seriesType: string;
      seriesIndex: number;
      dataIndex: number;
      data: Record<string, unknown>;
      treePathInfo?: Array<{
        name: string;
        dataIndex: number;
        value: number;
      }>;
    }

    interface EChartsHoverParams {
      componentType: string;
      name: string;
      value: number;
    }
    ```

    **Conversion Strategy**:
    - Create interface definitions based on actual usage
    - Use Record<string, unknown> for truly dynamic objects
    - Leverage bindings.ts types from Rust (via specta)
    - Enable strict mode incrementally

    **Target**:
    - Zero any in component props
    - <5 any in utility code (only where truly necessary)
    - Document why any is necessary when used

  | **Restrictions**: Zero any in component code, <5 any total, strict mode enabled, all types from Rust use bindings.ts
  | **Success**: ✅ Component props type-safe, ✅ <5 any usages, ✅ Strict mode enabled, ✅ Tests pass
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (type interface definitions, component prop types, strict mode configuration), (4) Mark this task as complete [x] in tasks.md

---

## Phase 7: Validation (Week 8)

### Verify all quality gates and metrics

- [ ] 41. Run full test suite
  - **Purpose**: Verify 100% test pass rate across all tests
  - **Requirements**: NFR2
  - **Leverage**: All tests from previous phases
  - **Prompt**: **Role**: QA Lead | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Execute complete test suite and validate 100% pass rate.

    **Test Execution**:
    ```bash
    # Rust tests
    cargo test --workspace --all-features

    # TypeScript tests
    npm run test

    # Integration tests
    cargo test --test '*'

    # E2E tests (if any)
    npm run test:e2e
    ```

    **Validation**:
    - All unit tests pass
    - All integration tests pass
    - All property-based tests pass
    - No flaky tests (run 3 times)

    **Failure Handling**:
    - Document any failures
    - Root cause analysis
    - Fix or create follow-up task

  | **Restrictions**: Must pass 100%, no skipped tests, no ignored failures, deterministic results
  | **Success**: ✅ 100% pass rate, ✅ No flaky tests, ✅ All test types passing, ✅ Fast execution
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (test execution logs, failure analyses if any, performance metrics), (4) Mark this task as complete [x] in tasks.md

- [ ] 42. Run clippy in strict mode
  - **Purpose**: Verify zero Clippy warnings in strict mode
  - **Requirements**: NFR1, NFR5
  - **Leverage**: Clippy configuration
  - **Prompt**: **Role**: Rust Code Quality Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Run Clippy in strict mode and ensure zero warnings.

    **Clippy Execution**:
    ```bash
    cargo clippy --workspace --all-features --all-targets -- \
      -D warnings \
      -W clippy::cognitive_complexity \
      -W clippy::too_many_lines \
      -W clippy::too_many_arguments \
      -W clippy::unwrap_used \
      -W clippy::expect_used
    ```

    **Expected Result**: Zero warnings

    **If Warnings Found**:
    - Fix immediately (refactor code)
    - If impossible to fix: #[allow(clippy::...)] with justification comment
    - Document exceptions in validation report

  | **Restrictions**: Zero warnings in production code, test warnings allowed with #[cfg(test)], justify all #[allow]
  | **Success**: ✅ Zero Clippy warnings, ✅ Strict lints enabled, ✅ Justified exceptions only, ✅ Runs in CI
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (clippy configurations, warning fixes, justified exceptions), (4) Mark this task as complete [x] in tasks.md

- [ ] 43. Run ESLint strict mode
  - **Purpose**: Verify zero ESLint errors in strict configuration
  - **Requirements**: NFR1, NFR5
  - **Leverage**: ESLint configuration
  - **Prompt**: **Role**: Frontend Code Quality Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Run ESLint in strict mode and ensure zero errors.

    **ESLint Execution**:
    ```bash
    npm run lint -- --max-warnings=0
    ```

    **Strict Rules to Enable**:
    ```json
    {
      "rules": {
        "@typescript-eslint/no-explicit-any": "error",
        "@typescript-eslint/no-unused-vars": "error",
        "react-hooks/rules-of-hooks": "error",
        "react-hooks/exhaustive-deps": "error"
      }
    }
    ```

    **Expected Result**: Zero errors, zero warnings

  | **Restrictions**: Zero errors, zero warnings, strict TypeScript rules, no disabled rules without justification
  | **Success**: ✅ Zero ESLint errors, ✅ Strict config enabled, ✅ No any violations, ✅ Runs in CI
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (ESLint configurations, error fixes, rule justifications), (4) Mark this task as complete [x] in tasks.md

- [ ] 44. Verify coverage reports meet targets
  - **Purpose**: Confirm ≥80% overall coverage, ≥90% for critical paths
  - **Requirements**: NFR1
  - **Leverage**: Coverage reports from task 31
  - **Prompt**: **Role**: QA Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Validate coverage reports meet all targets.

    **Verification**:
    ```bash
    # Rust
    cargo tarpaulin --fail-under 80

    # TypeScript
    npm run test:coverage -- --coverage --coverageThreshold='{"global":{"lines":80}}'
    ```

    **Module-Specific Checks**:
    - coupling.rs ≥90%
    - parser.rs ≥90%
    - metrics modules ≥90%
    - All components ≥80%

    **Report Generation**:
    - Create validation summary
    - List all modules with coverage %
    - Highlight any gaps

  | **Restrictions**: Must meet all thresholds, automated validation, clear reporting
  | **Success**: ✅ Overall ≥80%, ✅ Critical paths ≥90%, ✅ All modules pass, ✅ Report generated
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (coverage validation scripts, module-level reports, threshold checks), (4) Mark this task as complete [x] in tasks.md

- [ ] 45. Run performance benchmarks
  - **Purpose**: Verify no performance regression from refactoring
  - **Requirements**: NFR2
  - **Leverage**: Benchmarks from task 34
  - **Prompt**: **Role**: Performance Engineer | **Task**: Implement the task for spec error-handling-code-quality-remediation, first run spec-workflow-guide to get the workflow guide then implement the task: Execute performance benchmarks and compare against baseline.

    **Benchmark Execution**:
    ```bash
    cargo bench --bench analysis_benchmarks
    ```

    **Comparison**:
    - Load baseline from before refactoring
    - Compare current results
    - Calculate % difference
    - Flag any >5% regression

    **Acceptance Criteria**:
    - Full analysis: ±5%
    - Parser: ±5%
    - Duplication: ±5%
    - All benchmarks: no regression >5%

    **Report**:
    - Performance comparison table
    - Identify any regressions
    - Investigate causes if found
    - Optimize if necessary

  | **Restrictions**: No >5% regression, deterministic benchmarks, comparison to baseline, actionable results
  | **Success**: ✅ All benchmarks ±5%, ✅ No significant regression, ✅ Report generated, ✅ Baseline documented
  | **After completing this task**: (1) Mark this task as in-progress [-] in tasks.md before starting, (2) Implement the changes, (3) Use log-implementation tool to record detailed artifacts (benchmark results, baseline comparisons, performance analysis), (4) Mark this task as complete [x] in tasks.md

---

## Summary

**Total Tasks**: 45
**Estimated Duration**: 8 weeks (9 weeks with validation buffer)

**Task Breakdown by Phase**:
- Phase 1 (Foundation): 5 tasks
- Phase 2 (Error Elimination): 7 tasks
- Phase 3 (Function Size): 8 tasks
- Phase 4 (Module Extraction): 5 tasks
- Phase 5 (Test Coverage): 10 tasks
- Phase 6 (Architecture): 5 tasks
- Phase 7 (Validation): 5 tasks

**Success Metrics**:
- ✅ Zero unwrap() in production code
- ✅ All functions ≤50 lines
- ✅ All files ≤500 lines
- ✅ Test coverage ≥80% (≥90% critical)
- ✅ Code duplication eliminated
- ✅ Extensible language plugin system
- ✅ Type-safe TypeScript (<5 any)
- ✅ All quality gates pass

**Implementation Order**: Must follow phase sequence (foundation before refactoring)
