# Migration Guide: Trait-Based Dependency Injection

## Overview

This guide explains how to migrate Tauri commands to use the `AppContext` trait instead of direct dependencies on `tauri::AppHandle`. This architectural change enables:
- **100% Testability**: Business logic can be tested without a full Tauri runtime using `MockContext`.
- **Flexibility**: The same logic can run in CLI (`CliContext`) or other environments.
- **Decoupling**: Business logic is separated from the application shell.

## Migration Pattern

### 1. Identify the Command
Locate the command in `crates/code-viz-tauri/src/commands.rs`.

### 2. Extract Business Logic to Inner Function
Move the core logic of the command to a new public async function that takes `impl AppContext` instead of `AppHandle`.

**Before:**
```rust
#[tauri::command]
pub async fn my_command(path: String) -> Result<MyData, String> {
    // Business logic...
    tracing::info!("Doing something");
    Ok(data)
}
```

**After:**
```rust
#[tauri::command]
pub async fn my_command(app: tauri::AppHandle, path: String) -> Result<MyData, String> {
    let ctx = TauriContext::new(app);
    my_command_inner(ctx, path).await
}

pub async fn my_command_inner(ctx: impl AppContext, path: String) -> Result<MyData, String> {
    // Business logic...
    ctx.report_progress(0.5, "Working...").await.ok();
    Ok(data)
}
```

### 3. Replace Platform APIs with AppContext Methods
- Replace `app.emit()` or `app.emit_all()` with `ctx.emit_event()`.
- Replace `app.path().app_data_dir()` with `ctx.get_app_dir()`.
- Use `ctx.report_progress()` for long-running operations.

### 4. Add Unit Tests with MockContext
Create or update tests in `crates/code-viz-tauri/tests/command_tests.rs` to verify the logic using `MockContext`.

```rust
#[tokio::test]
async fn test_my_command() {
    let ctx = MockContext::new();
    let result = my_command_inner(ctx.clone(), "path".to_string()).await;
    assert!(result.is_ok());
    ctx.assert_event_emitted("progress");
}
```

## Migration Checklist

For each command:
- [ ] Add `app: tauri::AppHandle` to command signature if not present.
- [ ] Extract logic to `[command_name]_inner` taking `impl AppContext`.
- [ ] Replace `AppHandle` calls with `AppContext` methods.
- [ ] Add `ctx.report_progress()` calls where appropriate.
- [ ] Create unit test in `command_tests.rs` using `MockContext`.
- [ ] Verify GUI still works.

## Command Inventory

| Command | Status | File |
|---------|--------|------|
| `analyze_repository` | [x] Migrated | `crates/code-viz-tauri/src/commands.rs` |
| `analyze_dead_code_command` | [ ] Pending | `crates/code-viz-tauri/src/commands.rs` |

## Benefits

- **Zero-Cost Abstraction**: Rust traits are inlined, so there is no runtime performance penalty.
- **Fast Feedback Loop**: Unit tests run in milliseconds compared to seconds for full integration tests.
- **Better DX**: AI agents can more easily reason about business logic when it's decoupled from framework boilerplate.
