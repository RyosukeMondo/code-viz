# code-viz-commands

Framework-agnostic orchestration layer for code-viz analysis commands.

## Purpose

This crate provides pure orchestration functions that coordinate code analysis workflows using trait-based dependency injection. It sits between presentation layers (Tauri GUI, CLI) and core business logic, enabling 100% testability without I/O.

## Architecture

```
Presentation Layer (Tauri/CLI)
         ↓
  Commands Layer (this crate)  ← Zero framework dependencies
         ↓
    Core Layer (algorithms)
```

### Why This Layer Exists

**Problem**: Business logic embedded in framework code is impossible to test

**Solution**: Extract orchestration into pure functions with trait bounds

**Benefits**:
- Test workflows without file I/O or Git operations
- Share logic between CLI and GUI with zero duplication
- Add new frontends (HTTP API, gRPC) trivially
- Fast tests (<100ms vs seconds)

## Public API

### `analyze_repository`

Orchestrates repository analysis using trait-based dependencies.

```rust
pub async fn analyze_repository(
    path: &Path,
    ctx: impl AppContext,
    fs: impl FileSystem,
) -> Result<AnalysisResult>
```

**Workflow**:
1. Scan directory via `fs.read_dir_recursive()`
2. Filter supported file extensions
3. Parse and calculate metrics for each file
4. Report progress via `ctx.report_progress()`
5. Emit "analysis_complete" event
6. Return `AnalysisResult`

**Example (Production)**:
```rust
use code_viz_commands::analyze_repository;
use code_viz_core::traits::{AppContext, FileSystem};

let ctx = TauriContext::new(app_handle);
let fs = RealFileSystem::new();
let result = analyze_repository(Path::new("/path/to/repo"), ctx, fs).await?;
```

**Example (Testing)**:
```rust
use code_viz_commands::analyze_repository;
use code_viz_core::mocks::{MockContext, MockFileSystem};

let ctx = MockContext::new();
let fs = MockFileSystem::new()
    .with_file("src/main.rs", "fn main() {}")
    .with_file("src/lib.rs", "pub fn add(a: i32, b: i32) -> i32 { a + b }");

let result = analyze_repository(Path::new("src"), ctx.clone(), fs.clone()).await?;

// Verify orchestration behavior
ctx.assert_event_emitted("analysis_complete");
fs.assert_read(Path::new("src/main.rs"));
assert_eq!(result.summary.total_files, 2);
```

### `calculate_dead_code`

Orchestrates dead code analysis.

```rust
pub async fn calculate_dead_code(
    path: &Path,
    ctx: impl AppContext,
    fs: impl FileSystem,
    git: impl GitProvider,
) -> Result<DeadCodeResult>
```

### `export_report`

Exports analysis results (stub implementation).

```rust
pub async fn export_report(
    result: AnalysisResult,
    format: &str,
    ctx: impl AppContext,
    fs: impl FileSystem,
) -> Result<()>
```

## Testing Approach

### Unit Tests (Fast)

Use mocks from `code-viz-core::mocks` to test orchestration logic without I/O:

```rust
#[tokio::test]
async fn test_analyze_emits_progress() {
    let ctx = MockContext::new();
    let fs = MockFileSystem::new()
        .with_file("test.rs", "fn foo() {}");

    analyze_repository(Path::new("/fake"), ctx.clone(), fs).await.unwrap();

    let progress_events = ctx.get_events_by_name("progress");
    assert!(!progress_events.is_empty());
    assert_eq!(progress_events.last().unwrap()["percentage"], 1.0);
}
```

### Integration Tests

Test with real implementations:

```rust
#[tokio::test]
async fn test_real_filesystem() {
    let ctx = MockContext::new();
    let fs = RealFileSystem::new();
    let result = analyze_repository(Path::new("."), ctx, fs).await.unwrap();
    assert!(result.files.len() > 0);
}
```

## Dependencies

**Zero framework dependencies** by design:

```toml
[dependencies]
code-viz-core = { path = "../code-viz-core" }
async-trait = "0.1"
anyhow = "1.0"
serde_json = "1.0"
tokio = { version = "1", features = ["rt", "macros"] }
```

**Validation**:
```bash
$ cargo tree -p code-viz-commands --depth 1
code-viz-commands v0.1.0
├── anyhow v1.0.100
├── async-trait v0.1.89
├── code-viz-core v0.1.0
├── serde_json v1.0.145
└── tokio v1.48.0

# No tauri, clap, or GUI dependencies ✓
```

## Implementation Guidelines

### DO:
- Use trait bounds for all external dependencies (`impl AppContext`, `impl FileSystem`)
- Report progress at key milestones (0.1, 0.5, 0.9, 1.0)
- Emit domain events for significant state changes
- Return structured results (`Result<T>` with `anyhow::Error`)
- Write tests using mocks first, integration tests second

### DON'T:
- Import `std::fs`, `tauri`, `clap`, or other framework types
- Perform direct file I/O or system calls
- Handle presentation concerns (formatting, CLI args, IPC serialization)
- Duplicate business logic from `code-viz-core`
- Mix orchestration with algorithm implementation

## Performance

**Test suite**: <100ms total (3 tests with MockContext and MockFileSystem)

```bash
$ cargo test -p code-viz-commands
running 3 tests
test test_analyze_repository_success ... ok
test test_analyze_repository_empty_dir ... ok
test test_analyze_repository_error_handling ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## See Also

- [Architecture Documentation](../../docs/ARCHITECTURE.md) - Full architecture overview
- [Core Traits](../code-viz-core/src/traits/) - Trait definitions
- [Mock Implementations](../code-viz-core/src/mocks/) - Testing utilities
