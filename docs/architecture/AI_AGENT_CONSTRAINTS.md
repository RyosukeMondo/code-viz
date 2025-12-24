# AI Agent Implementation Constraints

**Purpose**: Formal constraints and rules for autonomous AI coding agents working on code-viz

**Last Updated**: 2025-12-24

---

## Mandatory Reading

Before implementing ANY code, agents MUST read in this order:

1. `.spec-workflow/steering/product.md` - Product vision and features
2. `.spec-workflow/steering/tech.md` - Technology stack and architecture
3. `.spec-workflow/steering/structure.md` - Project structure and conventions
4. `docs/architecture/ARCHITECTURE.md` - Trait-based DI architecture
5. `docs/architecture/diagrams/FRONTEND_BACKEND_ARCHITECTURE.md` - Full stack architecture
6. This document (AI_AGENT_CONSTRAINTS.md)

---

## Core Principles (NEVER VIOLATE)

### 1. Trait-Based Dependency Injection

**Rule**: ALL business logic MUST use trait abstractions, NEVER direct I/O

✅ **CORRECT**:
```rust
// Command layer (code-viz-commands)
pub async fn analyze_repository(
    path: &Path,
    ctx: impl AppContext,      // ✅ Trait bound
    fs: impl FileSystem,        // ✅ Trait bound
) -> Result<AnalysisResult> {
    let files = fs.read_dir_recursive(path)?;  // ✅ Uses trait method
    ctx.report_progress(0.5, "Analyzing...");  // ✅ Uses trait method
    // ... business logic ...
}
```

❌ **FORBIDDEN**:
```rust
// ❌ NEVER do this in command or core layer
pub async fn analyze_repository(path: &Path) -> Result<AnalysisResult> {
    let files = std::fs::read_dir(path)?;  // ❌ Direct filesystem access
    println!("Analyzing...");              // ❌ Direct console I/O
}
```

**Verification**: Run `rg "std::fs::" crates/code-viz-commands crates/code-viz-core --type rust`
- Result MUST be empty (except in test files)

### 2. Test-First Development

**Rule**: Write tests BEFORE production code

**Workflow**:
1. Write failing test that defines expected behavior
2. Run test, confirm it fails
3. Implement minimal code to make test pass
4. Run test, confirm it passes
5. Refactor if needed
6. Repeat

✅ **CORRECT**:
```rust
// Step 1: Write test first
#[tokio::test]
async fn test_analyze_emits_progress() {
    let ctx = MockContext::new();
    let fs = MockFileSystem::new()
        .with_file("main.rs", "fn main() {}");

    let result = analyze_repository(Path::new("/fake"), ctx.clone(), fs)
        .await
        .unwrap();

    // Assert expected behavior
    ctx.assert_event_emitted("analysis_complete");
}

// Step 2: Implement to make test pass
pub async fn analyze_repository(...) -> Result<...> {
    // Implementation here
    ctx.emit_event("analysis_complete", json!({}));
    Ok(result)
}
```

### 3. Single Source of Truth (SSOT)

**Rule**: Business logic lives ONLY in command layer, shared by all interfaces

**Architecture**:
```
Core Layer (code-viz-core)
    ↓ used by
Command Layer (code-viz-commands) ← SSOT for business logic
    ↓ used by               ↓ used by
Tauri GUI                   CLI Binary
```

❌ **FORBIDDEN**: Duplicating business logic in Tauri commands or CLI

**Verification**: Command wrappers MUST be <15 lines of code

### 4. No Backward Compatibility Required

**Rule**: Break existing APIs freely if it improves the codebase

**Rationale**: User's CLAUDE.md explicitly states:
> "No backward compatibility required unless explicitly requested."

✅ **ALLOWED**:
- Rename functions/types
- Change function signatures
- Remove unused exports
- Refactor module structure

❌ **FORBIDDEN**:
- Adding compatibility shims (`_old_function` renamed)
- Re-exporting deprecated types
- Keeping unused code with `// removed` comments

**When breaking APIs**: Update ALL call sites in same commit

---

## Code Quality Metrics (MUST ENFORCE)

### File Size
- **Max 500 lines/file** (excluding comments/blank lines)
- If exceeded: Extract modules or split into smaller files

### Function Size
- **Max 50 lines/function**
- If exceeded: Extract helper functions or refactor

### Test Coverage
- **Min 80% coverage** for all code
- **Min 90% coverage** for critical paths (command layer, core algorithms)

### Test Performance
- **Unit tests**: Must run in <100ms (no I/O)
- **Full test suite**: Must complete in <10 seconds
- **Contract tests**: Must complete in <1 second

### Dependency Constraints
- **code-viz-core**: ZERO external dependencies (except serde, anyhow for error handling)
- **code-viz-commands**: ZERO Tauri or GUI dependencies
- **Validation**: `cargo tree -p code-viz-commands` must show no Tauri deps

---

## Architectural Patterns (MUST FOLLOW)

### 1. Layer Separation

**Core Layer** (`code-viz-core`):
- ✅ Trait definitions
- ✅ Pure algorithms (no I/O)
- ✅ Domain models
- ✅ Mock implementations
- ❌ NO direct filesystem, network, or console access
- ❌ NO framework dependencies

**Command Layer** (`code-viz-commands`):
- ✅ Orchestration functions with trait bounds
- ✅ Progress reporting via `AppContext`
- ✅ Event emission
- ❌ NO direct I/O
- ❌ NO framework coupling

**Presentation Layer** (`code-viz-tauri`, `code-viz-cli`):
- ✅ Thin wrappers (11-15 LOC max)
- ✅ Framework-specific code
- ✅ Production trait implementations (`RealFileSystem`, `TauriContext`)
- ❌ NO business logic

### 2. Error Handling

**Rule**: Use structured errors, fail fast

✅ **CORRECT**:
```rust
use anyhow::{Context, Result};

pub fn parse_file(path: &Path, fs: &impl FileSystem) -> Result<Ast> {
    let content = fs.read_file(path)
        .context(format!("Failed to read file: {}", path.display()))?;

    let ast = tree_sitter_parse(&content)
        .context("Failed to parse syntax tree")?;

    Ok(ast)
}
```

❌ **FORBIDDEN**:
```rust
// ❌ Silent failures
pub fn parse_file(path: &Path, fs: &impl FileSystem) -> Option<Ast> {
    let content = fs.read_file(path).ok()?;  // ❌ Loses error context
    tree_sitter_parse(&content).ok()
}

// ❌ Catching and continuing
pub fn parse_file(...) -> Ast {
    match fs.read_file(path) {
        Ok(content) => { /* ... */ }
        Err(_) => return Ast::default()  // ❌ Hiding errors
    }
}
```

### 3. Logging

**Rule**: Use structured logging with `tracing` crate

✅ **CORRECT**:
```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(fs))]
pub async fn analyze_repository(
    path: &Path,
    ctx: impl AppContext,
    fs: impl FileSystem,
) -> Result<AnalysisResult> {
    info!(path = %path.display(), "Starting analysis");

    let files = fs.read_dir_recursive(path)?;
    info!(file_count = files.len(), "Discovered files");

    // ...
}
```

❌ **FORBIDDEN**:
```rust
// ❌ Using println! or eprintln!
println!("Starting analysis for {}", path.display());

// ❌ Logging secrets or PII
info!("API key: {}", api_key);  // ❌ Security violation
```

---

## Frontend Development (React/TypeScript)

### 1. Component Structure

**Rule**: Use functional components with hooks, no class components

✅ **CORRECT**:
```typescript
export const SunburstView: React.FC = () => {
  const { treeData, isAnalyzing } = useAnalysisStore();
  const { analyzeRepository } = useAnalysis();

  return (
    <div>
      {/* JSX */}
    </div>
  );
};
```

### 2. State Management

**Rule**: Use Zustand for global state, avoid Context API re-render issues

✅ **CORRECT**:
```typescript
// store/analysisStore.ts
export const useAnalysisStore = create<AnalysisStore>((set) => ({
  treeData: null,
  setTreeData: (data) => set({ treeData: data }),
}));

// Component
const treeData = useAnalysisStore(state => state.treeData);  // ✅ Selector-based
```

### 3. Tauri API Calls

**Rule**: Use typed bindings from `bindings.ts` (auto-generated by Specta)

✅ **CORRECT**:
```typescript
import { invoke } from '@tauri-apps/api/tauri';
import type { TreeNode } from './bindings';  // ✅ Generated types

const result = await invoke<TreeNode>('analyze_repository', { path });
```

---

## Forbidden Operations

Agents MUST NOT perform these operations without explicit human approval:

### Security
- ❌ Disable security features (CORS, CSP, sandboxing)
- ❌ Execute arbitrary code (`eval()`, `Function()`, unsafe Rust)
- ❌ Log secrets, API keys, passwords, or PII
- ❌ Make network requests to external services
- ❌ Read/write files outside project directory

### Git
- ❌ Force push to main/master
- ❌ Rewrite published history (`git rebase -i`, `git commit --amend` on pushed commits)
- ❌ Skip hooks (`--no-verify`, `--no-gpg-sign`)
- ❌ Hard reset (`git reset --hard`) without user confirmation

### Build System
- ❌ Modify `Cargo.toml` to add unsafe dependencies
- ❌ Change Rust edition or MSRV without approval
- ❌ Disable warnings or lints (`#[allow(clippy::all)]`)

### Testing
- ❌ Disable tests (`#[ignore]` without justification)
- ❌ Increase timeouts to mask slow code
- ❌ Mock production implementations in non-test code

---

## Pre-Implementation Checklist

Before starting ANY implementation, verify:

- [ ] I have read all steering documents
- [ ] I understand the trait-based DI architecture
- [ ] I know which layer my code belongs in (Core/Command/Presentation)
- [ ] I have searched existing implementations (grep/ripgrep)
- [ ] I have a failing test that defines expected behavior
- [ ] I will NOT use direct I/O in Core or Command layers

---

## Implementation Workflow

### 1. Research Phase
```bash
# Search existing implementations
rg "similar_function_name" crates/
rg "struct SimilarType" crates/

# Check trait definitions
cat crates/code-viz-core/src/traits/*.rs

# Review related tests
find crates/ -name "*test*.rs" -exec rg "test_similar" {} \;
```

### 2. Specification Phase (for non-trivial features)
```bash
# Create spec
.spec-workflow/specs/my-feature/
  ├── requirements.md   # What to build
  ├── design.md         # How to build
  └── tasks.md          # Step-by-step tasks
```

### 3. Test-First Implementation
```rust
// Step 1: Write test
#[tokio::test]
async fn test_my_feature() {
    // Arrange
    let ctx = MockContext::new();
    let fs = MockFileSystem::new().with_file(...);

    // Act
    let result = my_feature(ctx, fs).await;

    // Assert
    assert!(result.is_ok());
}

// Step 2: Run test (should fail)
cargo nextest run test_my_feature

// Step 3: Implement
pub async fn my_feature(ctx: impl AppContext, fs: impl FileSystem) -> Result<()> {
    // Implementation
}

// Step 4: Run test (should pass)
cargo nextest run test_my_feature
```

### 4. Validation Phase
```bash
# Run all tests
cargo nextest run --workspace --all-targets

# Run contract tests
cargo nextest run --package code-viz-tauri --test contract_tests

# Check no forbidden patterns
rg "std::fs::" crates/code-viz-commands crates/code-viz-core --type rust
# Should be empty

# Verify dependency constraints
cargo tree -p code-viz-commands --depth 1
# Should show NO Tauri dependencies

# Check code metrics
./scripts/check-code-metrics.sh  # To be created
```

### 5. Logging Phase
```bash
# Log implementation artifacts using MCP tool
{
  "taskId": "1.2.3",
  "summary": "Added X feature",
  "artifacts": {
    "functions": [{
      "name": "my_feature",
      "purpose": "Does X",
      "location": "crates/code-viz-commands/src/my_module.rs:42",
      "signature": "pub async fn my_feature(ctx: impl AppContext) -> Result<()>",
      "isExported": true
    }]
  },
  "filesModified": ["crates/code-viz-commands/src/my_module.rs"],
  "filesCreated": [],
  "statistics": { "linesAdded": 45, "linesRemoved": 3 }
}
```

---

## Error Recovery Protocol

When encountering errors:

### Compilation Errors
1. Read error message carefully
2. Identify root cause (missing import, type mismatch, etc.)
3. Fix ONCE with minimal change
4. Recompile
5. If error persists after 2 attempts → STOP, ask human

### Test Failures
1. Read test output
2. Identify assertion failure or panic
3. Fix implementation (NOT the test)
4. Rerun test
5. If failure persists after 2 attempts → STOP, ask human

### Repeated Failures
- **3 consecutive failures** on same issue → STOP, ask human
- **Never** disable tests or warnings to make failures "go away"
- **Never** modify tests to match buggy implementation

---

## Review Checklist

Before creating PR or finalizing implementation:

### Code Quality
- [ ] All functions <50 lines
- [ ] All files <500 lines
- [ ] No code duplication (DRY principle)
- [ ] Descriptive names (no `temp`, `data`, `x`)

### Architecture
- [ ] Core layer has no I/O
- [ ] Command layer uses trait bounds
- [ ] Presentation layer is thin wrappers
- [ ] No business logic in Tauri commands

### Testing
- [ ] All tests pass
- [ ] Contract tests pass
- [ ] Coverage ≥80% (≥90% for critical paths)
- [ ] No flaky tests (run suite 3x to verify)

### Documentation
- [ ] Implementation logged with artifacts
- [ ] Public APIs documented with examples
- [ ] Architectural decisions recorded (if significant)
- [ ] README updated (if user-facing change)

---

## Summary

**Core Philosophy**: Clean architecture, test-first, trait-based DI, SSOT

**Golden Rules**:
1. Read steering docs FIRST
2. Follow existing patterns (no invention)
3. Write tests BEFORE code
4. Use traits, NEVER direct I/O
5. Keep wrappers thin (<15 LOC)
6. Log implementations completely
7. When in doubt, ASK

**Remember**: These constraints exist to maintain code quality, testability, and architectural integrity. Violations risk creating unmaintainable code that defeats the project's purpose of combating AI-generated bloat.
