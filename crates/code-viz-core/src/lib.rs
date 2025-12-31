#![allow(dead_code)]

pub mod analyzer;
pub mod cache;
pub mod coupling;
pub mod coverage;
pub mod duplication;
pub mod error;
pub mod hotspot;
pub mod metrics;
pub mod models;
pub mod parser;
pub mod scanner;
pub mod traits;
pub mod mocks;
pub mod context;

pub use analyzer::calculate_summary;
pub use models::*;