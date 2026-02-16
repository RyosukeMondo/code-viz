//! Shared API layer for code-viz
//!
//! This crate provides the Single Source of Truth (SSOT) for all API handlers,
//! models, and transformations. Both Tauri and Web implementations depend on
//! this crate to ensure zero duplication and compile-time consistency.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  code-viz-api (THIS CRATE)              │
//! │  - Shared types (TreeNode, etc.)        │
//! │  - Handler trait (SSOT contract)        │
//! │  - Transformations (flat_to_hierarchy)  │
//! │  - Contract tests (JSON validation)     │
//! └─────────────────────────────────────────┘
//!          ▲                         ▲
//!          │                         │
//!   ┌──────┴────────┐       ┌───────┴────────┐
//!   │ code-viz-tauri│       │ code-viz-web   │
//!   │ (IPC wrapper) │       │ (HTTP wrapper) │
//!   └───────────────┘       └────────────────┘
//! ```

pub mod contracts;
pub mod error;
pub mod handlers;
pub mod models;
pub mod transform;

pub use error::*;
pub use handlers::*;
pub use models::*;
