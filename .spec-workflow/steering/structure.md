# Project Structure

## Directory Organization

```
code-viz/
├── src-tauri/                      # Rust backend (Tauri application)
│   ├── src/
│   │   ├── main.rs                 # Application entry point
│   │   ├── lib.rs                  # Library exports for plugin system
│   │   ├── commands/               # Tauri command handlers
│   │   │   ├── mod.rs
│   │   │   ├── analysis.rs         # Analysis-related commands
│   │   │   ├── git.rs              # Git history commands
│   │   │   └── config.rs           # Configuration commands
│   │   ├── analysis/               # Analysis engine core
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs           # Tree-sitter wrapper
│   │   │   ├── metrics.rs          # Complexity calculations
│   │   │   ├── graph.rs            # Stack-graphs integration
│   │   │   └── dead_code.rs        # Dead code detection
│   │   ├── git/                    # Git integration
│   │   │   ├── mod.rs
│   │   │   ├── history.rs          # Commit history traversal
│   │   │   ├── diff.rs             # Diff analysis
│   │   │   └── churn.rs            # Churn metrics
│   │   ├── cache/                  # Caching layer
│   │   │   ├── mod.rs
│   │   │   ├── memory.rs           # In-memory cache
│   │   │   └── disk.rs             # Disk-backed cache (sled/SQLite)
│   │   ├── models/                 # Data models (shared with frontend via specta)
│   │   │   ├── mod.rs
│   │   │   ├── metrics.rs          # Metric data structures
│   │   │   ├── graph.rs            # Graph data structures
│   │   │   └── events.rs           # Event types for IPC
│   │   └── utils/                  # Utility functions
│   │       ├── mod.rs
│   │       └── logging.rs          # Tracing setup
│   ├── Cargo.toml                  # Rust dependencies
│   ├── tauri.conf.json             # Tauri configuration
│   ├── build.rs                    # Build script (specta codegen)
│   └── icons/                      # Application icons
│
├── src/                            # React frontend
│   ├── main.tsx                    # React entry point
│   ├── App.tsx                     # Root component
│   ├── bindings.ts                 # Auto-generated Tauri bindings (from specta)
│   ├── components/                 # Reusable UI components
│   │   ├── layout/
│   │   │   ├── AppLayout.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   └── Toolbar.tsx
│   │   ├── visualizations/
│   │   │   ├── Treemap.tsx         # 2D treemap (ECharts)
│   │   │   ├── CodeCity.tsx        # 3D code city (R3F)
│   │   │   ├── Timeline.tsx        # Git history timeline
│   │   │   └── MetricsPanel.tsx    # Metrics display
│   │   └── common/
│   │       ├── Button.tsx
│   │       ├── Modal.tsx
│   │       └── Tooltip.tsx
│   ├── features/                   # Feature-based modules
│   │   ├── analysis/
│   │   │   ├── AnalysisView.tsx
│   │   │   ├── useAnalysis.ts      # Custom hook
│   │   │   └── types.ts
│   │   ├── history/
│   │   │   ├── HistoryView.tsx
│   │   │   ├── useHistory.ts
│   │   │   └── types.ts
│   │   └── settings/
│   │       ├── SettingsView.tsx
│   │       └── useSettings.ts
│   ├── hooks/                      # Shared custom hooks
│   │   ├── useTauriCommand.ts      # Wrapper for IPC calls
│   │   ├── useTauriEvent.ts        # Event subscription hook
│   │   └── useFileWatcher.ts
│   ├── store/                      # State management (Zustand)
│   │   ├── index.ts
│   │   ├── analysisStore.ts
│   │   └── uiStore.ts
│   ├── utils/                      # Frontend utilities
│   │   ├── formatting.ts           # Number/date formatting
│   │   ├── colors.ts               # Color mapping for metrics
│   │   └── validation.ts
│   ├── styles/                     # Global styles
│   │   └── globals.css             # Tailwind imports
│   └── types/                      # TypeScript type definitions
│       └── index.ts
│
├── crates/                         # Workspace crates (library-first architecture)
│   ├── code-viz-core/              # Pure business logic (ZERO DEPENDENCIES)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── traits/            # Trait abstractions (AppContext, FileSystem, GitProvider)
│   │   │   ├── analysis/          # Analysis algorithms (pure functions)
│   │   │   ├── metrics/           # Metric calculations (pure functions)
│   │   │   └── models/            # Data structures (TreeNode, AnalysisResult)
│   │   ├── tests/                 # Unit tests (100+ tests, <1s runtime)
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── code-viz-commands/          # Orchestration layer (TRAIT-BASED)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── analyze.rs         # analyze_repository(ctx, fs) -> Result
│   │   │   ├── dead_code.rs       # calculate_dead_code(ctx, fs) -> Result
│   │   │   └── export.rs          # export_report(ctx, fs, format) -> Result
│   │   ├── tests/                 # Command layer tests (50+ tests, ~2s runtime)
│   │   │   ├── analyze_tests.rs   # Tests with MockContext, MockFileSystem
│   │   │   └── fixtures/          # Test data (not repositories)
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── code-viz-cli/               # CLI wrapper (THIN PRESENTATION LAYER)
│   │   ├── src/
│   │   │   ├── main.rs            # Clap argument parsing + CliContext
│   │   │   └── formatters/        # Output formatting only
│   │   ├── tests/                 # Presentation tests only (arg parsing, formatting)
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── code-viz-tauri/             # Tauri wrapper (THIN PRESENTATION LAYER)
│   │   ├── src/
│   │   │   ├── main.rs            # Tauri setup + TauriContext
│   │   │   ├── commands.rs        # Thin IPC wrappers calling code-viz-commands
│   │   │   └── context/           # TauriContext, MockContext implementations
│   │   ├── tests/                 # Contract + IPC wrapper tests
│   │   │   ├── contract_tests.rs  # Specta schema validation
│   │   │   └── wrapper_tests.rs   # IPC binding validation
│   │   ├── Cargo.toml
│   │   └── tauri.conf.json
│   └── code-viz-dead-code/         # Specialized analysis library
│       ├── src/
│       │   └── lib.rs
│       ├── Cargo.toml
│       └── README.md
│
├── tests/                          # Test pyramid implementation
│   ├── unit/                       # Unit tests (in-crate #[cfg(test)])
│   │   └── README.md               # Unit test guidelines
│   ├── contract/                   # Contract validation tests
│   │   ├── specta_schema_tests.rs  # Rust → TS type validation
│   │   └── serialization_tests.rs  # Round-trip tests
│   ├── integration/                # CLI integration tests
│   │   ├── cli_analysis_tests.sh   # Shell-based CLI tests
│   │   └── fixtures/               # Test repositories
│   └── e2e/                        # Playwright E2E (minimal)
│       └── smoke_test.spec.ts      # Single critical path test
│
├── benchmarks/                     # Performance benchmarks
│   ├── analysis_bench.rs
│   └── parsing_bench.rs
│
├── docs/                           # Documentation
│   ├── architecture.md             # High-level architecture
│   ├── api/                        # API documentation
│   ├── search.md                   # Original specification
│   └── fast-iteration.md           # Development workflow doc
│
├── scripts/                        # Build and utility scripts
│   ├── generate-llm-context.sh     # Generate LLM.md
│   └── setup-dev.sh                # Dev environment setup
│
├── .zellij/                        # Zellij terminal layouts
│   └── layout.kdl
│
├── .cargo/                         # Cargo configuration
│   └── config.toml                 # Linker settings (mold)
│
├── .vscode/                        # VSCode settings
│   └── settings.json               # rust-analyzer config
│
├── .spec-workflow/                 # Spec workflow (steering docs)
│   ├── steering/
│   │   ├── product.md
│   │   ├── tech.md
│   │   └── structure.md
│   └── templates/
│
├── .cursorrules                    # AI agent guidelines
├── LLM.md                          # Auto-generated AI context
├── Justfile                        # Task runner recipes
├── Cargo.toml                      # Workspace root (if using crates/)
├── package.json                    # Frontend dependencies
├── vite.config.ts                  # Vite configuration
├── tailwind.config.js              # Tailwind CSS config
├── tsconfig.json                   # TypeScript configuration
├── .gitignore
└── README.md
```

## Naming Conventions

### Files

#### Rust (Backend)
- **Modules**: `snake_case.rs` (e.g., `dead_code.rs`, `git_history.rs`)
- **Tests**: `#[cfg(test)] mod tests` inline or `tests/` directory with `_test.rs` suffix
- **Binary**: `main.rs` (application entry point)
- **Library**: `lib.rs` (library exports)

#### TypeScript/React (Frontend)
- **Components**: `PascalCase.tsx` (e.g., `Treemap.tsx`, `CodeCity.tsx`)
- **Hooks**: `camelCase.ts` with `use` prefix (e.g., `useAnalysis.ts`, `useTauriCommand.ts`)
- **Utilities**: `camelCase.ts` (e.g., `formatting.ts`, `colors.ts`)
- **Tests**: `[filename].test.ts` or `[filename].spec.ts`
- **Types**: `types.ts` or `index.ts` (for barrel exports)

### Code

#### Rust
- **Structs/Enums**: `PascalCase` (e.g., `CodeMetric`, `AnalysisResult`)
- **Traits**: `PascalCase` (e.g., `MetricCalculator`, `Parser`)
- **Functions**: `snake_case` (e.g., `calculate_complexity`, `parse_file`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_FILE_SIZE`, `DEFAULT_TIMEOUT`)
- **Lifetimes**: `'a`, `'b` (lowercase single letters)
- **Generics**: `T`, `U`, `Item` (descriptive or single uppercase letters)

#### TypeScript/React
- **Interfaces/Types**: `PascalCase` (e.g., `AnalysisResult`, `MetricData`)
- **Functions**: `camelCase` (e.g., `calculateComplexity`, `formatBytes`)
- **React Components**: `PascalCase` (e.g., `Treemap`, `CodeCity`)
- **Constants**: `SCREAMING_SNAKE_CASE` or `camelCase` (e.g., `MAX_NODES` or `maxNodes`)
- **Variables**: `camelCase` (e.g., `fileCount`, `metricData`)
- **Enum Values**: `PascalCase` (e.g., `MetricType.CognitiveComplexity`)

## Import Patterns

### Rust Import Order
1. Standard library (`use std::...`)
2. External crates (`use serde::...`, `use tauri::...`)
3. Workspace crates (`use code_viz_analysis::...`)
4. Local modules (`use crate::...`, `use super::...`)

**Example:**
```rust
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::command;
use tree_sitter::Parser;

use crate::models::CodeMetric;
use crate::utils::logging::setup_tracing;

use super::parser::parse_file;
```

### TypeScript Import Order
1. React core (`import React from 'react'`)
2. External libraries (`import { Canvas } from '@react-three/fiber'`)
3. Internal components/features (`import { Treemap } from '@/components'`)
4. Hooks and stores (`import { useAnalysis } from '@/hooks'`)
5. Utilities and types (`import { formatBytes } from '@/utils'`)
6. Styles (`import './styles.css'`)

**Path Aliases** (configured in tsconfig.json):
- `@/` → `src/`
- `@components/` → `src/components/`
- `@hooks/` → `src/hooks/`
- `@store/` → `src/store/`

**Example:**
```typescript
import React, { useEffect, useState } from 'react';

import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';

import { Treemap } from '@/components/visualizations';
import { useAnalysis } from '@/hooks/useAnalysis';
import { analysisStore } from '@/store';

import { formatBytes } from '@/utils/formatting';
import type { AnalysisResult } from '@/types';

import './CodeCity.css';
```

## Code Structure Patterns

### Rust Module Organization

```rust
// 1. Imports (grouped by origin)
use std::collections::HashMap;
use serde::Serialize;
use crate::models::CodeMetric;

// 2. Constants and static configuration
const MAX_COMPLEXITY_THRESHOLD: u32 = 50;

// 3. Type definitions (structs, enums, type aliases)
#[derive(Debug, Clone, Serialize)]
pub struct ComplexityCalculator {
    threshold: u32,
    cache: HashMap<String, u32>,
}

// 4. Trait definitions
pub trait MetricCalculator {
    fn calculate(&self, code: &str) -> Result<u32, Error>;
}

// 5. Main implementation blocks
impl ComplexityCalculator {
    pub fn new(threshold: u32) -> Self {
        Self {
            threshold,
            cache: HashMap::new(),
        }
    }
}

// 6. Trait implementations
impl MetricCalculator for ComplexityCalculator {
    fn calculate(&self, code: &str) -> Result<u32, Error> {
        // Implementation
    }
}

// 7. Private helper functions
fn normalize_score(score: u32) -> f64 {
    // Helper logic
}

// 8. Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complexity_calculation() {
        // Test logic
    }
}
```

### React Component Organization

```typescript
// 1. Imports
import React, { useState, useEffect } from 'react';
import { useAnalysis } from '@/hooks/useAnalysis';

// 2. Types/Interfaces
interface TreemapProps {
  data: AnalysisResult;
  onNodeClick?: (nodeId: string) => void;
}

// 3. Constants
const DEFAULT_COLOR_SCHEME = ['#22c55e', '#eab308', '#ef4444'];

// 4. Component definition
export const Treemap: React.FC<TreemapProps> = ({ data, onNodeClick }) => {
  // 4a. State hooks
  const [selectedNode, setSelectedNode] = useState<string | null>(null);

  // 4b. Custom hooks
  const { metrics, isLoading } = useAnalysis();

  // 4c. Side effects
  useEffect(() => {
    // Setup logic
    return () => {
      // Cleanup
    };
  }, [data]);

  // 4d. Event handlers
  const handleNodeClick = (nodeId: string) => {
    setSelectedNode(nodeId);
    onNodeClick?.(nodeId);
  };

  // 4e. Render helpers
  const renderNode = (node: Node) => {
    // Render logic
  };

  // 4f. Early returns
  if (isLoading) return <Loading />;
  if (!data) return <EmptyState />;

  // 4g. Main render
  return (
    <div className="treemap-container">
      {/* JSX */}
    </div>
  );
};

// 5. Display name (for debugging)
Treemap.displayName = 'Treemap';

// 6. Default export (if applicable)
export default Treemap;
```

### Function Organization Principles

```rust
// Example: Well-organized function
pub fn analyze_repository(
    repo_path: PathBuf,
    options: AnalysisOptions,
) -> Result<AnalysisResult, AnalysisError> {
    // 1. Input validation
    if !repo_path.exists() {
        return Err(AnalysisError::InvalidPath(repo_path));
    }

    // 2. Setup/initialization
    let parser = Parser::new()?;
    let mut metrics = Vec::new();

    // 3. Core logic
    for file in discover_files(&repo_path)? {
        let metric = calculate_file_metrics(&file, &parser)?;
        metrics.push(metric);
    }

    // 4. Aggregation/post-processing
    let summary = aggregate_metrics(&metrics);

    // 5. Return result
    Ok(AnalysisResult {
        metrics,
        summary,
        timestamp: Utc::now(),
    })
}
```

## Code Organization Principles

### SOLID Principles (Enforced)

1. **Single Responsibility**: Each module/file has ONE clear purpose
   - `parser.rs` only handles parsing (not metrics or visualization)
   - `Treemap.tsx` only renders treemap (not data fetching or state management)

2. **Open/Closed**: Extend via traits/interfaces, not modification
   - Use trait `MetricCalculator` for new metrics (don't modify existing calculators)

3. **Liskov Substitution**: Implementations must be interchangeable
   - Any `MetricCalculator` implementation works in analysis pipeline

4. **Interface Segregation**: Small, focused traits/interfaces
   - Don't create `SuperAnalyzer` trait; use `Parser`, `MetricCalculator`, `Grapher` separately

5. **Dependency Injection**: All external deps (APIs, DBs) injected
   - Pass `Parser` as parameter, don't create inside function
   - Mock-friendly for testing

### DRY (Don't Repeat Yourself)
- Shared Rust logic → Extract to `utils/` or create workspace crate
- Shared React logic → Custom hooks or utility functions
- Duplicate constants → Move to config file or shared module

### KISS (Keep It Simple, Stupid)
- Prefer flat structures over deep nesting
- Avoid premature abstraction (wait for 3rd use case before generalizing)
- No "clever" one-liners; readability > brevity

### SLAP (Single Level of Abstraction Principle)
- Functions should operate at one abstraction level
- Mix of high-level (`analyze_repository()`) and low-level (`read_file()`) is confusing
- Extract helpers to maintain consistent abstraction

## Module Boundaries

### Backend Module Dependencies

```
commands/          → analysis/, git/, cache/     (orchestration layer)
analysis/          → models/                      (business logic)
git/               → models/                      (business logic)
cache/             → models/                      (infrastructure)
models/            → (no dependencies)            (pure data)
utils/             → (minimal dependencies)       (utilities)
```

**Rules:**
- Commands orchestrate; don't put business logic in commands
- Analysis/Git modules are independent (don't cross-reference)
- Models are pure data structures (no I/O, no business logic)

### Frontend Module Dependencies

```
features/          → components/, hooks/, store/ (feature modules)
components/        → hooks/, utils/              (UI building blocks)
hooks/             → store/, bindings.ts         (state/IPC access)
store/             → bindings.ts                 (state management)
utils/             → (no dependencies)           (pure functions)
bindings.ts        → (auto-generated)            (IPC layer)
```

**Rules:**
- Features compose components and hooks; don't put UI logic in hooks
- Components call hooks for data; no direct IPC calls from components
- Hooks encapsulate IPC and state access
- Store is the single source of truth for global state

### Public API vs Internal

**Rust:**
- `pub` items in `lib.rs` or `mod.rs` → Public API
- `pub(crate)` → Internal to crate
- No `pub` → Private to module

**TypeScript:**
- Named exports in `index.ts` (barrel file) → Public API
- Direct imports from files → Internal usage (discouraged outside module)

## Code Size Guidelines (KPIs)

### File Size (excluding comments/blank lines)
- **Maximum**: 500 lines per file
- **Ideal**: 200-300 lines
- **Action**: If >500 lines, split into sub-modules

### Function/Method Size
- **Maximum**: 50 lines per function
- **Ideal**: 10-20 lines
- **Action**: Extract helpers or refactor into smaller functions

### Complexity Limits
- **Cyclomatic Complexity**: Max 10 per function
- **Cognitive Complexity**: Max 15 per function (measured by our own tool!)
- **Nesting Depth**: Max 4 levels

### Test Coverage
- **Minimum**: 80% line coverage
- **Critical Paths**: 90% coverage (analysis engine, dead code detection)
- **Action**: Pre-commit hook fails if coverage drops below threshold

### Enforcement
- Pre-commit hooks run:
  - `cargo clippy -- -D warnings` (Rust)
  - `eslint --max-warnings 0` (TypeScript)
  - `cargo test` (unit tests)
  - Custom script to check file LOC (<500)

## Tauri-Specific Patterns

### IPC Command Pattern

**Rust (Backend):**
```rust
// src-tauri/src/commands/analysis.rs
use crate::models::CodeMetric;
use tauri::State;

#[tauri::command]
#[specta::specta] // Generates TypeScript types
pub async fn get_metrics(
    repo_path: String,
    state: State<'_, AppState>,
) -> Result<Vec<CodeMetric>, String> {
    // Implementation
}
```

**TypeScript (Frontend):**
```typescript
// Auto-generated from Rust via tauri-specta
import { commands } from './bindings';

// Type-safe, no string literals!
const metrics = await commands.getMetrics('/path/to/repo');
```

### Event Pattern

**Rust (Backend):**
```rust
use tauri::Manager;

// Emit event
app_handle.emit_all("analysis-progress", ProgressEvent::FileProcessed(path))?;
```

**TypeScript (Frontend):**
```typescript
import { listen } from '@tauri-apps/api/event';

// Subscribe to events
const unlisten = await listen<ProgressEvent>('analysis-progress', (event) => {
  console.log('Progress:', event.payload);
});
```

## Documentation Standards

### Rust Documentation
- **Public items**: MUST have `///` doc comments with examples
- **Module-level**: `//!` at top of `mod.rs` explaining module purpose
- **Complex logic**: Inline `//` comments explaining "why", not "what"

**Example:**
```rust
/// Calculates cognitive complexity of a Rust function.
///
/// Cognitive complexity measures how difficult code is to understand,
/// weighting nested structures more heavily than sequential code.
///
/// # Arguments
/// * `ast` - The abstract syntax tree node representing the function
///
/// # Returns
/// The cognitive complexity score (higher = more complex)
///
/// # Examples
/// ```
/// let ast = parse_function("fn foo() { if x { if y { } } }");
/// let score = calculate_cognitive_complexity(&ast);
/// assert_eq!(score, 3); // 1 + (1+1) for nested ifs
/// ```
pub fn calculate_cognitive_complexity(ast: &AstNode) -> u32 {
    // Implementation
}
```

### TypeScript Documentation
- **Public components/functions**: JSDoc comments
- **Complex hooks**: Document side effects and dependencies
- **Types**: Descriptive names + comments for non-obvious fields

**Example:**
```typescript
/**
 * Hook for managing code analysis operations.
 *
 * Handles IPC communication with Rust backend, caching results,
 * and providing real-time updates via events.
 *
 * @param repoPath - Absolute path to the Git repository
 * @returns Analysis state and control functions
 *
 * @example
 * ```tsx
 * const { metrics, isLoading, refetch } = useAnalysis('/path/to/repo');
 *
 * if (isLoading) return <Spinner />;
 * return <Treemap data={metrics} />;
 * ```
 */
export function useAnalysis(repoPath: string) {
  // Implementation
}
```

### README Requirements
- **Root README.md**: Project overview, installation, quick start
- **Module READMEs** (e.g., `src-tauri/src/analysis/README.md`):
  - Purpose: What this module does
  - Architecture: How it fits in the system
  - Key APIs: Main entry points
  - Invariants: Assumptions that must hold

## Special Files

### .cursorrules (AI Agent Guidelines)
- Located at project root
- Contains rules for AI code generation (architecture constraints, style)
- Updated whenever architectural decisions change

### LLM.md (AI Context File)
- Auto-generated via `just generate-llm-context`
- Contains project structure, public API signatures, invariants
- Regenerated before major AI-assisted refactoring sessions

### Justfile (Task Runner)
- All common tasks defined as recipes
- Replaces npm scripts for cross-cutting concerns
- Examples: `just dev`, `just test`, `just codegen`, `just release`

### .cargo/config.toml
- Linker configuration (mold on Linux, lld on macOS/Windows)
- Environment variables synced with Tauri config
- Build flags for optimization

## Testing Strategy & Structure

### Test Pyramid Architecture (Library-First)

The codebase follows a strict library-first test pyramid to achieve 70% UAT cost reduction:

```
        /\
       /E2E\         1 smoke test (GUI validation only)
      /------\       ~30s runtime
     /Wrapper \      10 tests (IPC + arg parsing)
    /  Tests   \     ~5s runtime
   /------------\
  /   Command   \    50+ tests (orchestration with mocks)
 /    Layer      \   ~2s runtime, 0ms per test startup
/----------------\
/   Contract     \   20+ tests (Rust ↔ TS validation)
/-----------------\  ~1s runtime, compile-time safety
/   Core Library  \ 100+ tests (pure algorithms)
-------------------- <1s runtime, 0 dependencies
```

**Layer Responsibilities:**

1. **Core Library Tests** (`crates/code-viz-core/tests/`):
   - Pure business logic: algorithms, calculations, data structures
   - Zero external dependencies (no I/O, no traits needed)
   - Direct function calls: `assert_eq!(calculate_complexity(ast), 5)`
   - Target: 90% line coverage, <1ms per test

2. **Contract Tests** (`crates/code-viz-tauri/tests/contract_tests.rs`):
   - Specta schema validation (Rust types → TypeScript)
   - Serialization round-trips (ensure data survives IPC)
   - Target: 100% coverage of all `#[specta::specta]` types
   - **Critical**: Prevents wrapper node bugs (empty string → undefined)

3. **Command Layer Tests** (`crates/code-viz-commands/tests/`):
   - Orchestration logic with MockContext, MockFileSystem, MockGit
   - Event emission verification, progress reporting, error handling
   - End-to-end business logic without I/O
   - Target: All command functions, all error paths

4. **Wrapper Tests** (in presentation crates):
   - `code-viz-tauri`: IPC binding correctness (thin layer)
   - `code-viz-cli`: Argument parsing, output formatting (thin layer)
   - Target: Presentation-specific behavior only

5. **E2E Tests** (`tests/e2e/smoke_test.spec.ts`):
   - Single test: Open app → Analyze → Visualize → Drill-down
   - Only validates GUI-specific behavior
   - Target: Minimal (smoke test only)

### Test Execution Speed

| Layer | Count | Runtime | Cost/Run |
|-------|-------|---------|----------|
| Unit | 100+ | <1s | $0 (local) |
| Contract | 20+ | ~2s | $0 (local) |
| CLI Integration | 50+ | ~5s | $0 (local) |
| E2E | 1 | ~30s | Low (minimal) |
| **Total** | **170+** | **<40s** | **Minimal** |

**CI Pipeline**: All tests run in parallel, total CI time <2 minutes (down from 5+ minutes).

## Workspace Crate Benefits

The current workspace structure (`crates/*`) enables:

1. **Shared Core Logic**: `code-viz-core` reused by both CLI and Tauri
2. **Fast Integration Testing**: CLI tests bypass Tauri/WebView overhead
3. **Modular Development**: Each crate independently testable
4. **Clear Boundaries**: Enforced via Cargo dependency graph

## Future Structure Evolution

As the project grows, consider:

1. **Trait-Based DI**: Add `AppContext` trait to `code-viz-core` for pure unit testing
2. **Plugin System**: Add `plugins/` directory for user-extensible metrics
3. **Cloud Backend** (if needed): New crate `code-viz-server/` for remote indexing
4. **Documentation Site**: `docs-site/` with Docusaurus for user-facing docs
