# Technology Stack

## Project Type

**Hybrid Desktop Application**: Cross-platform code analysis and visualization tool built with Tauri v2, combining a high-performance Rust backend for static analysis with a modern React frontend for interactive 3D/2D visualizations. Operates locally on developer machines with optional CLI interface for CI/CD integration.

## Core Technologies

### Primary Languages

- **Rust 1.75+** (Backend/Analysis Engine)
  - Compiler: rustc with stable channel
  - Memory safety without garbage collection overhead
  - Zero-cost abstractions for C/C++ library bindings
  - Package Manager: Cargo
  - Build Tools: cargo-watch, cargo-nextest

- **TypeScript 5.0+** (Frontend/Visualization)
  - Runtime: Node.js 20+ (dev), Browser WebView (prod)
  - Build Tool: Vite 5+ with HMR
  - Package Manager: npm/pnpm

### Application Framework

- **Tauri v2**: Rust-based application shell
  - Lightweight alternative to Electron (~3MB vs 100MB+ bundle)
  - Security-first IPC with type-safe command system
  - Native system integration (file watchers, OS APIs)
  - WebView2 (Windows), WKWebView (macOS), WebKitGTK (Linux)

### Key Dependencies/Libraries

#### Rust Backend (Analysis Core)

- **tree-sitter 0.20+**: Incremental multi-language parser
  - Generates concrete syntax trees (CST)
  - Sub-second re-parsing on file edits
  - Language grammars: TypeScript, JavaScript, Rust, Python, Go

- **stack-graphs 0.13+**: Cross-file semantic analysis
  - Name resolution and symbol binding
  - Dead code detection via reachability analysis
  - Incremental graph updates on file changes

- **git2-rs 0.18+**: Direct Git repository access
  - libgit2 bindings for high-performance history traversal
  - Commit graph analysis without subprocess overhead
  - Diff calculation for churn metrics

- **rayon 1.8+**: Data parallelism
  - Work-stealing thread pool for file processing
  - Parallel iterator abstractions

- **notify 6.1+**: File system watcher
  - Cross-platform inotify/FSEvents/ReadDirectoryChangesW
  - Real-time codebase monitoring

- **serde 1.0+**: Serialization framework
  - JSON/Binary serialization for IPC
  - Zero-copy deserialization with serde_json

- **tauri-specta 2.0+**: Contract-driven IPC
  - Automatic TypeScript type generation from Rust
  - Compile-time type safety across language boundaries

- **tracing 0.1+**: Structured logging and telemetry
  - Async-aware instrumentation
  - OpenTelemetry integration (feature-gated)

#### React Frontend (Visualization UI)

- **React 18+**: Component-based UI framework
  - Concurrent rendering for smooth animations
  - Suspense for lazy-loaded visualizations

- **React Three Fiber (R3F) 8.15+**: WebGL 3D visualization
  - Declarative Three.js wrapper
  - React hooks for animation loops
  - Sub-packages: @react-three/drei, @react-three/postprocessing

- **Three.js 0.160+**: Low-level 3D graphics
  - InstancedMesh for rendering 100K+ objects
  - WebGL shaders for color mapping
  - BVH acceleration (three-mesh-bvh) for raycasting

- **Apache ECharts 5.5+**: 2D data visualization
  - Canvas-based rendering (non-DOM)
  - Zoomable treemap with drill-down
  - Git history timeline charts

- **Zustand 4.4+**: Lightweight state management
  - No Context API re-render issues
  - Selector-based subscriptions
  - Middleware for persistence

- **Tailwind CSS 3.4+**: Utility-first styling
  - JIT compilation for minimal bundle size
  - Dark mode support

### Application Architecture

**Layered Hybrid Architecture** with strict separation of concerns:

```
┌─────────────────────────────────────────────┐
│         Frontend (React + WebGL)            │
│  ┌────────────┐  ┌──────────────────────┐   │
│  │  UI Layer  │  │  Visualization Layer │   │
│  │  (Tailwind)│  │  (R3F, ECharts)      │   │
│  └─────┬──────┘  └──────────┬───────────┘   │
│        │                    │               │
│        └────────┬───────────┘               │
│                 │ Type-safe IPC             │
│         ┌───────▼──────────┐                │
│         │  Tauri Commands  │                │
│         │  (Generated TS)  │                │
└─────────┴──────────────────┴────────────────┘
                 │ IPC Bridge
┌─────────────────▼───────────────────────────┐
│         Backend (Rust Core)                 │
│  ┌──────────────────────────────────────┐   │
│  │     Command Handlers (Tauri)         │   │
│  │     (#[tauri::command] functions)    │   │
│  └─────────────────┬────────────────────┘   │
│                    │                        │
│  ┌─────────────────▼────────────────────┐   │
│  │      Analysis Engine                 │   │
│  │  ┌─────────┐  ┌──────────────────┐   │   │
│  │  │Tree-    │  │ Stack-graphs     │   │   │
│  │  │sitter   │→ │ (Name Resolution)│   │   │
│  │  │Parsers  │  └──────────────────┘   │   │
│  │  └─────────┘                         │   │
│  │  ┌─────────────────────────────────┐ │   │
│  │  │ Metrics Calculator (Rayon)      │ │   │
│  │  │ - Cognitive Complexity          │ │   │
│  │  │ - LOC, Churn, AI Bloat Index    │ │   │
│  │  └─────────────────────────────────┘ │   │
│  └──────────────────────────────────────┘   │
│                    │                        │
│  ┌─────────────────▼────────────────────┐   │
│  │  Git History Manager (git2-rs)      │   │
│  └──────────────────────────────────────┘   │
│                    │                        │
│  ┌─────────────────▼────────────────────┐   │
│  │  Cache Layer (sled / SQLite)        │   │
│  │  - Parsed ASTs, Metrics Snapshots   │   │
│  └──────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

**Key Architectural Principles:**

1. **SSOT (Single Source of Truth)**: Rust backend defines all data models; TypeScript types auto-generated via tauri-specta
2. **Contract-Driven**: IPC contracts enforced at compile time, not runtime
3. **Incremental Processing**: Tree-sitter and stack-graphs minimize re-computation on file changes
4. **Parallel Execution**: Rayon parallelizes file analysis across CPU cores
5. **Lazy Visualization**: 3D scenes use LOD (Level of Detail) and culling; 2D charts lazy-load deep nodes

### Data Storage

- **Primary Storage**: Local file system
  - User repositories analyzed in-place (no copying)
  - Analysis cache: `.code-viz/cache/` directory (gitignored)

- **Caching**: Hybrid approach
  - **Hot Cache (In-Memory)**: Recently analyzed files in Rust HashMap/BTreeMap
  - **Cold Cache (Disk)**: sled embedded database or SQLite
    - Key: `(repo_path, commit_hash, file_path)`
    - Value: Compressed bincode-serialized metrics

- **Data Formats**:
  - **IPC**: JSON (serde_json) for human-readable debugging; MessagePack for large datasets
  - **Disk Cache**: bincode (binary) with optional zstd compression
  - **Export**: JSON, CSV, SVG/PNG (visualization snapshots)

### External Integrations

- **APIs**: None (fully offline operation)
- **Protocols**: File system APIs, Git object database (local)
- **Optional Features** (feature-gated):
  - **GitHub API** (future): Fetch CI build metrics, PR comments
  - **OpenTelemetry**: Export tracing spans to Jaeger/Honeycomb

## Development Environment

### Build & Development Tools

#### Rust Compilation Acceleration (2025 Standard)

**Linker Optimization:**
- **Primary**: Mold (Linux) - 10-50x faster than GNU ld
- **macOS**: System linker (Apple's optimized ld) or lld fallback
- **Windows**: lld (LLVM linker)
- Configuration: `.cargo/config.toml`
  ```toml
  [target.x86_64-unknown-linux-gnu]
  linker = "clang"
  rustflags = ["-C", "link-arg=-fuse-ld=mold"]
  ```

**Compilation Cache:**
- **sccache**: Shared compilation cache across projects
  - Environment: `RUSTC_WRAPPER=sccache`
  - Backend: Local disk (~10GB cache) or S3 for CI
  - Speedup: 35-60% on incremental builds

**Environment Synchronization:**
- Problem: rust-analyzer and `tauri dev` use different `MACOSX_DEPLOYMENT_TARGET`, causing double-builds
- Solution: Sync VSCode settings with tauri.conf.json:
  ```json
  // .vscode/settings.json
  {
    "rust-analyzer.cargo.extraEnv": {
      "MACOSX_DEPLOYMENT_TARGET": "10.13"
    }
  }
  ```

#### Task Runner: Just (not Make)

- **Justfile**: Modern command runner with cross-platform compatibility
  - No tab/space issues
  - Native Windows PowerShell support
  - Better error messages than Make

**Example Justfile Recipes:**
```just
# Generate TypeScript bindings from Rust types
codegen:
    cargo test --package tauri-plugin --lib -- --nocapture specta

# Development with hot reload
dev:
    just codegen
    tauri dev --features dev-tools

# Production build with optimizations
release:
    cargo build --release --target x86_64-apple-darwin
    npm run build
    tauri build

# Run tests in parallel
test:
    cargo nextest run --all-features
```

### Terminal Workspace: Zellij

- **Zellij**: Rust-based terminal multiplexer (tmux alternative)
  - KDL layout files committed to repo
  - Standard 3-pane layout:
    - Left: File navigator (strider plugin)
    - Center: Editor / main terminal
    - Bottom: Split panes for `cargo run` and `npm run dev`
    - Right (optional): TUI logger for structured logs

**Layout File** (`.zellij/layout.kdl`):
```kdl
layout {
    pane split_direction="vertical" {
        pane size="20%" { command "strider"; }
        pane split_direction="horizontal" {
            pane size="70%"
            pane split_direction="vertical" {
                pane { command "cargo"; args "watch" "-x" "run"; }
                pane { command "npm"; args "run" "dev"; }
            }
        }
    }
}
```

### Code Quality Tools

- **Static Analysis**:
  - **clippy**: Rust linter with `clippy::pedantic` enabled
  - **cargo-audit**: Security vulnerability scanner
  - **ESLint + typescript-eslint**: TypeScript linting

- **Formatting**:
  - **rustfmt**: Rust (edition 2021, 100-char line width)
  - **Prettier**: TypeScript/JSON/Markdown

- **Testing Frameworks**:
  - **Rust**: cargo-nextest (faster test runner), insta (snapshot testing)
  - **React**: Vitest (unit), Playwright (E2E for Tauri WebView)
  - **Storybook**: Component visual testing

- **Documentation**:
  - **rustdoc**: API docs with examples
  - **Docusaurus**: User-facing documentation site

### Version Control & Collaboration

- **VCS**: Git
- **Branching Strategy**: GitHub Flow (main + feature branches)
- **Pre-commit Hooks** (husky + cargo-husky):
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings`
  - `npm run lint`
  - `cargo test --quiet`
- **Code Review**: GitHub PRs with required CI checks

### Dashboard Development

- **Live Reload**:
  - Frontend: Vite HMR (Hot Module Replacement)
  - Backend: cargo-watch auto-restarts on Rust changes
  - Tauri: WebView survives backend restarts via state persistence

- **Port Management**: Vite dev server on dynamic port (5173+), configurable via env

- **Logging**: tui-logger in Zellij pane for real-time structured log filtering

## Deployment & Distribution

- **Target Platforms**:
  - macOS 10.13+ (x64, ARM64)
  - Windows 10+ (x64)
  - Linux (x64, ARM64) - AppImage + Debian package

- **Distribution Method**:
  - **GitHub Releases**: Signed binaries + auto-update manifests
  - **Homebrew** (macOS): `brew install code-viz`
  - **Future**: Snap (Linux), Chocolatey (Windows)

- **Installation Requirements**:
  - No external dependencies (Rust and Node.js embedded in binary)
  - Git installed (for repository analysis)

- **Update Mechanism**: Tauri's built-in updater with signature verification

## Technical Requirements & Constraints

### Performance Requirements

- **Startup Time**: <2 seconds from launch to UI ready
- **Analysis Latency**:
  - Incremental re-analysis: <100ms for single file change
  - Full repository scan: <30 seconds for 100K files (parallelized)
- **Memory Usage**: <500MB baseline, +1MB per 1K files analyzed
- **Frame Rate**: 60 FPS for 3D visualization with 50K objects (via InstancedMesh)

### Compatibility Requirements

- **Rust**: MSRV (Minimum Supported Rust Version) 1.75+
- **Node.js**: 20+ for development
- **Git**: 2.30+ (for advanced diff algorithms)
- **GPU**: WebGL 2.0 support (fallback to 2D-only mode if unavailable)

### Security & Compliance

- **Local-Only Processing**: No data leaves the machine
- **Tauri Security**:
  - IPC allowlist (only whitelisted commands callable)
  - CSP (Content Security Policy) enforced
  - No eval() or inline scripts in WebView
- **Threat Model**: User-controlled input (local repositories); no network attack surface

### Scalability & Reliability

- **Expected Load**: Single-user desktop app; 1-10 repositories monitored simultaneously
- **Graceful Degradation**:
  - If Git history too large (>100K commits), sample every Nth commit
  - If file count exceeds memory, use disk-backed graph (rocksdb/sled)
- **Crash Recovery**: Analysis progress checkpointed every 10 seconds; resume on restart

## Technical Decisions & Rationale

### Decision Log

1. **Rust for Analysis Engine (vs Python/Node.js)**
   - **Rationale**: Tree-sitter and stack-graphs are Rust-native; Python bindings add FFI overhead. Memory safety critical for parsing untrusted codebases.
   - **Trade-off**: Longer compile times (mitigated by sccache/mold).

2. **Tauri v2 over Electron**
   - **Rationale**: 97% smaller binary size (3MB vs 100MB), better security model, native performance.
   - **Trade-off**: Smaller ecosystem than Electron; requires Rust knowledge.

3. **React Three Fiber over Unity/Unreal WebGL Export**
   - **Rationale**: Lighter runtime, tighter integration with React state, no licensing restrictions.
   - **Trade-off**: Manual optimization (InstancedMesh, BVH) required for performance.

4. **Stack-graphs over LSP (Language Server Protocol)**
   - **Rationale**: LSP designed for editor support, not batch analysis. Stack-graphs provide incremental, exportable semantic data.
   - **Trade-off**: Language coverage limited (need to write grammars).

5. **Type-Safe IPC (tauri-specta) over JSON Schema**
   - **Rationale**: Compile-time enforcement prevents runtime IPC errors. DX improvement (autocomplete in TS).
   - **Trade-off**: Requires codegen step (`just codegen`) after Rust changes.

6. **Just over Make**
   - **Rationale**: Cross-platform (Windows native support), better syntax (no tab/space hell).
   - **Trade-off**: Team must install Just (but it's Rust-based, one binary).

7. **Apache ECharts over D3.js**
   - **Rationale**: Canvas rendering 10x faster than SVG for large datasets; treemap built-in.
   - **Trade-off**: Less flexible than D3; harder to customize animations.

## Known Limitations

- **Language Support**: Tree-sitter grammars required for each language; currently supports TypeScript/JavaScript/Rust/Python. Go/Java/C++ require additional grammar integration.

- **Monorepo Performance**: Workspaces with shared dependencies (npm/yarn) analyzed separately; no cross-workspace dead code detection yet.

- **Dynamic Language Challenges**: Python/JavaScript's dynamic nature limits stack-graphs accuracy (e.g., `eval()`, dynamic imports).

- **3D Rendering on Integrated GPUs**: Intel UHD graphics may struggle with >100K objects; fallback to 2D-only mode recommended.

- **Git LFS**: Large binary files in Git history slow down analysis; recommend excluding via `.code-viz-ignore`.

## AI-Native Development Workflows

### .cursorrules Protocol

**Purpose**: Enforce architectural constraints and coding standards for AI agents (Cursor, GitHub Copilot, Claude).

**Location**: `.cursorrules` (project root)

**Key Directives**:
```markdown
# Code-Viz Development Rules

## Technology Stack
- ALWAYS use tauri-specta for IPC. Raw `invoke()` calls are PROHIBITED.
- After modifying Rust structs, RUN `just codegen` to regenerate TypeScript types.

## Rust Standards
- Follow clippy::pedantic warnings.
- Use thiserror for library errors, anyhow for application errors.
- ALL public functions must have doc comments with examples.

## React Standards
- Prefer functional components and hooks over classes.
- Use Zustand for global state; avoid Context API for performance-critical data.
- Tailwind CSS for styling; no inline styles.

## Testing
- Rust: Unit tests for all public functions. Use insta for snapshot tests.
- React: Storybook stories for all reusable components.

## Before Making Changes
- READ `LLM.md` for project structure and API contracts.
- CHECK existing patterns in similar modules before inventing new abstractions.
```

### LLM.md Context Optimization

**Purpose**: Provide AI agents with project overview without full codebase traversal.

**Generation**:
```bash
# Automated via Just recipe
just generate-llm-context
# → Runs rustdoc-json + custom script to extract signatures
```

**Contents**:
- Public API signatures (no implementation)
- Module dependency graph
- Architecture invariants (e.g., "Never call git2 from UI thread")
- Common pitfalls (e.g., "Tree-sitter nodes invalidated after edit")

**Usage**: Include in AI prompts:
```
Context: @LLM.md
Task: Add a new metric for "cyclomatic complexity" to the analysis engine.
```

### Documentation as Code

- Each crate (e.g., `crates/analysis/`) has `README.md` with:
  - **Purpose**: What this crate does (for AI understanding)
  - **Public API**: Entry points and traits
  - **Invariants**: Assumptions that must hold (e.g., "Parsers are !Send")

- AI agents navigate high-level docs → module docs → function docs (hierarchical context loading).

## Future Technical Enhancements

- **WASM Plugin System**: Allow users to write custom metrics in Rust/C compiled to WASM, sandboxed execution.
- **GPU Compute Shaders**: Offload complexity calculations to GPU via WebGPU for real-time updates.
- **Remote Indexing**: Optional cloud-based indexing for teams (with end-to-end encryption).
- **ML-Based Dead Code Prediction**: Train model on "code that eventually got deleted" to predict future dead code.
