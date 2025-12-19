# LLM Context: code-viz

This document provides high-level architectural context for AI agents working on the code-viz project.

## Dependency Injection Pattern (AppContext)

We use a trait-based dependency injection pattern to decouple business logic from platform-specific APIs (Tauri, CLI, etc.) and enable 100% unit testability.

### The Trait: `AppContext`

Defined in `code-viz-core`:

```rust
#[async_trait]
pub trait AppContext: Send + Sync {
    /// Emit an event to the frontend or listener.
    async fn emit_event(&self, event: &str, payload: Value) -> Result<()>;

    /// Get the application data directory.
    fn get_app_dir(&self) -> PathBuf;

    /// Report progress of a long-running operation.
    async fn report_progress(&self, percentage: f32, message: &str) -> Result<()>;
}
```

### Implementations

1.  **`TauriContext`** (`code-viz-tauri`): Production implementation wrapping `tauri::AppHandle`.
2.  **`CliContext`** (`code-viz-cli`): Headless implementation printing to stdout.
3.  **`MockContext`** (`code-viz-tauri`): Test double for unit testing. Captures events for verification.

### Usage in Commands

Tauri commands should be refactored into two parts:
1.  A `#[tauri::command]` wrapper that takes `AppHandle` and creates a `TauriContext`.
2.  An `inner` function that takes `impl AppContext` and contains the actual logic.

**Pattern Example:**

```rust
#[tauri::command]
pub async fn my_command(app: tauri::AppHandle, path: String) -> Result<MyData, String> {
    let ctx = TauriContext::new(app);
    my_command_inner(ctx, path).await
}

pub async fn my_command_inner(ctx: impl AppContext, path: String) -> Result<MyData, String> {
    ctx.report_progress(0.1, "Initializing...").await.ok();
    // ... business logic ...
    ctx.report_progress(1.0, "Done").await.ok();
    Ok(data)
}
```

### Unit Testing with MockContext

```rust
#[tokio::test]
async fn test_my_command() {
    let ctx = MockContext::new();
    let result = my_command_inner(ctx.clone(), "test_path".to_string()).await;
    assert!(result.is_ok());
    ctx.assert_event_emitted("progress");
}
```

## Architectural Invariants

- **Business Logic Isolation**: Core analysis logic must live in `code-viz-core` or dedicated logic crates, NOT in Tauri command handlers.
- **Contract-Driven**: All IPC types must derive `specta::Type` for TypeScript binding generation.
- **Async First**: Use `async` for all I/O and long-running operations.
