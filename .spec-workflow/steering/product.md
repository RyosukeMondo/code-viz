# Product Overview

## Product Purpose

Code-Viz is a comprehensive code monitoring and visualization platform designed to address the critical challenge of **AI-generated code bloat** in the era of AI coding assistants (GitHub Copilot, Cursor, Windsurf). As AI agents generate code at unprecedented speeds, codebases rapidly accumulate technical debt through redundant logic, unnecessary complexity, and dead code. Code-Viz provides real-time, visual insights into codebase health, enabling developers to maintain control and prevent systems from becoming unmaintainable black boxes.

## Target Users

### Primary Users: Software Engineers & Tech Leads
- **Pain Points**:
  - Cannot keep up with reviewing massive volumes of AI-generated code
  - Lack visibility into which AI-generated code is actually being used
  - Struggle to identify bloated, overly complex, or dead code regions
  - Need to understand how codebase evolved over time through AI interventions

### Secondary Users: Engineering Managers & Architects
- **Pain Points**:
  - Require metrics to evaluate code quality trends
  - Need to prioritize refactoring efforts based on objective data
  - Want to understand impact of AI tools on technical debt

## Key Features

1. **SpaceSniffer-like Treemap Visualization**
   - Interactive 2D hierarchical view showing code volume (LOC) as rectangle size
   - Color-coded by cognitive complexity or dead code ratio (green → yellow → red heatmap)
   - Drill-down navigation through directory structures

2. **Dead Code Detection via Semantic Analysis**
   - Uses stack-graphs for cross-file reachability analysis
   - Identifies unused functions, classes, and modules with function-level granularity
   - Distinguishes between local and global dead code

3. **Git History Time Travel**
   - Timeline slider to replay codebase evolution from first to latest commit
   - Visual highlighting of AI-generated commits (identified by author/commit message)
   - Animated transitions showing code growth/reduction over time

4. **3D Code City Visualization**
   - Buildings represent files (height = LOC/methods, footprint = dependencies)
   - Districts represent directories
   - Connection lines show import/export relationships

5. **AI-Specific Metrics**
   - **AI Bloat Index**: (duplicate code × cognitive complexity) / test coverage
   - **Zombie Code Score**: Dead code with long-term stagnation and high internal coupling
   - **Churn Analysis**: Identify frequently-changed files correlated with complexity

## Business Objectives

- **Prevent Technical Debt Accumulation**: Provide early warning signals before codebases become unmaintainable
- **Optimize AI Tool Usage**: Help teams understand and control AI code generation impact
- **Accelerate Refactoring Decisions**: Surface objective data to prioritize cleanup efforts
- **Improve Code Review Efficiency**: Visual diff alternatives for massive AI-generated changes

## Success Metrics

- **Adoption**: Number of repositories monitored per active user (Target: 5+ repos/user)
- **Action Rate**: Percentage of users who delete/refactor code after viewing visualizations (Target: 40%)
- **Time Savings**: Reduction in code review time for AI-generated PRs (Target: 30% reduction)
- **Code Health**: Tracked improvement in dead code ratio and complexity scores over 90 days (Target: 20% improvement)
- **Testing Efficiency**: UAT cost reduction through automated testing (Target: 60% reduction in UAT time)
- **Bug Detection Speed**: Time to detect bugs in development (Target: Shift from runtime/UAT to compile-time, 100x faster)
- **Test Iteration Time**: Average time for full test suite (Target: <2 minutes, down from 5+ minutes)

## Product Principles

1. **Visual-First Cognition**
   - Code is "seen" not "read" - leverage spatial memory and pattern recognition
   - Use metaphors from familiar tools (SpaceSniffer) to reduce learning curve

2. **Performance at Scale**
   - Must handle codebases with 100K+ files without lag
   - Real-time updates during active development (sub-second refresh)
   - Incremental analysis to avoid full re-scans

3. **Actionable Intelligence**
   - Never show metrics without interpretation (no "info for info's sake")
   - Prioritize findings by impact (highlight high-bloat, low-effort wins)
   - Provide deletion confidence scores (safe-to-remove indicators)

4. **Transparency & Trust**
   - Show analysis methodology (why something is marked as dead code)
   - Allow users to mark false positives (learning system)
   - Preserve Git history context (who/when/why for every line)

5. **CLI-First Development**
   - Command-line interface takes priority over GUI for automation and scripting
   - Debug mode mandatory for troubleshooting and transparency
   - All features accessible via both CLI and GUI for maximum flexibility

6. **Testing-First Quality Assurance**
   - Follow test pyramid: 90% fast tests (unit + contract + CLI integration), 10% E2E
   - Catch bugs at compile-time via contract validation (Rust ↔ TypeScript interface)
   - Enable rapid iteration through fast, deterministic testing (seconds, not minutes)
   - Leverage dual-head architecture (CLI + GUI) for comprehensive, cost-effective testing

## Monitoring & Visibility

- **Dashboard Type**: Desktop application (Tauri-based) with web-based UI
- **Real-time Updates**: File system watcher (notify) + incremental re-analysis on save
- **Key Metrics Displayed**:
  - Total LOC, dead code percentage, average cognitive complexity
  - Top 10 bloated files (by AI Bloat Index)
  - Historical trend charts (complexity/size over time)
- **Sharing Capabilities**:
  - Export visualizations as PNG/SVG for documentation
  - Generate reports (JSON/CSV) for CI/CD integration
  - Shareable snapshots with permalink to specific commit views

## Future Vision

### Potential Enhancements

- **Remote Access**: Cloud-hosted analysis for team collaboration; shareable dashboard URLs with read-only access
- **CI/CD Integration**: GitHub Actions plugin to fail builds if bloat metrics exceed thresholds
- **AI Feedback Loop**: Flag AI-generated code patterns that consistently become dead code; provide prompts to improve AI output
- **Multi-Language Support**: Expand beyond TypeScript/JavaScript/Rust/Python to cover Go, Java, C++
- **Collaborative Refactoring**: Multi-user mode where team members can claim/assign bloated code regions
- **Cost Analysis**: Estimate compute/storage costs of dead code in serverless environments
