# Tasks Document: Interactive Treemap Visualization MVP

## Phase 1: Project Setup & Infrastructure

- [x] 1.1. Initialize Tauri project structure
  - Files: `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/src/main.rs`
  - Create Tauri v2 application shell with dependencies on `code-viz-core`
  - Configure build system and application metadata
  - Purpose: Establish Tauri desktop application foundation
  - _Leverage: Existing `Cargo.toml` workspace configuration, `code-viz-core` crate_
  - _Requirements: 5_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust DevOps Engineer specializing in Tauri and workspace configuration | Task: Initialize Tauri v2 project structure with proper dependencies on existing code-viz-core crate following requirement 5, configure application shell and build system | Restrictions: Do not modify code-viz-core, maintain workspace compatibility with existing CLI crate, use Tauri v2 stable APIs | Success: Tauri app compiles successfully, code-viz-core dependency resolves, tauri.conf.json is properly configured, app launches with empty window. After completion, log implementation with log-implementation tool documenting the Tauri project structure created. Mark task as complete in tasks.md by changing [ ] to [x]._

- [x] 1.2. Set up React + Vite frontend
  - Files: `package.json`, `vite.config.ts`, `tsconfig.json`, `src/main.tsx`, `src/App.tsx`
  - Initialize React 18 with TypeScript and Vite 5
  - Configure Tailwind CSS 3.4 and path aliases
  - Purpose: Establish frontend build pipeline
  - _Leverage: Tauri's recommended Vite configuration_
  - _Requirements: 1, 5_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Frontend Build Engineer with expertise in Vite, React, and TypeScript | Task: Initialize React 18 + Vite 5 project with TypeScript following requirement 1 and 5, configure Tailwind CSS 3.4, set up path aliases (@/ for src/) | Restrictions: Must use Vite 5+ for Tauri compatibility, configure for Tauri WebView target, do not include unnecessary dependencies | Success: Frontend compiles successfully, hot reload works, Tailwind classes applied, TypeScript strict mode enabled, path aliases resolve. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

- [x] 1.3. Add ECharts and Zustand dependencies
  - Files: `package.json` (update)
  - Install Apache ECharts 5.5+, Zustand 4.4+, and required types
  - Configure ECharts for treemap-only imports (tree-shaking)
  - Purpose: Add core visualization and state management libraries
  - _Leverage: None (new dependencies)_
  - _Requirements: 1, 3_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Frontend Dependency Manager with expertise in npm and bundle optimization | Task: Add ECharts 5.5+ (treemap module only), Zustand 4.4+, and TypeScript definitions following requirements 1 and 3, configure for optimal tree-shaking | Restrictions: Only import treemap-related ECharts modules to minimize bundle size, ensure type definitions are installed, use exact versions for reproducibility | Success: Dependencies install successfully, ECharts treemap is importable, Zustand types work, bundle size <200KB for ECharts. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

## Phase 2: Backend Wrapper (Tauri Commands)

- [x] 2.1. Create code-viz-tauri crate structure
  - Files: `crates/code-viz-tauri/Cargo.toml`, `crates/code-viz-tauri/src/lib.rs`
  - Set up new crate with dependencies on `code-viz-core`, `tauri`, `serde`, `tauri-specta`
  - Export public modules for commands and models
  - Purpose: Create Tauri backend wrapper library
  - _Leverage: Existing `code-viz-core` crate APIs_
  - _Requirements: 5, 7_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Library Developer with expertise in crate organization and Tauri | Task: Create code-viz-tauri crate with proper dependencies on code-viz-core, tauri, serde, tauri-specta following requirements 5 and 7, set up module structure | Restrictions: Do not duplicate code-viz-core logic, ensure tauri-specta version compatibility, maintain workspace structure | Success: Crate compiles independently, exports lib.rs with mod declarations, dependencies resolve, workspace includes new crate. After completion, log implementation with log-implementation tool documenting crate structure and public API. Mark task as complete in tasks.md._

- [x] 2.2. Implement TreeNode model for visualization
  - Files: `crates/code-viz-tauri/src/models.rs`
  - Create `TreeNode` struct extending `FileMetrics` with hierarchical structure
  - Add serde and specta derives for IPC serialization
  - Purpose: Define visualization-specific data model
  - _Leverage: Existing `code-viz-core::models::FileMetrics`_
  - _Requirements: 1, 2, 5, 7_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Data Modeling Expert with expertise in serde and type-safe serialization | Task: Create TreeNode struct for hierarchical visualization following requirements 1, 2, 5, and 7, extending FileMetrics with children field and adding complexity placeholder (loc/10) | Restrictions: Must derive Serialize, Deserialize, and specta::Type, maintain compatibility with FileMetrics, do not modify code-viz-core models | Success: TreeNode compiles with correct derives, supports hierarchical structure, auto-generates TypeScript types via specta, includes id, name, path, loc, complexity, type, children, lastModified fields. After completion, log implementation with log-implementation tool documenting the TreeNode model structure. Mark task as complete in tasks.md._

- [x] 2.3. Create flat-to-hierarchy transformation utility
  - Files: `crates/code-viz-tauri/src/transform.rs`
  - Implement function to convert `Vec<FileMetrics>` to hierarchical `TreeNode`
  - Build directory tree structure from flat file paths
  - Purpose: Transform core library output to visualization format
  - _Leverage: `code-viz-core::models::FileMetrics`, `std::path::PathBuf`_
  - _Requirements: 1, 5_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Algorithm Developer with expertise in tree data structures and path manipulation | Task: Implement flat_to_hierarchy function converting Vec<FileMetrics> to TreeNode tree following requirements 1 and 5, building directory structure from file paths | Restrictions: Must handle edge cases (empty paths, single file), maintain performance for 100K files, do not modify input data | Success: Function correctly builds hierarchical tree, handles all path edge cases, unit tests pass for various file structures, complexity is O(n) where n is file count. After completion, log implementation with log-implementation tool documenting the transformation algorithm. Mark task as complete in tasks.md._

- [x] 2.4. Implement analyze_repository Tauri command
  - Files: `crates/code-viz-tauri/src/commands.rs`
  - Create `analyze_repository` function calling `code_viz_core::analyze()`
  - Transform `AnalysisResult` to hierarchical `TreeNode` using transform utility
  - Add `#[tauri::command]` and `#[specta::specta]` attributes
  - Purpose: Expose analysis engine to frontend via IPC
  - _Leverage: `code-viz-core::analyze`, `code-viz-tauri::transform`_
  - _Requirements: 5, 7_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Backend Developer with expertise in Tauri IPC and async Rust | Task: Implement analyze_repository Tauri command following requirements 5 and 7, calling code-viz-core::analyze and transforming result to TreeNode hierarchy | Restrictions: Must use async/await properly, handle errors with Result<>, add proper tracing logs, do not bypass code-viz-core | Success: Command compiles with correct attributes, calls core analyze(), transforms flat to hierarchy, returns TreeNode via IPC, error handling works, TypeScript types auto-generated. After completion, log implementation with log-implementation tool documenting the command interface and data flow. Mark task as complete in tasks.md._

- [x] 2.5. Wire Tauri commands into src-tauri/main.rs
  - Files: `src-tauri/src/main.rs`
  - Register `analyze_repository` command in Tauri builder
  - Generate TypeScript bindings via specta
  - Purpose: Enable frontend to call backend commands
  - _Leverage: `code-viz-tauri::commands`_
  - _Requirements: 5, 7_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Integration Engineer with expertise in Tauri application setup | Task: Register analyze_repository command in Tauri app builder following requirements 5 and 7, configure specta to generate TypeScript bindings in src/types/bindings.ts | Restrictions: Must follow Tauri v2 builder pattern, ensure command is accessible from frontend, generate bindings on build | Success: Tauri app starts successfully, command is registered and callable, TypeScript bindings generated in src/types/bindings.ts, IPC communication works. After completion, log implementation with log-implementation tool documenting the Tauri command registration. Mark task as complete in tasks.md._

## Phase 3: Frontend Foundation

- [x] 3.1. Create TypeScript type definitions
  - Files: `src/types/index.ts`, `src/types/bindings.ts` (auto-generated)
  - Import auto-generated types from Tauri specta
  - Add supplementary types for component props and state
  - Purpose: Establish type safety across frontend
  - _Leverage: Auto-generated bindings from `code-viz-tauri`_
  - _Requirements: 1, 5, 7_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: TypeScript Type System Expert with expertise in type definitions and imports | Task: Create type definitions following requirements 1, 5, and 7, importing auto-generated Tauri bindings and adding component/state types | Restrictions: Do not duplicate auto-generated types, use TypeScript 5+ features, ensure strict mode compatibility | Success: All types compile successfully, bindings are importable, component prop types defined, no any types used. After completion, log implementation with log-implementation tool documenting the type system structure. Mark task as complete in tasks.md._

- [x] 3.2. Implement color mapping utility
  - Files: `src/utils/colors.ts`
  - Create `complexityToColor` function mapping score 0-100 to green→yellow→red gradient
  - Implement `getComplexityGradient` returning array of color stops
  - Purpose: Provide consistent color mapping for complexity visualization
  - _Leverage: None (new utility)_
  - _Requirements: 2_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Frontend Visualization Developer with expertise in color theory and data visualization | Task: Implement color mapping utilities following requirement 2, creating smooth green-yellow-red gradient for complexity scores 0-100 with WCAG AA compliance | Restrictions: Must use accessible colors (WCAG AA contrast), handle edge cases (NaN, negative), return hex color codes | Success: Colors meet WCAG AA standards, gradient is visually smooth, edge cases handled, unit tests pass for all score ranges. After completion, log implementation with log-implementation tool documenting the color mapping function. Mark task as complete in tasks.md._

- [x] 3.3. Implement formatting utilities
  - Files: `src/utils/formatting.ts`
  - Create functions: `formatNumber` (LOC with commas), `formatPath` (truncate), `formatBytes`, `formatDate`
  - Purpose: Provide consistent data formatting across UI
  - _Leverage: None (new utilities)_
  - _Requirements: 1, 3, 4_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Frontend Utility Developer with expertise in internationalization and formatting | Task: Implement formatting utilities following requirements 1, 3, and 4, creating functions for number, path, bytes, and date formatting | Restrictions: Must handle null/undefined gracefully, use Intl API for numbers where possible, make path truncation configurable | Success: All formatters handle edge cases, output is user-friendly, unit tests pass for various inputs. After completion, log implementation with log-implementation tool documenting the formatting utilities. Mark task as complete in tasks.md._

- [x] 3.4. Implement tree transformation utility
  - Files: `src/utils/treeTransform.ts`
  - Create `treeNodeToECharts` function converting `TreeNode` to ECharts treemap data format
  - Implement `filterByPath` function for drill-down filtering
  - Purpose: Transform backend data to ECharts-compatible format
  - _Leverage: `TreeNode` type from bindings_
  - _Requirements: 1, 3_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Frontend Data Transformation Expert with expertise in ECharts and tree structures | Task: Implement tree transformation utilities following requirements 1 and 3, converting TreeNode to ECharts format with { name, value, complexity, path, type, children } and implementing path-based filtering | Restrictions: Must preserve tree structure, handle empty children arrays, maintain performance for large trees | Success: Transformation produces valid ECharts data, filtering works correctly, unit tests pass for various tree structures. After completion, log implementation with log-implementation tool documenting the transformation utilities. Mark task as complete in tasks.md._

- [x] 3.5. Create Zustand store for analysis state
  - Files: `src/store/analysisStore.ts`
  - Implement store with: metrics (TreeNode[]), drillDownPath (string[]), selectedFile (TreeNode | null), loading, error states
  - Add actions: setMetrics, setDrillDownPath, setSelectedFile, reset
  - Purpose: Manage global analysis state
  - _Leverage: Zustand 4.4+, `TreeNode` type_
  - _Requirements: 1, 3_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React State Management Expert with expertise in Zustand and immutable patterns | Task: Create Zustand store for analysis state following requirements 1 and 3, managing metrics, drill-down path, selected file, and loading/error states | Restrictions: Must use Zustand's immer middleware for immutability, follow Zustand best practices, avoid unnecessary re-renders | Success: Store compiles successfully, state updates trigger re-renders correctly, actions are type-safe, store is testable. After completion, log implementation with log-implementation tool documenting the store structure and actions. Mark task as complete in tasks.md._

- [x] 3.6. Create useTauriCommand hook
  - Files: `src/hooks/useTauriCommand.ts`
  - Implement generic hook wrapping Tauri `invoke` with loading/error handling
  - Add TypeScript generics for command name and return type
  - Purpose: Provide type-safe Tauri command invocation
  - _Leverage: Tauri bindings from `@tauri-apps/api`_
  - _Requirements: 5, 7_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React Hooks Developer with expertise in async patterns and TypeScript generics | Task: Create useTauriCommand hook following requirements 5 and 7, wrapping Tauri invoke with type-safe generics, loading/error states, and automatic cleanup | Restrictions: Must use React hooks properly (no violations of rules), handle cleanup on unmount, support TypeScript generics | Success: Hook is type-safe with generics, loading/error states work, cleanup prevents memory leaks, usable with all Tauri commands. After completion, log implementation with log-implementation tool documenting the hook interface. Mark task as complete in tasks.md._

- [x] 3.7. Create useAnalysis hook
  - Files: `src/hooks/useAnalysis.ts`
  - Implement hook calling `analyze_repository` command and updating store
  - Add refetch function and loading/error state management
  - Purpose: Provide high-level analysis execution interface
  - _Leverage: `useTauriCommand`, `analysisStore`_
  - _Requirements: 5, 7_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React Developer with expertise in custom hooks and state synchronization | Task: Create useAnalysis hook following requirements 5 and 7, calling analyze_repository command via useTauriCommand and syncing with analysisStore | Restrictions: Must use useTauriCommand hook, sync with Zustand store correctly, handle concurrent requests gracefully | Success: Hook triggers analysis successfully, updates store on completion, error handling works, refetch is callable, loading state accurate. After completion, log implementation with log-implementation tool documenting the hook usage. Mark task as complete in tasks.md._

## Phase 4: Core Components

- [x] 4.1. Create Breadcrumb component
  - Files: `src/components/common/Breadcrumb.tsx`
  - Implement breadcrumb showing drill-down path with click handlers
  - Add home button for navigating to root
  - Purpose: Provide navigation context and controls
  - _Leverage: Tailwind CSS, `formatPath` utility_
  - _Requirements: 3_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React Component Developer with expertise in navigation UI and Tailwind CSS | Task: Create Breadcrumb component following requirement 3, displaying drill-down path with clickable segments and home button | Restrictions: Must use Tailwind for styling, keep component pure (no side effects), support keyboard navigation | Success: Breadcrumb displays path correctly, click handlers fire with correct indices, home button works, keyboard accessible, styled with Tailwind. After completion, log implementation with log-implementation tool documenting the component props and usage. Mark task as complete in tasks.md._

- [x] 4.2. Create DetailPanel component
  - Files: `src/components/common/DetailPanel.tsx`
  - Implement panel showing file metadata (path, LOC, complexity, last modified)
  - Add close button and expandable sections
  - Purpose: Display detailed file information
  - _Leverage: Tailwind CSS, formatting utilities_
  - _Requirements: 3_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React Component Developer with expertise in panel UI and data display | Task: Create DetailPanel component following requirement 3, displaying file metadata with formatted values and close button | Restrictions: Must use Tailwind for styling, handle null file gracefully, support keyboard close (Escape) | Success: Panel displays all file metadata correctly, formatting utilities used, close button works, Escape key closes panel, animations smooth. After completion, log implementation with log-implementation tool documenting the component interface. Mark task as complete in tasks.md._

- [x] 4.3. Create Treemap component (core visualization)
  - Files: `src/components/visualizations/Treemap.tsx`
  - Implement ECharts treemap with click/hover handlers
  - Add color mapping by complexity score
  - Implement smooth animations for drill-down transitions
  - Purpose: Provide main treemap visualization
  - _Leverage: ECharts treemap, `treeNodeToECharts`, `complexityToColor`, `analysisStore`_
  - _Requirements: 1, 2, 3, 4_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Data Visualization Developer with expertise in ECharts and React integration | Task: Create Treemap component following requirements 1-4, rendering ECharts treemap with color-coded complexity, click/hover handlers, and smooth animations | Restrictions: Must use React.memo for performance, dispose ECharts instance on unmount, handle window resize, support drill-down filtering | Success: Treemap renders correctly with proportional rectangles, colors map to complexity, click/hover work, animations smooth, performance 60 FPS, responsive to window resize. After completion, log implementation with log-implementation tool documenting the component architecture and ECharts configuration. Mark task as complete in tasks.md._

- [x] 4.4. Create AnalysisView feature component
  - Files: `src/features/analysis/AnalysisView.tsx`
  - Orchestrate Treemap, Breadcrumb, DetailPanel components
  - Add repository path input and analyze button
  - Implement drill-down state management
  - Purpose: Provide complete analysis UI workflow
  - _Leverage: All previous components, `useAnalysis` hook, `analysisStore`_
  - _Requirements: 1, 3, 4, 6_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React Feature Developer with expertise in component composition and state orchestration | Task: Create AnalysisView feature component following requirements 1, 3, 4, and 6, orchestrating Treemap, Breadcrumb, and DetailPanel with drill-down logic | Restrictions: Must use useAnalysis hook, subscribe to analysisStore, handle loading/error states with UI feedback, support keyboard navigation | Success: All components integrated correctly, drill-down works with breadcrumb sync, file detail panel opens on click, loading/error states displayed, keyboard navigation functional. After completion, log implementation with log-implementation tool documenting the feature component architecture. Mark task as complete in tasks.md._

- [x] 4.5. Update App.tsx with AnalysisView
  - Files: `src/App.tsx`
  - Add AnalysisView as main content
  - Configure Tailwind dark mode support
  - Purpose: Wire feature into application
  - _Leverage: `AnalysisView` component_
  - _Requirements: 1_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React Application Developer with expertise in app structure and theming | Task: Update App.tsx following requirement 1, rendering AnalysisView as main content and configuring Tailwind dark mode | Restrictions: Must keep app structure simple, enable dark mode toggle, ensure Tailwind classes applied correctly | Success: AnalysisView renders in app, dark mode works, app compiles and runs, no console errors. After completion, log implementation with log-implementation tool documenting the app structure. Mark task as complete in tasks.md._

## Phase 5: Logging & Telemetry

- [x] 5.1. Add tracing instrumentation to code-viz-core
  - Files: `crates/code-viz-core/src/analyzer.rs`, `crates/code-viz-core/src/parser.rs`, `crates/code-viz-core/src/scanner.rs`
  - Add `#[tracing::instrument]` to key functions (analyze, parse_file, scan_directory)
  - Instrument performance milestones with `tracing::info!` and `tracing::debug!`
  - Purpose: Enable structured logging for debugging
  - _Leverage: `tracing` crate (add as dependency)_
  - _Requirements: 8_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Observability Engineer with expertise in tracing and structured logging | Task: Add tracing instrumentation to code-viz-core following requirement 8, instrumenting analyze, parse_file, and scan_directory with structured logs | Restrictions: Must not change function signatures, use appropriate log levels (info for milestones, debug for details), preserve performance | Success: Logs emit structured JSON, key functions instrumented, debug mode shows detailed logs, no performance regression. After completion, log implementation with log-implementation tool documenting the instrumented functions. Mark task as complete in tasks.md._

- [x] 5.2. Configure tracing subscriber with JSON output
  - Files: `crates/code-viz-tauri/src/logging.rs`, `src-tauri/src/main.rs`
  - Set up tracing subscriber with JSON formatter
  - Add environment variable control for log level (CODE_VIZ_DEBUG)
  - Purpose: Output structured logs to stderr
  - _Leverage: `tracing-subscriber` with JSON layer_
  - _Requirements: 8_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Logging Configuration Expert with expertise in tracing-subscriber and structured logging | Task: Configure tracing subscriber following requirement 8, setting up JSON formatter with environment variable log level control | Restrictions: Must output to stderr, support CODE_VIZ_DEBUG=1 for debug logs, include timestamp and level in JSON | Success: Logs output as valid JSON, environment variable controls log level, timestamps accurate, logs are parseable. After completion, log implementation with log-implementation tool documenting the logging configuration. Mark task as complete in tasks.md._

- [x] 5.3. Add request ID correlation for IPC calls
  - Files: `crates/code-viz-tauri/src/commands.rs`, `src/hooks/useTauriCommand.ts`
  - Generate unique request ID on frontend, pass to backend
  - Include request ID in all backend logs
  - Purpose: Correlate frontend and backend logs
  - _Leverage: `uuid` crate (backend), crypto.randomUUID (frontend)_
  - _Requirements: 8_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Full-Stack Observability Developer with expertise in distributed tracing and correlation IDs | Task: Add request ID correlation following requirement 8, generating IDs on frontend and threading through backend with tracing context | Restrictions: Must generate cryptographically random IDs, include in all log events, pass via IPC cleanly | Success: Request IDs present in all logs, frontend and backend logs correlatable, IDs are unique, no performance impact. After completion, log implementation with log-implementation tool documenting the correlation mechanism. Mark task as complete in tasks.md._

## Phase 6: Testing & Quality Assurance

- [x] 6.1. Create unit tests for utilities
  - Files: `src/utils/colors.test.ts`, `src/utils/formatting.test.ts`, `src/utils/treeTransform.test.ts`
  - Test all utility functions with edge cases
  - Use Vitest as test runner
  - Purpose: Ensure utility reliability
  - _Leverage: Vitest, existing utilities_
  - _Requirements: All_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: QA Engineer with expertise in unit testing and Vitest | Task: Create comprehensive unit tests for all utility functions covering edge cases, using Vitest as test runner | Restrictions: Must test success and failure paths, achieve 80%+ coverage, tests must be isolated and fast | Success: All utility functions tested, edge cases covered, tests pass consistently, coverage >80%. After completion, log implementation with log-implementation tool documenting the test coverage. Mark task as complete in tasks.md._

- [x] 6.2. Create unit tests for hooks
  - Files: `src/hooks/useTauriCommand.test.ts`, `src/hooks/useAnalysis.test.ts`
  - Test hooks with mocked Tauri commands
  - Use @testing-library/react-hooks
  - Purpose: Ensure hook reliability
  - _Leverage: Vitest, @testing-library/react-hooks, mock Tauri invoke_
  - _Requirements: 5, 7_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React Testing Expert with expertise in hook testing and mocking | Task: Create unit tests for custom hooks following requirements 5 and 7, mocking Tauri invoke and testing loading/error states | Restrictions: Must mock Tauri API completely, test all state transitions, ensure cleanup is tested | Success: All hooks tested with mocked Tauri, loading/error states verified, cleanup tested, coverage >80%. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

- [ ] 6.3. Create component tests
  - Files: `src/components/common/Breadcrumb.test.tsx`, `src/components/common/DetailPanel.test.tsx`, `src/components/visualizations/Treemap.test.tsx`
  - Test component rendering and user interactions
  - Use @testing-library/react and user-event
  - Purpose: Ensure component reliability
  - _Leverage: Vitest, @testing-library/react, mock data_
  - _Requirements: 1, 2, 3, 6_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Frontend Testing Engineer with expertise in React Testing Library and component testing | Task: Create component tests following requirements 1-3 and 6, testing rendering and user interactions with mock data | Restrictions: Must test user interactions (click, hover, keyboard), mock ECharts for Treemap, ensure accessibility | Success: All components tested, interactions verified, accessibility tested, snapshot tests included, coverage >80%. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

- [ ] 6.4. Create Rust unit tests for transformation
  - Files: `crates/code-viz-tauri/src/transform.rs` (add #[cfg(test)] mod tests)
  - Test flat_to_hierarchy with various file structures
  - Test edge cases (empty, single file, deep nesting)
  - Purpose: Ensure transformation reliability
  - _Leverage: Rust test framework, mock FileMetrics_
  - _Requirements: 1, 5_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Testing Engineer with expertise in unit testing and data structures | Task: Create unit tests for transformation functions following requirements 1 and 5, testing flat_to_hierarchy with edge cases | Restrictions: Must test empty input, single file, deep nesting, large datasets, ensure O(n) complexity | Success: All transformation paths tested, edge cases covered, tests pass, performance verified. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

- [ ] 6.5. Create E2E tests with Playwright
  - Files: `tests/e2e/treemap.spec.ts`, `playwright.config.ts`
  - Set up Playwright for Tauri testing
  - Test full user flow: analyze → drill down → view details
  - Purpose: Validate end-to-end functionality
  - _Leverage: Playwright with Tauri test driver, sample repository_
  - _Requirements: All_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: E2E Testing Expert with expertise in Playwright and Tauri testing | Task: Create end-to-end tests covering all requirements, testing complete user workflows with Playwright and Tauri test driver | Restrictions: Must test on real Tauri app, use sample repository for consistency, ensure tests are reliable and fast | Success: Full user flow tested (analyze → drill down → details), tests pass consistently, performance validated (<3s render), keyboard navigation tested. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

## Phase 7: Integration & Polish

- [ ] 7.1. Verify CLI-GUI parity
  - Files: `tests/integration/cli-gui-parity.test.ts`
  - Run same analysis via CLI (JSON output) and GUI
  - Compare results (LOC totals, file counts, largest files)
  - Purpose: Validate single source of truth
  - _Leverage: Existing CLI binary, GUI analyze command, sample repository_
  - _Requirements: 7_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Integration Testing Engineer with expertise in cross-interface validation | Task: Create parity tests following requirement 7, running CLI and GUI analysis on same repository and comparing outputs | Restrictions: Must use identical AnalysisConfig, compare all metrics exactly, test on multiple repository sizes | Success: CLI and GUI produce identical results (LOC, file counts, largest files), test passes on small and large repos, parity validated. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

- [ ] 7.2. Add loading states and error boundaries
  - Files: `src/App.tsx`, `src/features/analysis/AnalysisView.tsx`
  - Add React error boundaries wrapping components
  - Implement loading skeletons and error messages
  - Purpose: Improve user experience
  - _Leverage: React error boundaries, Tailwind for skeleton UI_
  - _Requirements: 4_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React UX Developer with expertise in error handling and loading states | Task: Add error boundaries and loading states following requirement 4, implementing graceful error fallbacks and skeleton UI | Restrictions: Must catch all component errors, show user-friendly messages, provide retry options | Success: Errors caught and displayed gracefully, loading states smooth, skeleton UI professional, user can recover from errors. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

- [ ] 7.3. Optimize performance for large datasets
  - Files: `src/components/visualizations/Treemap.tsx`, `src/utils/treeTransform.ts`
  - Add React.memo for Treemap, useMemo for transformations
  - Implement ECharts lazy rendering for >50K files
  - Purpose: Ensure smooth performance at scale
  - _Leverage: React performance APIs, ECharts lazy loading_
  - _Requirements: 4_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: React Performance Engineer with expertise in optimization and profiling | Task: Optimize performance following requirement 4, adding memoization and lazy rendering for large datasets (>50K files) | Restrictions: Must maintain functionality, use React DevTools Profiler for validation, ensure 60 FPS | Success: Treemap renders <3s for 100K files, drill-down transitions <500ms, maintains 60 FPS during interactions, React Profiler shows minimal re-renders. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

- [ ] 7.4. Add keyboard navigation support
  - Files: `src/components/visualizations/Treemap.tsx`, `src/features/analysis/AnalysisView.tsx`
  - Implement focus management and keyboard handlers (Enter, Escape, Tab)
  - Add visible focus indicators
  - Purpose: Ensure accessibility
  - _Leverage: React focus management, Tailwind focus styles_
  - _Requirements: 6_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Accessibility Engineer with expertise in keyboard navigation and ARIA | Task: Implement keyboard navigation following requirement 6, adding Enter (select), Escape (back), Tab (navigate), with visible focus indicators | Restrictions: Must follow WCAG 2.1 guidelines, test with screen readers, ensure focus trap in modal | Success: All interactions work via keyboard, focus indicators visible (Tailwind ring), Tab order logical, Escape closes panels, Enter drills down. After completion, log implementation with log-implementation tool. Mark task as complete in tasks.md._

- [ ] 7.5. Final integration testing and bug fixes
  - Files: Various (bug fixes as needed)
  - Run full test suite (unit, integration, E2E)
  - Fix any failing tests or discovered bugs
  - Validate all requirements met
  - Purpose: Ensure production readiness
  - _Leverage: All tests created in Phase 6_
  - _Requirements: All_
  - _Prompt: Implement the task for spec treemap-visualization, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Senior QA Engineer with expertise in integration testing and bug triage | Task: Run complete test suite covering all requirements, fix failing tests and bugs, validate all acceptance criteria met | Restrictions: Must not skip any tests, document all bugs found and fixed, ensure no regressions introduced | Success: All tests pass (unit, integration, E2E), no critical bugs, all requirements validated, production-ready build created. After completion, log implementation with log-implementation tool documenting final test results and bugs fixed. Mark task as complete in tasks.md._

---

## Implementation Notes

**Task Execution Order:**
- Tasks within each phase can be parallelized if independent
- Phases must be completed sequentially (Phase 2 requires Phase 1, etc.)
- Each task is atomic (1-3 files) and independently testable

**Success Validation:**
- Each task includes specific success criteria
- Mark task as completed ([ ] → [x]) only when all criteria met
- Use `log-implementation` tool after each task to document artifacts

**Autonomous Agent Instructions:**
- Read the `_Prompt` field for detailed role and task context
- Follow `_Leverage` section to reuse existing code
- Reference `_Requirements` to understand acceptance criteria
- After completion, log implementation with artifacts, then mark task complete
