#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Comprehensive error scenario tests for code-viz-core
//!
//! This test module validates all error paths across the crate,
//! ensuring errors are handled gracefully and provide useful information.
//!
//! Test modules are organized by error category in the error_scenarios/ directory

#[path = "error_scenarios/parser_errors.rs"]
mod parser_errors;

#[path = "error_scenarios/file_system_errors.rs"]
mod file_system_errors;

#[path = "error_scenarios/coverage_errors.rs"]
mod coverage_errors;

#[path = "error_scenarios/analysis_errors.rs"]
mod analysis_errors;

#[path = "error_scenarios/git_config_cache_errors.rs"]
mod git_config_cache_errors;

#[path = "error_scenarios/error_traits_conversions.rs"]
mod error_traits_conversions;

#[path = "error_scenarios/circular_dependency_errors.rs"]
mod circular_dependency_errors;
