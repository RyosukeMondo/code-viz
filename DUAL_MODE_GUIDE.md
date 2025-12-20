# Dual-Mode Frontend Guide

This document explains how the frontend automatically detects and works with both Tauri (desktop) and Web modes.

## Overview

The code-viz frontend now **automatically detects** whether it's running as:
- **Tauri Desktop App** (Windows/Mac/Linux)
- **Web Application** (Browser via HTTP)

**No configuration needed** - it just works! 🎉

## How It Works

### Auto-Detection

```typescript
// src/api/client.ts
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;
```

- **Tauri mode**: `window.__TAURI__` exists → Use IPC
- **Web mode**: `window.__TAURI__` doesn't exist → Use HTTP REST API

### Unified API Client

```typescript
// src/api/client.ts
export async function analyzeRepository(path: string): Promise<TreeNode> {
  if (isTauri) {
    // Tauri: Use IPC
    return await invoke<TreeNode>('analyze_repository', { path });
  } else {
    // Web: Use HTTP REST API
    const response = await fetch('/api/analyze', {
      method: 'POST',
      body: JSON.stringify({ path })
    });
    return await response.json();
  }
}
```

**Same function, different transport!**

## Running the Application

### Desktop Mode (Tauri)

```bash
# Development
npm run tauri dev

# Production
npm run tauri build
```

**Uses:** Tauri IPC → code-viz-api handlers

### Web Mode (HTTP Server)

```bash
# Build frontend
npm run build

# Start web server
cargo run -p code-viz-web

# Open browser
open http://localhost:3000
```

**Uses:** HTTP REST → code-viz-api handlers (same code!)

## Architecture Diagram

```
┌─────────────────────────────────────┐
│      React Frontend (Same Code)     │
│                                      │
│  ✓ Auto-detects mode                │
│  ✓ useAnalysis() hook               │
│  ✓ Unified API client                │
└──────────┬──────────────────────────┘
           │
    ┌──────┴────────┐
    │   Detection   │
    │   Logic       │
    └──────┬────────┘
           │
    ┌──────┴────────────────┐
    ▼                       ▼
┌────────────┐      ┌──────────────┐
│ Tauri IPC  │      │  HTTP REST   │
│  (invoke)  │      │  (fetch)     │
└─────┬──────┘      └──────┬───────┘
      │                    │
      ▼                    ▼
┌────────────┐      ┌──────────────┐
│code-viz-   │      │code-viz-web  │
│tauri       │      │(Axum server) │
└─────┬──────┘      └──────┬───────┘
      │                    │
      └──────────┬─────────┘
                 ▼
        ┌────────────────┐
        │ code-viz-api   │
        │ (SSOT Handlers)│
        │  SAME CODE!    │
        └────────────────┘
```

## API Endpoints (Web Mode)

When running in web mode:

- `POST /api/analyze` - Analyze repository
  ```json
  {
    "path": "/path/to/repo",
    "requestId": "optional-uuid"
  }
  ```

- `POST /api/dead-code` - Analyze dead code
  ```json
  {
    "path": "/path/to/repo",
    "minConfidence": 70,
    "requestId": "optional-uuid"
  }
  ```

- `GET /api/health` - Health check
  ```json
  {
    "status": "healthy",
    "service": "code-viz-web"
  }
  ```

## Frontend Components (No Changes Needed!)

All existing components work in both modes:

```typescript
// src/features/analysis/AnalysisView.tsx
function AnalysisView() {
  const { analyze, data, loading } = useAnalysis();

  // Works in BOTH Tauri and Web mode!
  const handleAnalyze = () => {
    analyze('/path/to/repo');
  };

  return <Treemap data={data} />;
}
```

**No mode-specific code required!**

## Environment Detection

You can check the current mode:

```typescript
import { getMode } from './api/client';

const mode = getMode(); // 'tauri' | 'web'

if (mode === 'tauri') {
  console.log('Running as desktop app');
} else {
  console.log('Running in browser');
}
```

## Debugging

Both modes log their actions:

```javascript
// Browser console / Tauri logs
[API Client] Running in Web mode
[API Client] Using HTTP REST API for /api/analyze

// or

[API Client] Running in Tauri (Desktop) mode
[API Client] Using Tauri IPC for analyze_repository
```

## Benefits

### For Users

✅ **Desktop App**: Native performance, file system access
✅ **Web App**: No installation, works anywhere
✅ **Same Features**: Identical functionality in both modes

### For Developers

✅ **Zero Duplication**: Same React code for both
✅ **SSOT Backend**: Same Rust handlers (code-viz-api)
✅ **Automatic**: No manual mode switching
✅ **Type-Safe**: TypeScript + Rust type validation

## Testing Both Modes

### Test Desktop Mode

```bash
npm run tauri dev
# Should show: [API Client] Running in Tauri (Desktop) mode
```

### Test Web Mode

```bash
npm run build
cargo run -p code-viz-web
# Open http://localhost:3000
# Should show: [API Client] Running in Web mode
```

## Deployment

### Desktop (Tauri)

```bash
npm run tauri build
# Creates installer in src-tauri/target/release/bundle/
```

### Web (Server)

```bash
# Build frontend
npm run build

# Deploy Rust binary + dist/ folder
cargo build --release -p code-viz-web
# Deploy: target/release/code-viz-web + dist/
```

## Summary

**One codebase, two modes, zero duplication!**

- ✅ Frontend auto-detects mode
- ✅ Same React components
- ✅ Same Rust handlers (SSOT)
- ✅ No manual configuration
- ✅ Full type safety maintained

The frontend "just works" regardless of how it's deployed.
