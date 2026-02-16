//! Code-viz Web Server Library
//!
//! Provides HTTP/REST API access to code-viz functionality.

pub mod context;
pub mod routes;

pub use context::{RealFileSystem, RealGit, WebContext};
