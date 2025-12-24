# Code-Viz Documentation

Welcome to the Code-Viz documentation. This directory contains all project documentation organized in a MECE (Mutually Exclusive, Collectively Exhaustive) structure.

## Documentation Structure

```
docs/
├── README.md (this file)
│
├── architecture/               # Architecture and design documentation
│   ├── ARCHITECTURE.md        # Main architecture overview (Rust layers)
│   ├── diagrams/              # Architecture diagrams
│   │   ├── FRONTEND_BACKEND_ARCHITECTURE.md
│   │   └── COMPONENT_INTERACTION.md
│   └── decisions/             # Architectural Decision Records (ADRs)
│       └── CHART_LIBRARY_RECOMMENDATION.md
│
├── guides/                    # User and developer guides
│   ├── development/           # Development guides
│   │   ├── fast-iteration.md # Fast development workflow
│   │   ├── tauri-cli.md      # Tauri CLI usage
│   │   └── search.md         # Code search tips
│   └── features/              # Feature documentation
│       └── DUAL_MODE_GUIDE.md # Dual-mode visualization guide
│
├── implementation/            # Implementation summaries and details
│   ├── IMPLEMENTATION_COMPLETE.md
│   ├── INTERFACE_VERIFICATION.md
│   ├── MIGRATION_COMPLETE.md
│   └── SSOT_IMPLEMENTATION_SUMMARY.md
│
├── testing/                   # Testing documentation
│   ├── TEST_COVERAGE_IMPROVEMENTS.md
│   ├── TEST_COVERAGE_RCA.md
│   ├── WHY_TESTS_MISSED_WRAPPER_BUG.md
│   └── UAT_COST_REDUCTION_PLAN.md
│
├── troubleshooting/           # Debugging and fixes
│   ├── DEBUGGING_CLICK_ISSUES.md
│   ├── DRILL_DOWN_FIX.md
│   └── WRAPPER_NODE_FIX.md
│
└── archive/                   # Deprecated/historical documents
    └── TRAIT_BASED_DI_MIGRATION.md
```

## Quick Links

### Getting Started
- [Main README](../README.md) - Project overview and setup
- [Fast Iteration Guide](guides/development/fast-iteration.md) - Quick development workflow
- [Tauri CLI Guide](guides/development/tauri-cli.md) - Tauri command reference

### Architecture
- [Architecture Overview](architecture/ARCHITECTURE.md) - Trait-based DI architecture
- [Frontend-Backend Architecture](architecture/diagrams/FRONTEND_BACKEND_ARCHITECTURE.md) - Full stack architecture
- [Component Interaction](architecture/diagrams/COMPONENT_INTERACTION.md) - Component flow diagrams

### Development
- [Development Guides](guides/development/) - Development workflows and tools
- [Feature Guides](guides/features/) - Feature-specific documentation
- [Testing Docs](testing/) - Testing strategy and coverage reports

### Troubleshooting
- [Troubleshooting](troubleshooting/) - Common issues and fixes
- [Test Coverage RCA](testing/TEST_COVERAGE_RCA.md) - Why tests missed bugs

## Document Categories

### Architecture
Contains high-level architecture documentation, design decisions, and diagrams explaining how the system is structured.

**When to add here:**
- System architecture changes
- Architectural Decision Records (ADRs)
- Architecture diagrams
- Design patterns and principles

### Guides
Step-by-step guides for developers and users on how to accomplish specific tasks.

**When to add here:**
- Development workflows
- Feature usage guides
- How-to documentation
- Tool usage guides

### Implementation
Detailed implementation summaries, verification reports, and migration notes.

**When to add here:**
- Implementation summaries
- Migration reports
- Interface verification
- SSOT implementation details

### Testing
Testing strategy, coverage reports, and test-related documentation.

**When to add here:**
- Test coverage reports
- Testing improvements
- Root cause analysis (RCA) for test failures
- UAT plans and results

### Troubleshooting
Debugging guides, bug fixes, and issue resolution documentation.

**When to add here:**
- Bug fix documentation
- Debugging guides
- Issue resolution steps
- Known issues and workarounds

### Archive
Historical or deprecated documentation kept for reference.

**When to add here:**
- Deprecated guides
- Old migration docs
- Historical context

## Contributing to Documentation

When adding new documentation:

1. **Choose the right category** - Follow MECE principles
2. **Use descriptive filenames** - `SCREAMING_SNAKE_CASE.md` for major docs
3. **Add to this README** - Update the structure tree above
4. **Link related docs** - Cross-reference related documentation
5. **Keep it current** - Archive outdated docs, don't delete them

## Document Naming Conventions

- **Architecture docs**: `ARCHITECTURE.md`, `DESIGN_PATTERN.md`
- **Guides**: `feature-guide.md`, `development-workflow.md` (kebab-case)
- **Implementation**: `IMPLEMENTATION_SUMMARY.md`, `MIGRATION_COMPLETE.md`
- **Troubleshooting**: `BUG_NAME_FIX.md`, `DEBUGGING_ISSUE.md`
- **Testing**: `TEST_COVERAGE_*.md`, `*_RCA.md`

## Maintenance

This documentation structure should be maintained as the project evolves:

- **Weekly**: Review and update implementation docs
- **Monthly**: Archive outdated documentation
- **Per Release**: Update architecture diagrams if changed
- **As Needed**: Add troubleshooting docs for new issues

---

**Last Updated**: 2025-12-24
**Maintained By**: Code-Viz Team
