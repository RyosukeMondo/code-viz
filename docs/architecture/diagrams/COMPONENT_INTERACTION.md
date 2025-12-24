# Component Interaction Diagram

## High-Level Component View

```
┌────────────────────────────────────────────────────────────────────┐
│                              FRONTEND                               │
│                         (Browser/Webview)                           │
│                                                                     │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────────────┐  │
│  │   UI Layer   │   │    Store     │   │   Visualization      │  │
│  │              │   │              │   │                      │  │
│  │ Components   │──►│   Zustand    │──►│  ECharts Sunburst   │  │
│  │ - Buttons    │   │              │   │  - Interactive       │  │
│  │ - Controls   │   │ State:       │   │  - Drill-down        │  │
│  │ - Panels     │   │ - TreeData   │   │  - Zoom              │  │
│  │              │   │ - Progress   │   │                      │  │
│  └──────┬───────┘   └──────┬───────┘   └──────────────────────┘  │
│         │                  │                                       │
│         └──────────────────┤                                       │
│                            │                                       │
│                    ┌───────▼────────┐                             │
│                    │   API Client   │                             │
│                    │  (Tauri API)   │                             │
│                    │                │                             │
│                    │ invoke()       │                             │
│                    │ listen()       │                             │
│                    └───────┬────────┘                             │
└────────────────────────────┼───────────────────────────────────────┘
                             │
                             │ IPC Bridge
                             │ (JSON/WebSocket)
                             │
┌────────────────────────────┼───────────────────────────────────────┐
│                            │                                        │
│                     BACKEND (Rust Process)                         │
│                            │                                        │
│                    ┌───────▼────────┐                             │
│                    │  Tauri Runtime │                             │
│                    │                │                             │
│                    │ - Router       │                             │
│                    │ - Serializer   │                             │
│                    │ - Event Bus    │                             │
│                    └───────┬────────┘                             │
│                            │                                       │
│         ┌──────────────────┼──────────────────┐                   │
│         │                  │                  │                   │
│    ┌────▼────┐      ┌──────▼──────┐    ┌─────▼──────┐           │
│    │ Command │      │   Command   │    │  Command   │           │
│    │    A    │      │      B      │    │     C      │           │
│    │         │      │             │    │            │           │
│    │ analyze │      │ dead_code   │    │   export   │           │
│    └────┬────┘      └──────┬──────┘    └─────┬──────┘           │
│         │                  │                  │                   │
│         └──────────────────┼──────────────────┘                   │
│                            │                                       │
│                    ┌───────▼────────┐                             │
│                    │ Command Layer  │                             │
│                    │ (Orchestration)│                             │
│                    │                │                             │
│                    │ - Workflow     │                             │
│                    │ - Progress     │                             │
│                    │ - Events       │                             │
│                    └───────┬────────┘                             │
│                            │                                       │
│         ┌──────────────────┼──────────────────┐                   │
│         │                  │                  │                   │
│    ┌────▼────┐      ┌──────▼──────┐    ┌─────▼──────┐           │
│    │  Core   │      │    Core     │    │    Core    │           │
│    │ Module  │      │   Module    │    │   Module   │           │
│    │         │      │             │    │            │           │
│    │ Parser  │      │  Metrics    │    │   Tree     │           │
│    └────┬────┘      └──────┬──────┘    └─────┬──────┘           │
│         │                  │                  │                   │
│         └──────────────────┼──────────────────┘                   │
│                            │                                       │
│                    ┌───────▼────────┐                             │
│                    │   External     │                             │
│                    │  Dependencies  │                             │
│                    │                │                             │
│                    │ - FileSystem   │                             │
│                    │ - Git (git2)   │                             │
│                    │ - tree-sitter  │                             │
│                    └────────────────┘                             │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

## Request Flow: Repository Analysis

```
┌──────┐
│ User │ Clicks "Select Folder"
└──┬───┘
   │
   ▼
┌─────────────────┐
│ Dialog Button   │
│ Component       │
└──┬──────────────┘
   │
   │ onClick() → open dialog
   ▼
┌─────────────────┐
│ Tauri Dialog    │
│ API             │ Native file picker
└──┬──────────────┘
   │
   │ Returns: /path/to/repo
   ▼
┌─────────────────┐
│ Store Action    │
│ setPath()       │
└──┬──────────────┘
   │
   │ Auto-trigger analysis
   ▼
┌─────────────────┐
│ API Client      │
│ invoke(         │
│   "analyze_     │
│   repository",  │
│   { path }      │
│ )               │
└──┬──────────────┘
   │
   │ IPC: Serialize JSON
   │ { cmd: "analyze_repository", path: "/path/to/repo" }
   ▼
┌─────────────────┐
│ Tauri Command   │
│ Router          │ Deserialize, validate
└──┬──────────────┘
   │
   │ Call Rust function
   ▼
┌─────────────────┐
│ analyze_        │
│ repository()    │
│ command         │
│                 │
│ - Create ctx    │
│ - Create fs     │
│ - Call command  │
│   layer         │
└──┬──────────────┘
   │
   │
   ▼
┌─────────────────┐
│ Command Layer   │
│                 │
│ Step 1: Init    │──► emit("progress", 0.1)
│ Step 2: Scan    │──► emit("progress", 0.3)
│ Step 3: Parse   │──► emit("progress", 0.6)
│ Step 4: Build   │──► emit("progress", 0.9)
│ Step 5: Return  │──► emit("analysis_complete")
└──┬──────────────┘
   │
   │ Return AnalysisResult
   ▼
┌─────────────────┐
│ Tauri Command   │
│ Handler         │ Transform to TreeNode
└──┬──────────────┘
   │
   │ IPC: Serialize JSON
   │ { id: "...", name: "...", value: 1234, children: [...] }
   ▼
┌─────────────────┐
│ API Client      │
│ (Promise        │ Deserialize
│  resolves)      │
└──┬──────────────┘
   │
   │ Update state
   ▼
┌─────────────────┐
│ Store           │
│ setTreeData()   │
└──┬──────────────┘
   │
   │ React re-render
   ▼
┌─────────────────┐
│ Sunburst Chart  │
│ Component       │ ECharts renders visualization
└─────────────────┘
```

## Event Flow: Progress Updates

```
BACKEND                          FRONTEND
───────                          ────────

┌──────────────┐
│ Command      │
│ Layer        │
└──┬───────────┘
   │
   │ ctx.report_progress(0.5, "Parsing...")
   ▼
┌──────────────┐
│ TauriContext │
│              │
│ report_      │
│ progress()   │
└──┬───────────┘
   │
   │ app.emit("progress", { percentage: 0.5, ... })
   ▼
┌──────────────┐
│ Tauri Event  │
│ Bus          │ Broadcast to all listeners
└──┬───────────┘
   │
   │ IPC: Event Stream
   │ { event: "progress", payload: { ... } }
   │
   │                              ┌──────────────┐
   │                              │ Event        │
   └─────────────────────────────►│ Listener     │
                                  │              │
                                  │ listen(      │
                                  │  "progress", │
                                  │  handler     │
                                  │ )            │
                                  └──┬───────────┘
                                     │
                                     │ handler(event)
                                     ▼
                                  ┌──────────────┐
                                  │ Store        │
                                  │ setProgress()│
                                  └──┬───────────┘
                                     │
                                     │ React re-render
                                     ▼
                                  ┌──────────────┐
                                  │ Progress Bar │
                                  │ Component    │
                                  └──────────────┘
```

## Component Dependencies

### Frontend Components

```
App.tsx
  ├── SunburstView (main visualization)
  │   ├── Controls Panel
  │   │   ├── Directory Selector
  │   │   ├── Depth Slider
  │   │   └── Export Button
  │   └── Sunburst Chart (ECharts)
  │
  ├── ProgressBar
  └── ErrorBoundary
```

### Backend Modules

```
main.rs (Tauri entry)
  │
  ├── commands.rs (Tauri command wrappers)
  │   ├── analyze_repository
  │   ├── calculate_dead_code
  │   └── export_analysis
  │
  ├── code-viz-commands (command layer)
  │   ├── analyze.rs
  │   ├── dead_code.rs
  │   └── export.rs
  │
  └── code-viz-core (core logic)
      ├── traits/
      │   ├── app_context.rs
      │   ├── filesystem.rs
      │   └── git.rs
      ├── parser/
      ├── metrics/
      └── tree/
```

## State Management

### Zustand Store Structure

```typescript
interface AnalysisStore {
  // Data
  treeData: TreeNode | null;
  selectedPath: string | null;

  // UI State
  progress: number;
  message: string;
  isAnalyzing: boolean;
  error: string | null;

  // Visualization State
  maxDepth: number;
  currentRoot: TreeNode | null;

  // Actions
  setTreeData: (data: TreeNode) => void;
  setProgress: (progress: number, message: string) => void;
  setMaxDepth: (depth: number) => void;
  resetState: () => void;
}
```

## Error Handling Flow

```
Backend Error
    │
    ├── Core Layer throws Result::Err
    │       ▼
    ├── Command Layer propagates error
    │       ▼
    ├── Tauri Command converts to String
    │       ▼
    └── IPC sends error response
            │
            ▼
Frontend Error Handler
    │
    ├── Promise.catch()
    │       ▼
    ├── Store setError()
    │       ▼
    └── Error UI Component renders
```
