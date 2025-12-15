# Requirements Document - MVP (CLI-First)

## Introduction

The Code-Viz MVP is a **command-line analysis tool** that provides developers with instant, actionable insights into codebase health through fast analysis and flexible output formats. This CLI-first approach prioritizes:

- **Rapid Iteration**: Sub-second analysis for incremental changes via watch mode
- **Scriptability**: JSON/CSV output for integration with CI/CD pipelines and custom tooling
- **Developer Workflow**: Fits naturally into terminal-based development (alongside git, ripgrep, etc.)
- **Composability**: Output can be piped to other tools or visualized with external viewers

The MVP enables developers to:
- **Analyze** any codebase with a single command: `code-viz analyze ./src`
- **Monitor** changes in real-time: `code-viz watch ./src`
- **Export** metrics as JSON/CSV for custom dashboards or CI checks
- **Debug** analysis with verbose logging and structured traces

By building a robust CLI foundation first, we enable maximum flexibility: developers can integrate Code-Viz into their existing workflows immediately, and GUI layers can be added later as thin clients consuming the CLI's structured output.

## Alignment with Product Vision

This MVP directly supports the product vision outlined in `product.md`:

- **CLI-First Development** (Product Principle #5): **PRIMARY FOCUS** - Build powerful CLI tool that developers can script, automate, and integrate into their existing terminal workflows
- **Performance at Scale** (Product Principle #2): Incremental analysis and watch mode enable real-time feedback loops essential for rapid iteration
- **Actionable Intelligence** (Product Principle #3): JSON output provides raw data that can be consumed by any visualization tool, CI system, or custom script
- **Visual-First Cognition** (Product Principle #1): **DEFERRED TO GUI LAYER** - CLI outputs structured data; visualization is a separate concern (can use external tools like `jq`, custom dashboards, or future Tauri GUI)

The MVP establishes the **analysis engine** as the core value - visualization becomes a pluggable consumer of this engine's output.

## Requirements

### Requirement 1: CLI Interface and Command Structure

**User Story:** As a developer, I want to run a simple command to analyze my codebase, so that I can integrate code health checks into my scripts and workflows.

#### Acceptance Criteria

1. WHEN the user runs `code-viz --help` THEN the system SHALL display usage documentation including:
   - Available commands (analyze, watch, export)
   - Common options (--exclude, --format, --verbose)
   - Example invocations

2. WHEN the user runs `code-viz analyze <path>` THEN the system SHALL:
   - Scan all supported files in `<path>` recursively
   - Print a text-based summary to stdout (total files, total LOC, largest files)
   - Exit with code 0 on success, non-zero on error

3. WHEN the user runs `code-viz analyze <path> --format json` THEN the system SHALL:
   - Output structured JSON to stdout with schema:
     ```json
     {
       "summary": { "total_files": 1234, "total_loc": 56789, ... },
       "files": [
         { "path": "src/main.rs", "loc": 450, "functions": 12, ... }
       ]
     }
     ```
   - Allow piping to other tools: `code-viz analyze . --format json | jq '.summary.total_loc'`

4. WHEN the user runs `code-viz analyze <path> --exclude "node_modules/**"` THEN the system SHALL skip matching paths

5. WHEN the user runs `code-viz analyze <path> --verbose` THEN the system SHALL:
   - Print structured logs to stderr (JSON format, filterable with `jq`)
   - Show progress: "Analyzing 1234/5678 files..."
   - Debug mode logs: parser timings, cache hits, file skips

### Requirement 2: Repository Analysis Engine

**User Story:** As a software developer, I want the CLI to analyze my codebase and calculate metrics, so that I can track code health over time and identify problem areas.

#### Acceptance Criteria

1. WHEN analyzing files THEN the system SHALL calculate per-file metrics:
   - Lines of Code (LOC, excluding comments/blank lines)
   - File size in bytes
   - Number of functions/methods
   - Last modified timestamp

2. WHEN a repository contains 10,000 files THEN the system SHALL complete analysis within 30 seconds on a mid-range laptop

3. WHEN the user runs `code-viz analyze . --format json > metrics.json` THEN the system SHALL produce a valid JSON file that can be:
   - Committed to version control for historical tracking
   - Diffed with previous runs: `diff metrics-old.json metrics-new.json`
   - Queried: `jq '.files | map(select(.loc > 500)) | length' metrics.json` (count files >500 LOC)

4. IF a file cannot be parsed THEN the system SHALL:
   - Log a warning to stderr with file path and error reason
   - Continue analyzing other files
   - NOT fail the entire analysis

5. WHEN analysis completes THEN the system SHALL write a cache file to `.code-viz/cache` to speed up subsequent runs

### Requirement 3: Watch Mode for Real-Time Feedback

**User Story:** As a developer actively writing code, I want the CLI to re-analyze files automatically when I save, so that I can see immediate feedback on how my changes affect code health.

#### Acceptance Criteria

1. WHEN the user runs `code-viz watch <path>` THEN the system SHALL:
   - Perform initial analysis
   - Monitor the directory for file changes (create, modify, delete)
   - Print "Watching for changes..." to stderr
   - Stay running until Ctrl+C

2. WHEN a monitored file is modified THEN the system SHALL:
   - Re-analyze only the changed file (incremental)
   - Print updated metrics to stdout within 100ms
   - Example output: `[2025-12-15 14:30:45] src/main.rs: 450 LOC (+12 from previous)`

3. WHEN the user runs `code-viz watch <path> --format json` THEN the system SHALL:
   - Print a new JSON object to stdout on each change (newline-delimited JSON stream)
   - Allow streaming to analysis tools: `code-viz watch . --format json | my-custom-dashboard`

4. WHEN multiple files change rapidly (e.g., git checkout) THEN the system SHALL:
   - Debounce updates (max 1 update per second)
   - Batch-analyze all changed files

5. WHEN watch mode encounters an error THEN the system SHALL:
   - Log error to stderr
   - Continue watching (don't exit on single-file errors)

### Requirement 4: Multi-Language Support

**User Story:** As a developer working in polyglot codebases, I want Code-Viz to analyze multiple programming languages, so that I can get unified metrics across my entire project.

#### Acceptance Criteria

1. WHEN the user analyzes a repository THEN the system SHALL support:
   - TypeScript (`.ts`, `.tsx`)
   - JavaScript (`.js`, `.jsx`)
   - Rust (`.rs`)
   - Python (`.py`)

2. WHEN calculating LOC THEN the system SHALL correctly exclude language-specific comments:
   - Single-line: `//` (JS/TS/Rust), `#` (Python)
   - Multi-line: `/* */` (JS/TS/Rust), `'''` / `"""` (Python)

3. WHEN the user runs `code-viz analyze . --format json` THEN the output SHALL include a `language` field for each file:
   ```json
   { "path": "src/main.rs", "language": "rust", "loc": 450 }
   ```

4. WHEN parsing a file THEN the system SHALL use Tree-sitter grammars for accurate syntax analysis

### Requirement 5: Configuration and Filtering

**User Story:** As a developer, I want to configure exclusion patterns and defaults, so that I don't have to pass the same flags on every invocation.

#### Acceptance Criteria

1. WHEN the user runs `code-viz analyze .` THEN the system SHALL apply default exclusions:
   - `node_modules/`, `dist/`, `build/`, `out/`, `target/`, `__pycache__/`, `.git/`

2. WHEN a `.code-viz.toml` config file exists in the repository root THEN the system SHALL:
   - Read exclusion patterns from `[analysis] exclude = ["**/generated/**"]`
   - Read output format preferences: `[output] format = "json"`
   - Allow CLI flags to override config (CLI > config > defaults)

3. WHEN the user runs `code-viz config init` THEN the system SHALL:
   - Create a `.code-viz.toml` template file
   - Include commented examples of all configuration options

4. WHEN the user runs `code-viz analyze . --exclude "**/tests/**"` THEN that pattern SHALL be merged with config-based exclusions

### Requirement 6: Export and CI/CD Integration

**User Story:** As a developer, I want to export metrics in various formats and fail CI builds based on thresholds, so that I can enforce code quality standards automatically.

#### Acceptance Criteria

1. WHEN the user runs `code-viz analyze . --format csv > metrics.csv` THEN the system SHALL output CSV format:
   ```
   path,language,loc,functions,size_bytes
   src/main.rs,rust,450,12,8192
   ```

2. WHEN the user runs `code-viz analyze . --threshold loc=500` THEN the system SHALL:
   - Exit with code 1 if any file exceeds 500 LOC
   - Print violating files to stderr: `ERROR: src/huge.rs has 756 LOC (limit: 500)`
   - Enable CI pipeline to fail: `code-viz analyze . --threshold loc=500 || exit 1`

3. WHEN the user runs `code-viz analyze . --format json --output report.json` THEN the system SHALL write to file instead of stdout

4. WHEN the user runs `code-viz diff metrics-old.json metrics-new.json` THEN the system SHALL:
   - Print a human-readable summary of changes:
     ```
     +12 files added
     -3 files deleted
     Total LOC: 45,678 → 48,901 (+3,223)
     Largest growth: src/api.ts (+234 LOC)
     ```

5. WHEN the user runs `code-viz analyze . --baseline metrics-baseline.json` THEN the system SHALL:
   - Compare current analysis to baseline
   - Exit with code 1 if total LOC increased by >10%
   - Support "code health ratcheting" in CI

## Non-Functional Requirements

### Code Architecture and Modularity

- **Single Responsibility**:
  - Analysis engine (Rust library crate) is pure: takes paths → returns metrics (no CLI concerns)
  - CLI binary (separate crate) handles argument parsing, output formatting, process exit codes
  - Output formatters (JSON, CSV, text) are pluggable modules

- **Testability**:
  - Analysis engine has zero dependencies on CLI framework (clap)
  - All core logic unit-tested with snapshot tests (insta crate)
  - CLI integration tests use temp directories and verify stdout/stderr

- **Modularity**:
  - Parser module uses trait `LanguageParser` - adding new languages requires zero changes to core engine
  - Output formatters implement trait `MetricsFormatter` - adding new formats (e.g., Prometheus) is isolated
  - File watcher abstracted behind `ChangeMonitor` trait

- **Dependency Management**:
  - CLI crate depends on analysis crate (not vice versa)
  - Analysis crate has minimal dependencies (tree-sitter, serde, rayon)
  - No GUI dependencies in MVP (no Tauri, React, Three.js)

### Performance

- **Startup Time**: CLI shall execute `--help` in <50ms (fast enough for shell autocomplete)
- **Analysis Speed**:
  - Initial scan: <30 seconds for 100K files (parallelized with rayon)
  - Incremental update (watch mode): <100ms for single file
  - Cache hit: <1 second for re-analyzing unchanged repo
- **Memory Efficiency**:
  - <100MB baseline
  - <500MB for 10K files
  - Stream output (don't buffer entire JSON in memory before printing)

### Developer Experience (Rapid Iteration)

- **Build Speed**:
  - Incremental Rust builds: <3 seconds with mold linker and sccache
  - `cargo watch -x run` for auto-recompile on save
  - Justfile recipes: `just dev` starts watch mode

- **Debug Mode**:
  - `--verbose` flag enables structured logging (JSON logs to stderr)
  - `RUST_LOG=debug code-viz analyze .` for detailed tracing
  - `--profile` flag outputs flame graph data for performance optimization

- **Fast Feedback Loop**:
  - Developer workflow: `code-viz watch . --verbose` in tmux/zellij pane
  - Sees analysis results update in <100ms after saving a file
  - Can pipe watch output to custom scripts for instant alerts

### Security

- **Local-Only**: No network requests; all operations offline
- **Read-Only**: Only read file system (no writes except `.code-viz/cache`)
- **Safe Defaults**: Skip symlinks, limit file size (warn on files >10MB)

### Reliability

- **Error Handling**:
  - Fail fast on invalid arguments (print usage, exit 1)
  - Fail gracefully on file errors (log, continue)
  - Structured errors with context: `ERROR: Failed to parse src/main.rs: unexpected token at line 45`

- **Cross-Platform**: CLI must work on macOS, Linux, Windows (use `std::path::PathBuf`, not string manipulation)

### Usability

- **Shell Integration**:
  - Exit codes follow Unix conventions (0 = success, 1 = error, 2 = usage error)
  - Respect `NO_COLOR` environment variable
  - Support shell completion: `code-viz completions bash > /etc/bash_completion.d/code-viz`

- **Documentation**:
  - `--help` is comprehensive with examples
  - `man code-viz` (future: auto-generate from help text)
  - README includes common recipes (CI integration, watch mode, custom scripts)

## Success Criteria

The MVP is considered successful if:

1. **Functional Completeness**: All 6 requirements implemented and passing acceptance criteria
2. **Performance Benchmark**: Analyzes Rust compiler repo (~500K LOC) in <10 seconds
3. **Developer Validation**: 5 beta testers successfully integrate `code-viz` into their workflows (shell aliases, git hooks, CI pipelines) without assistance
4. **Composability**: Can be composed with standard Unix tools:
   - `code-viz analyze . --format json | jq '.summary.total_loc'`
   - `code-viz watch . | grep "ERROR"`
   - `code-viz analyze . --threshold loc=500 && git commit`
5. **Technical Foundation**: Analysis engine is library-first; GUI can be built as separate binary importing the engine

## Out of Scope for MVP

Explicitly deferred to post-MVP or separate tools:

- ❌ GUI/Desktop Application (Tauri) - Build as separate binary consuming CLI JSON output
- ❌ Treemap visualization (use external tool like `https://codesee.io` or custom web app)
- ❌ Dead code detection (stack-graphs) - Requires semantic analysis beyond syntax parsing
- ❌ Git history analysis - Not needed for "current state" health checks
- ❌ 3D Code City - Visualization layer, not core analysis
- ❌ AI-specific metrics - Needs more research
- ❌ Web server mode (`code-viz serve`) - Can be added post-MVP if demand exists
- ❌ Cognitive complexity - Use simple LOC heuristic for MVP

## MVP Development Approach (Rapid Iteration)

### Phase 1: Core Engine (Week 1)
- Rust library crate with Tree-sitter integration
- Parse TypeScript/JavaScript only (defer Rust/Python)
- Calculate LOC, skip functions count
- Parallel file scanning with rayon
- Snapshot tests with insta

### Phase 2: CLI Interface (Week 2)
- Binary crate with clap for arg parsing
- `analyze` command with JSON output
- Default exclusion patterns
- Exit codes and error handling

### Phase 3: Watch Mode (Week 3)
- Integrate `notify` crate for file watching
- Incremental re-analysis
- Debouncing logic
- Stream JSON output on changes

### Phase 4: CI/CD Features (Week 4)
- `--threshold` flag
- `diff` command
- CSV output format
- GitHub Actions example workflow

Each phase is shippable and provides incremental value.

## Assumptions and Dependencies

### Assumptions
- Users comfortable with CLI tools (target audience: developers)
- JSON is acceptable output format (can use jq for queries)
- Visual representation can be built separately (D3.js web app, custom dashboard, future GUI)

### Dependencies
- **Tree-sitter 0.20+**: Core parsing engine
- **clap 4.0+**: CLI argument parsing
- **serde 1.0+**: JSON serialization
- **notify 6.0+**: File system watching
- **rayon 1.8+**: Parallel processing

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Developers expect GUI, reject CLI-only MVP | High | Market validation first: survey target users on CLI vs GUI preference; emphasize JSON output flexibility |
| Tree-sitter too slow for watch mode | Medium | Implement parse timeout; cache ASTs; provide `--fast` mode that skips detailed parsing |
| JSON output schema changes break user scripts | Medium | Semantic versioning; `--format json-v1` for stable schema; deprecation warnings |
| CLI usage too complex | Low | Comprehensive examples in README; GIF demos; shell aliases cheat sheet |

## Future Enhancement: GUI as CLI Consumer

Once CLI is stable, a GUI (Tauri app) can be built as a **thin client**:

```
┌─────────────────────┐
│   Tauri GUI App     │
│  (Visualization)    │
└──────────┬──────────┘
           │ spawns process
           ▼
┌─────────────────────┐
│  code-viz CLI       │
│  (Analysis Engine)  │
│  --format json      │
└─────────────────────┘
```

The GUI would:
- Spawn `code-viz watch . --format json` as subprocess
- Parse newline-delimited JSON stream
- Render treemap using React + ECharts
- Provide file picker UI (instead of CLI path argument)

This architecture allows:
- CLI to evolve independently
- Multiple GUIs (web, desktop, mobile) consuming same CLI
- Users to choose: terminal-only OR GUI based on preference
