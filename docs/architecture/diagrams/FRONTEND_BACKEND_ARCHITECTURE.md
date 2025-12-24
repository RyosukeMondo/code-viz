# Frontend-Backend Architecture

## Overview

Code-Viz uses a Tauri-based architecture with React frontend and Rust backend, communicating via IPC (Inter-Process Communication).

## Full Stack Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                          FRONTEND (React/TypeScript)                 │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────┐         ┌──────────────────┐                 │
│  │  User Interface  │         │  State Management│                 │
│  │                  │         │    (Zustand)     │                 │
│  │  - App.tsx       │◄────────┤                  │                 │
│  │  - Components/   │         │  - analysisStore │                 │
│  │  - Features/     │         │  - settingsStore │                 │
│  └────────┬─────────┘         └────────┬─────────┘                 │
│           │                            │                            │
│           │    ┌──────────────────────┴──────────┐                 │
│           │    │                                  │                 │
│           ▼    ▼                                  ▼                 │
│  ┌─────────────────────┐              ┌──────────────────┐         │
│  │  Visualization      │              │   API Layer      │         │
│  │  (ECharts)          │              │   (Tauri API)    │         │
│  │                     │              │                  │         │
│  │  - Sunburst Chart   │              │  - invoke()      │         │
│  │  - Interactive UI   │              │  - listen()      │         │
│  └─────────────────────┘              └────────┬─────────┘         │
│                                                 │                   │
└─────────────────────────────────────────────────┼───────────────────┘
                                                  │
                                                  │ IPC Channel
                                                  │ (JSON over WebSocket)
                                                  │
┌─────────────────────────────────────────────────┼───────────────────┐
│                                                 │                   │
│                          BACKEND (Rust/Tauri)   │                   │
├─────────────────────────────────────────────────┴───────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────┐      │
│  │                    Tauri IPC Layer                        │      │
│  │                                                            │      │
│  │  #[tauri::command]                                        │      │
│  │  - analyze_repository(path) -> TreeNode                   │      │
│  │  - calculate_dead_code(path) -> DeadCodeResult           │      │
│  │  - export_analysis(data, format) -> Result               │      │
│  │                                                            │      │
│  │  Event Emitters:                                          │      │
│  │  - emit("progress", { percentage, message })             │      │
│  │  - emit("analysis_complete", { result })                 │      │
│  └──────────────────────┬───────────────────────────────────┘      │
│                         │                                           │
│                         │ Thin Wrappers (11-15 LOC)                │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────┐      │
│  │              Command Layer (code-viz-commands)            │      │
│  │                                                            │      │
│  │  Pure orchestration functions:                            │      │
│  │  - analyze_repository(ctx, fs) -> AnalysisResult         │      │
│  │  - calculate_dead_code(ctx, fs, git) -> DeadCodeResult   │      │
│  │  - export_report(result, ctx, fs) -> Result              │      │
│  │                                                            │      │
│  │  Framework-agnostic, 100% testable                        │      │
│  └──────────────────────┬───────────────────────────────────┘      │
│                         │                                           │
│                         │ Uses Traits                               │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────┐      │
│  │              Core Layer (code-viz-core)                   │      │
│  │                                                            │      │
│  │  Traits:                          Implementations:        │      │
│  │  - AppContext ────────────────► - TauriContext           │      │
│  │  - FileSystem ────────────────► - RealFileSystem         │      │
│  │  - GitProvider ───────────────► - RealGit                │      │
│  │                                                            │      │
│  │  Domain Logic:                    Mocks (for tests):      │      │
│  │  - AST parsing                  - MockContext             │      │
│  │  - Metrics calculation          - MockFileSystem          │      │
│  │  - Tree building                - MockGit                 │      │
│  └──────────────────────┬───────────────────────────────────┘      │
│                         │                                           │
│                         │ Uses                                      │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────┐      │
│  │              External Dependencies                        │      │
│  │                                                            │      │
│  │  - std::fs (File System)                                 │      │
│  │  - git2 (Git Operations)                                 │      │
│  │  - tree-sitter (Code Parsing)                            │      │
│  └──────────────────────────────────────────────────────────┘      │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

## Communication Flow

### 1. User Analysis Request

```
User clicks "Analyze" button
    ↓
React Component (SunburstView.tsx)
    ↓
Zustand Store (analysisStore.ts)
    ↓
Tauri API: invoke("analyze_repository", { path: "/path/to/repo" })
    ↓
[IPC Channel - Serializes to JSON]
    ↓
Tauri Command Handler (commands.rs)
    ↓
Command Layer (analyze_repository function)
    ↓
Core Layer (AST parsing, metrics calculation)
    ↓
[IPC Channel - Serializes result to JSON]
    ↓
React Component receives TreeNode
    ↓
ECharts renders Sunburst visualization
```

### 2. Real-time Progress Updates

```
Backend (Core Layer)
    ↓
AppContext::report_progress(0.5, "Processing files...")
    ↓
TauriContext::report_progress()
    ↓
app.emit("progress", { percentage: 0.5, message: "..." })
    ↓
[IPC Channel - Event Stream]
    ↓
Frontend: listen("progress", (event) => { ... })
    ↓
Zustand Store updates progress state
    ↓
UI renders progress bar
```

## Data Flow Diagram

```
┌─────────────┐
│    User     │
└──────┬──────┘
       │ Interaction
       ▼
┌─────────────────────────────────────────┐
│  React Components                        │
│  ┌──────────────┐    ┌───────────────┐ │
│  │ SunburstView │◄──►│ AnalysisStore │ │
│  └──────────────┘    └───────────────┘ │
└───────────────┬─────────────────────────┘
                │
                │ invoke("analyze_repository")
                ▼
┌─────────────────────────────────────────┐
│  Tauri IPC Bridge                       │
│  - Serialization/Deserialization        │
│  - Command routing                      │
│  - Event emission                       │
└───────────────┬─────────────────────────┘
                │
                │ async fn analyze_repository()
                ▼
┌─────────────────────────────────────────┐
│  Command Layer                          │
│  - Orchestrates workflow                │
│  - Reports progress                     │
│  - Emits events                         │
└───────────────┬─────────────────────────┘
                │
                │ Uses traits (FileSystem, Git)
                ▼
┌─────────────────────────────────────────┐
│  Core Layer                             │
│  - AST parsing (tree-sitter)           │
│  - Metrics calculation                  │
│  - Tree building                        │
└───────────────┬─────────────────────────┘
                │
                │ File I/O, Git operations
                ▼
┌─────────────────────────────────────────┐
│  File System & Git Repository          │
└─────────────────────────────────────────┘
```

## Frontend Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| UI Framework | React 18 | Component-based UI |
| Language | TypeScript | Type-safe development |
| State Management | Zustand | Lightweight state management |
| Visualization | ECharts 5 | Interactive sunburst charts |
| Styling | Tailwind CSS | Utility-first CSS |
| Build Tool | Vite | Fast dev server & bundling |
| Desktop Runtime | Tauri 2 | Native desktop app wrapper |

## Backend Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Language | Rust | Safe, fast systems programming |
| Desktop Framework | Tauri 2 | Bridge between frontend and backend |
| Code Parsing | tree-sitter | AST parsing for multiple languages |
| Git Operations | git2 | Git repository analysis |
| Async Runtime | Tokio | Asynchronous task execution |
| Serialization | serde/serde_json | Data serialization for IPC |

## Key Design Patterns

### 1. Trait-Based Dependency Injection

**Purpose**: Enable testing without I/O and maintain clean architecture

```rust
// Core traits (code-viz-core)
pub trait FileSystem {
    fn read_file(&self, path: &Path) -> Result<String>;
    fn list_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
}

// Production implementation
pub struct RealFileSystem;
impl FileSystem for RealFileSystem {
    fn read_file(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path)
    }
}

// Test mock
pub struct MockFileSystem {
    files: HashMap<PathBuf, String>,
}
```

### 2. Single Source of Truth (SSOT)

**Principle**: All business logic lives in the command layer, shared by:
- Tauri GUI
- CLI binary
- Unit tests
- Integration tests

**Benefit**: No code duplication, consistent behavior across interfaces

### 3. Event-Driven Progress Reporting

**Pattern**: Backend emits progress events, frontend listens and updates UI

```typescript
// Frontend listens
await listen("progress", (event) => {
  setProgress(event.payload.percentage);
  setMessage(event.payload.message);
});

// Backend emits
ctx.report_progress(0.5, "Analyzing files...");
```

## Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| Repository analysis | 2-5s | Depends on repo size |
| UI rendering | <100ms | ECharts optimized |
| IPC round-trip | <10ms | Local WebSocket |
| Unit tests | <100ms | No I/O, pure logic |
| Full test suite | ~6s | 170 tests |

## Testing Strategy

### Frontend Tests
- **Unit**: Vitest + React Testing Library
- **E2E**: Playwright
- **Coverage**: 80%+ target

### Backend Tests
- **Unit**: Rust `#[test]` with mocks
- **Integration**: Real filesystem tests
- **Coverage**: 80%+ target

### IPC Tests
- End-to-end tests calling Tauri commands
- Validates serialization/deserialization
- Ensures frontend-backend contract

## Security Considerations

1. **IPC Validation**: All commands validate input before execution
2. **Path Traversal**: Sanitize file paths to prevent directory traversal
3. **Resource Limits**: Prevent DoS by limiting analysis scope
4. **No Remote Code**: No eval(), no dynamic code execution
5. **Sandbox**: Tauri provides OS-level sandboxing

## Future Extensibility

The architecture supports easy addition of:
- **Web version**: Replace Tauri with HTTP API server
- **CLI version**: Already implemented
- **gRPC API**: Add gRPC server wrapping command layer
- **Plugin system**: Extend core with custom analyzers

All would share the same command layer and core logic.
