# AI Agent Error Recovery Runbook

**Purpose**: Automated diagnostic and recovery procedures for common errors during autonomous implementation

**Last Updated**: 2025-12-24

---

## Error Recovery Philosophy

1. **Fail Fast**: Detect errors immediately, don't propagate
2. **Diagnose First**: Understand root cause before attempting fix
3. **Minimal Fix**: Change only what's necessary to resolve error
4. **Verify Fix**: Confirm error resolved before proceeding
5. **Limit Attempts**: Max 2 fix attempts, then escalate to human

---

## Error Categories

- [Compilation Errors](#compilation-errors)
- [Test Failures](#test-failures)
- [Contract Validation Failures](#contract-validation-failures)
- [Dependency Errors](#dependency-errors)
- [Architecture Violations](#architecture-violations)
- [Performance Regressions](#performance-regressions)

---

## Compilation Errors

### Error: "cannot find trait `AppContext` in scope"

**Symptom**:
```
error[E0405]: cannot find trait `AppContext` in this scope
  --> src/commands/my_feature.rs:5:20
```

**Diagnosis**:
```bash
# Check if trait is defined
rg "pub trait AppContext" crates/code-viz-core/src/traits/

# Check if module is public
cat crates/code-viz-core/src/lib.rs | rg "pub mod traits"
```

**Recovery**:
```rust
// Add import
use code_viz_core::traits::AppContext;

// Or use fully qualified path
pub async fn my_feature(ctx: impl code_viz_core::AppContext) { ... }
```

**Verification**:
```bash
cargo check --package code-viz-commands
```

---

### Error: "trait bound not satisfied"

**Symptom**:
```
error[E0277]: the trait bound `RealFileSystem: FileSystem` is not satisfied
  --> src/commands.rs:42:15
```

**Diagnosis**:
```bash
# Check trait implementation exists
rg "impl FileSystem for RealFileSystem" crates/

# Check trait is in scope
rg "use.*FileSystem" src/commands.rs
```

**Recovery**:
1. Ensure trait is imported:
   ```rust
   use code_viz_core::traits::FileSystem;
   ```

2. Ensure impl block exists:
   ```rust
   impl FileSystem for RealFileSystem {
       fn read_file(&self, path: &Path) -> Result<String> { ... }
   }
   ```

**Verification**:
```bash
cargo check --all-targets
```

---

### Error: "direct filesystem access detected"

**Symptom**:
```
error: found `std::fs::read_to_string` in core/command layer
```

**Diagnosis**: Architecture violation - direct I/O in wrong layer

**Recovery**:
```rust
// ❌ WRONG (direct I/O)
pub fn parse_file(path: &Path) -> Result<Ast> {
    let content = std::fs::read_to_string(path)?;  // ❌
    parse_tree_sitter(&content)
}

// ✅ CORRECT (use trait)
pub fn parse_file(path: &Path, fs: &impl FileSystem) -> Result<Ast> {
    let content = fs.read_file(path)?;  // ✅
    parse_tree_sitter(&content)
}
```

**Verification**:
```bash
# Should return nothing
rg "std::fs::" crates/code-viz-core crates/code-viz-commands --type rust
```

---

## Test Failures

### Error: Test panic - "assertion failed"

**Symptom**:
```
thread 'test_analyze_emits_progress' panicked at 'assertion failed: events.len() >= 5'
  left: 3
  right: 5
```

**Diagnosis**:
```bash
# Run specific test with output
cargo nextest run test_analyze_emits_progress -- --nocapture

# Check mock expectations
rg "MockContext::new" tests/
```

**Recovery Steps**:
1. **Verify expected behavior**: Read test to understand what it expects
2. **Check implementation**: Ensure implementation meets expectations
3. **Fix implementation** (NOT the test):
   ```rust
   // If test expects 5 progress events, emit 5:
   ctx.report_progress(0.2, "Step 1");
   ctx.report_progress(0.4, "Step 2");
   ctx.report_progress(0.6, "Step 3");
   ctx.report_progress(0.8, "Step 4");
   ctx.report_progress(1.0, "Complete");
   ```

**Verification**:
```bash
# Run test 3 times to ensure not flaky
for i in {1..3}; do cargo nextest run test_analyze_emits_progress; done
```

---

### Error: "Mock expectation failed"

**Symptom**:
```
thread 'test_reads_files' panicked at 'MockFileSystem: unexpected call to read_file("/unexpected/path")'
```

**Diagnosis**: Implementation is calling methods the mock doesn't expect

**Recovery**:
1. **Check mock setup**:
   ```rust
   let fs = MockFileSystem::new()
       .with_file("expected.rs", "content");  // Only expects this file
   ```

2. **Fix implementation** to match mock OR **update mock** to match implementation:
   ```rust
   // Option 1: Fix implementation
   let content = fs.read_file(Path::new("expected.rs"))?;  // ✅

   // Option 2: Update mock
   let fs = MockFileSystem::new()
       .with_file("expected.rs", "content")
       .with_file("unexpected/path", "other content");  // ✅
   ```

**Verification**:
```bash
cargo nextest run test_reads_files
```

---

### Error: "Test timeout"

**Symptom**:
```
test test_analyze_repository timed out after 60 seconds
```

**Diagnosis**: Infinite loop, deadlock, or actual slowness

**Recovery Steps**:
1. **Add tracing** to identify bottleneck:
   ```rust
   #[instrument]
   pub async fn analyze_repository(...) -> Result<...> {
       info!("Starting analysis");
       let files = fs.read_dir_recursive(path)?;
       info!("Read {} files", files.len());  // ← Check if we get here
       // ...
   }
   ```

2. **Run with debug output**:
   ```bash
   RUST_LOG=debug cargo nextest run test_analyze_repository -- --nocapture
   ```

3. **Check for blocking operations**:
   ```rust
   // ❌ Blocking in async context
   let result = blocking_operation();  // Could cause hang

   // ✅ Use tokio spawn_blocking
   let result = tokio::task::spawn_blocking(|| blocking_operation()).await?;
   ```

**Verification**:
```bash
# Should complete in <1 second
time cargo nextest run test_analyze_repository
```

---

## Contract Validation Failures

### Error: "Type mismatch - Rust struct doesn't match TypeScript interface"

**Symptom**:
```
Contract test failed: TreeNode serialization mismatch
Expected: { "id": "...", "name": "...", "value": 123 }
Got:      { "id": "...", "name": "..." }
```

**Diagnosis**: Rust struct changed but TypeScript types not regenerated

**Recovery**:
```bash
# Regenerate TypeScript bindings
cargo build --package code-viz-tauri

# Check generated types
cat src/bindings.ts | rg "interface TreeNode"
```

**If field is missing**:
```rust
// Ensure field is public and has serde attribute
#[derive(Serialize, Deserialize, specta::Type)]
pub struct TreeNode {
    pub id: String,
    pub name: String,
    pub value: u64,  // ← Ensure this is pub and has correct type
}
```

**Verification**:
```bash
cargo nextest run --package code-viz-tauri --test contract_tests
```

---

### Error: "Serialization failed - non-finite float value"

**Symptom**:
```
Contract test failed: cannot serialize NaN or Infinity
```

**Diagnosis**: Floating-point division by zero or invalid calculation

**Recovery**:
```rust
// ❌ Can produce NaN or Infinity
let ratio = dead_loc as f64 / total_loc as f64;

// ✅ Guard against division by zero
let ratio = if total_loc == 0 {
    0.0
} else {
    (dead_loc as f64 / total_loc as f64).clamp(0.0, 1.0)
};
```

**Verification**:
```bash
# Add test case with edge case
#[test]
fn test_ratio_with_zero_denominator() {
    let result = calculate_ratio(10, 0);
    assert!(result.is_finite());
    assert_eq!(result, 0.0);
}
```

---

## Dependency Errors

### Error: "Tauri dependency found in command layer"

**Symptom**:
```
$ cargo tree -p code-viz-commands | grep tauri
tauri v2.0.0
```

**Diagnosis**: Accidental Tauri import in framework-agnostic layer

**Recovery**:
```bash
# Find the import
rg "use tauri" crates/code-viz-commands/

# Remove it
# Replace with trait abstraction
```

**Example**:
```rust
// ❌ WRONG
use tauri::AppHandle;

pub fn my_command(app: AppHandle) { ... }

// ✅ CORRECT
use code_viz_core::traits::AppContext;

pub fn my_command(ctx: impl AppContext) { ... }
```

**Verification**:
```bash
# Should show NO tauri dependencies
cargo tree -p code-viz-commands --depth 1 | rg tauri
# Expected: no output
```

---

### Error: "Dependency not found"

**Symptom**:
```
error: no matching package named `code-viz-core` found
```

**Diagnosis**: Workspace member not added to `Cargo.toml`

**Recovery**:
```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "crates/code-viz-core",      # ← Ensure this exists
    "crates/code-viz-commands",
    "crates/code-viz-cli",
    "crates/code-viz-tauri",
]
```

**Verification**:
```bash
cargo metadata --format-version 1 | jq '.workspace_members'
```

---

## Architecture Violations

### Error: "Business logic detected in Tauri command"

**Symptom**: Manual code review finds >15 lines in command wrapper

**Diagnosis**: Business logic leaking into presentation layer

**Recovery**:
```rust
// ❌ WRONG (35 lines of business logic in Tauri command)
#[tauri::command]
pub async fn analyze_repository(app: AppHandle, path: String) -> Result<TreeNode, String> {
    let files = std::fs::read_dir(&path).unwrap();  // ❌ Business logic
    let mut total_loc = 0;
    for file in files {
        // ... 30 more lines of logic ...  // ❌ Business logic
    }
    Ok(TreeNode { ... })
}

// ✅ CORRECT (11 lines - thin wrapper)
#[tauri::command]
pub async fn analyze_repository(
    app: AppHandle,
    path: String,
) -> Result<TreeNode, String> {
    let ctx = TauriContext::new(app);
    let fs = RealFileSystem::new();

    code_viz_commands::analyze_repository(&PathBuf::from(path), ctx, fs)
        .await
        .map_err(|e| e.to_string())
}
```

**Verification**:
```bash
# Count lines in command functions
sed -n '/^pub async fn analyze_repository/,/^}/p' src/commands.rs | wc -l
# Should be <15
```

---

### Error: "Test depends on real filesystem"

**Symptom**: Test fails on CI but passes locally

**Diagnosis**: Test is using real filesystem instead of mocks

**Recovery**:
```rust
// ❌ WRONG (depends on real filesystem)
#[tokio::test]
async fn test_analyze() {
    let result = analyze_repository(Path::new("./test-data")).await;  // ❌
    assert!(result.is_ok());
}

// ✅ CORRECT (uses mocks)
#[tokio::test]
async fn test_analyze() {
    let ctx = MockContext::new();
    let fs = MockFileSystem::new()
        .with_file("test.rs", "fn main() {}");

    let result = analyze_repository(Path::new("/fake"), ctx, fs).await;
    assert!(result.is_ok());
}
```

**Verification**:
```bash
# Unit tests should run with no filesystem access
# Should complete in <100ms
time cargo nextest run --lib --all-targets
```

---

## Performance Regressions

### Error: "Test suite taking >10 seconds"

**Symptom**:
```
Summary [12.345s] 170 tests run: 170 passed
```

**Diagnosis**: Slow tests introduced, likely due to real I/O

**Recovery**:
1. **Identify slow tests**:
   ```bash
   cargo nextest run --profile ci --slow-timeout 1s
   # Lists tests taking >1 second
   ```

2. **Fix slow tests** by using mocks:
   ```rust
   // Replace RealFileSystem with MockFileSystem
   // Replace actual Git repo with MockGit
   ```

3. **Optimize algorithms** if legitimately complex:
   ```rust
   // Use parallel iterators
   use rayon::prelude::*;

   files.par_iter().map(|f| analyze_file(f)).collect()
   ```

**Verification**:
```bash
# Should complete in <10 seconds
time cargo nextest run --workspace --all-targets
```

---

### Error: "Analysis taking >5 seconds for small repo"

**Symptom**: User reports slow performance on <100 files

**Diagnosis**: Missing caching or inefficient algorithm

**Recovery**:
1. **Add caching**:
   ```rust
   // Cache parsed ASTs
   let mut cache = HashMap::new();
   if let Some(cached) = cache.get(path) {
       return Ok(cached.clone());
   }
   ```

2. **Profile to find bottleneck**:
   ```bash
   cargo build --release
   perf record ./target/release/code-viz analyze ./test-repo
   perf report
   ```

3. **Parallelize if sequential**:
   ```rust
   // ❌ Sequential
   for file in files {
       analyze_file(file)?;
   }

   // ✅ Parallel
   files.par_iter()
       .map(|file| analyze_file(file))
       .collect::<Result<Vec<_>>>()?;
   ```

**Verification**:
```bash
# Benchmark before/after
hyperfine "cargo run -- analyze ./test-repo"
```

---

## Escalation Protocol

If error persists after 2 recovery attempts:

### 1. Collect Context
```bash
# Capture error output
cargo build 2>&1 | tee build-error.log
cargo nextest run 2>&1 | tee test-error.log

# Capture environment
rustc --version > environment.txt
cargo --version >> environment.txt
uname -a >> environment.txt
```

### 2. Create Diagnostic Report
```markdown
# Error Diagnostic Report

## Error Type
[Compilation / Test Failure / Contract Validation / etc.]

## Error Message
```
[Paste full error output]
```

## Recovery Attempts
1. [What was tried]
   - Result: [Success/Failure]
2. [What was tried]
   - Result: [Success/Failure]

## Environment
- Rust version: [...]
- OS: [...]
- Workspace state: [git status output]

## Hypothesis
[What might be causing this]

## Request
[What help is needed from human]
```

### 3. Escalate to Human
- Create GitHub issue with diagnostic report
- Tag as `ai-agent-blocked`
- Pause autonomous implementation
- Wait for human guidance

---

## Prevention Checklist

To avoid common errors:

**Before Writing Code**:
- [ ] Read steering documents
- [ ] Understand architecture (trait-based DI)
- [ ] Search existing implementations (grep)
- [ ] Write failing test first

**While Writing Code**:
- [ ] Use trait bounds, not direct I/O
- [ ] Keep functions <50 lines
- [ ] Keep files <500 lines
- [ ] Run tests frequently

**Before Committing**:
- [ ] All tests pass
- [ ] Contract tests pass
- [ ] No architecture violations
- [ ] Code metrics within limits

---

## Quick Reference

### Health Check Command
```bash
# Run this to verify everything is working
./scripts/health-check.sh  # To be created

# Manual version:
cargo build --workspace && \
cargo nextest run --workspace --all-targets && \
cargo tree -p code-viz-commands | grep -q tauri && echo "❌ FAIL: Tauri dep found" || echo "✅ PASS" && \
rg "std::fs::" crates/code-viz-{core,commands} --type rust && echo "❌ FAIL: Direct I/O found" || echo "✅ PASS"
```

### Common Recovery Commands
```bash
# Regenerate bindings
cargo build --package code-viz-tauri

# Clean and rebuild
cargo clean && cargo build

# Run specific test
cargo nextest run test_name

# Check dependency tree
cargo tree -p code-viz-commands

# Find architectural violations
rg "std::fs::" crates/code-viz-{core,commands} --type rust
```

---

**Last Updated**: 2025-12-24
**Maintainer**: Code-Viz Team
