# Contributing to code-viz

Thank you for your interest in contributing to code-viz! This document provides guidelines for contributing to the project.

## Code Quality Standards

This project enforces strict code quality standards to maintain a healthy codebase. All contributions must meet these standards.

### Quality Metrics

- **File Size**: Maximum 500 lines per file (excluding comments and blank lines)
- **Function Size**: Maximum 50 lines per function (recommended), 100 lines hard limit
- **Test Coverage**: Minimum 80% overall, 90% for critical paths
- **Error Handling**: No `unwrap()` or `expect()` in production code
- **Type Safety**: Minimal use of TypeScript `any` type

### Pre-commit Hooks

This project uses [Husky](https://typicode.github.io/husky/) to run automated checks before each commit. The pre-commit hook will:

1. **Check code metrics** (`scripts/check-metrics.sh`):
   - Verify file sizes are under 500 lines
   - Check function sizes (warn at 50 lines, fail at 100)
   - Detect `unwrap()`/`expect()` in production Rust code
   - Warn about TypeScript `any` usage

2. **Run Clippy** on staged Rust files:
   - Must pass with zero warnings (`-D warnings`)
   - Ensures code follows Rust best practices

3. **Run ESLint** on staged TypeScript files:
   - Must pass with zero errors
   - Enforces TypeScript and React best practices

### Bypassing Pre-commit Hooks

In rare cases where you need to bypass the pre-commit hooks (not recommended):

```bash
git commit --no-verify -m "Your commit message"
```

**Note**: Bypassing hooks should only be done in exceptional circumstances. CI will still run the same checks, so bypassed commits may fail in CI.

## Error Handling Guidelines

All production code must use proper error handling. See [MIGRATION.md](./MIGRATION.md) for detailed patterns.

### Rust Error Handling

- Use `Result<T, CodeVizError>` for fallible operations
- Use `?` operator for error propagation
- Provide context with error messages
- Never use `unwrap()` or `expect()` in production code
- Test code may use `unwrap()` with a `// Test-only unwrap: [reason]` comment

### TypeScript Error Handling

- Use try-catch blocks for async operations
- Provide user-friendly error messages
- Log errors appropriately
- Handle errors at appropriate boundaries

## Testing

All new features and bug fixes must include tests:

- **Rust**: Unit tests using `#[test]`, integration tests in `tests/`
- **TypeScript**: Component tests using Vitest and Testing Library
- **Coverage**: Aim for ≥80% coverage for new code

Run tests:

```bash
# Rust tests
cargo test --workspace

# TypeScript tests
npm run test

# Coverage reports
cargo tarpaulin --out Html
npm run test:coverage
```

## Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Run tests locally: `cargo test && npm run test`
5. Commit your changes (pre-commit hooks will run automatically)
6. Push to your fork: `git push origin feature/your-feature`
7. Open a Pull Request

## Pull Request Guidelines

- Provide a clear description of the changes
- Reference any related issues
- Ensure all CI checks pass
- Keep commits focused and atomic
- Follow conventional commit messages when possible

## Getting Help

- Check existing issues and discussions
- Join our community chat (if available)
- Ask questions in your PR

## License

By contributing to code-viz, you agree that your contributions will be licensed under the project's license.
