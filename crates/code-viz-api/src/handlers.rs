//! Shared API handlers - Single Source of Truth
//!
//! This module defines the handler trait and implementations that BOTH
//! Tauri and Web must use. This ensures compile-time consistency.

use crate::error::ApiError;
use crate::models::TreeNode;
use crate::transform::flat_to_hierarchy;
use code_viz_core::traits::{AppContext, FileSystem, GitProvider};
use code_viz_dead_code::DeadCodeResult;
use std::path::PathBuf;

/// SSOT Handler Trait - Both Tauri and Web MUST implement this
///
/// This trait defines the complete API surface. Any handler that implements
/// this trait is guaranteed to have the same interface, preventing drift.
#[async_trait::async_trait]
pub trait ApiHandler: Send + Sync {
    /// Analyze a repository and return hierarchical tree structure
    async fn analyze_repository(
        &self,
        path: String,
        request_id: Option<String>,
    ) -> Result<TreeNode, ApiError>;

    /// Analyze dead code in a repository
    async fn analyze_dead_code(
        &self,
        path: String,
        min_confidence: u8,
        request_id: Option<String>,
    ) -> Result<DeadCodeResult, ApiError>;
}

/// Shared handler implementation using dependency injection
///
/// This is the actual business logic that both Tauri and Web use.
/// They just provide different Context implementations.
///
/// Note: This struct stores references to avoid cloning. For use with
/// impl Trait functions, use the standalone handler functions instead.
pub struct SharedHandler<C, F, G>
where
    C: AppContext + Clone,
    F: FileSystem + Clone,
    G: GitProvider + Clone,
{
    ctx: C,
    fs: F,
    git: G,
}

impl<C, F, G> SharedHandler<C, F, G>
where
    C: AppContext + Clone,
    F: FileSystem + Clone,
    G: GitProvider + Clone,
{
    pub fn new(ctx: C, fs: F, git: G) -> Self {
        Self { ctx, fs, git }
    }
}

#[async_trait::async_trait]
impl<C, F, G> ApiHandler for SharedHandler<C, F, G>
where
    C: AppContext + Clone + Send + Sync,
    F: FileSystem + Clone + Send + Sync,
    G: GitProvider + Clone + Send + Sync,
{
    async fn analyze_repository(
        &self,
        path: String,
        _request_id: Option<String>,
    ) -> Result<TreeNode, ApiError> {
        let repo_path = PathBuf::from(&path);

        // Call framework-agnostic command layer (clones because impl Trait consumes)
        let analysis_result = code_viz_commands::analyze_repository(
            &repo_path,
            self.ctx.clone(),
            self.fs.clone(),
        )
        .await
        .map_err(|e| ApiError::AnalysisFailed(e.to_string()))?;

        // Transform flat metrics to hierarchical tree (presentation layer)
        let tree = flat_to_hierarchy(analysis_result.files);

        Ok(tree)
    }

    async fn analyze_dead_code(
        &self,
        path: String,
        min_confidence: u8,
        _request_id: Option<String>,
    ) -> Result<DeadCodeResult, ApiError> {
        let repo_path = PathBuf::from(&path);

        // Call framework-agnostic command layer (clones because impl Trait consumes)
        let analysis_result = code_viz_commands::calculate_dead_code(
            &repo_path,
            self.ctx.clone(),
            self.fs.clone(),
            self.git.clone(),
        )
        .await
        .map_err(|e| ApiError::DeadCodeFailed(e.to_string()))?;

        // Filter by confidence score (presentation layer)
        let filtered_result = analysis_result.filter_by_confidence(min_confidence);

        Ok(filtered_result)
    }
}

/// Standalone handler functions for frameworks that don't use traits
///
/// These are convenience wrappers for simple function-based APIs.
/// Useful for Tauri which prefers free functions.
pub async fn analyze_repository_handler<C, F>(
    ctx: C,
    fs: F,
    path: String,
    _request_id: Option<String>,
) -> Result<TreeNode, ApiError>
where
    C: AppContext,
    F: FileSystem,
{
    let repo_path = PathBuf::from(&path);

    let analysis_result = code_viz_commands::analyze_repository(&repo_path, ctx, fs)
        .await
        .map_err(|e| ApiError::AnalysisFailed(e.to_string()))?;

    let tree = flat_to_hierarchy(analysis_result.files);

    Ok(tree)
}

pub async fn analyze_dead_code_handler<C, F, G>(
    ctx: C,
    fs: F,
    git: G,
    path: String,
    min_confidence: u8,
    _request_id: Option<String>,
) -> Result<DeadCodeResult, ApiError>
where
    C: AppContext,
    F: FileSystem,
    G: GitProvider,
{
    let repo_path = PathBuf::from(&path);

    let analysis_result = code_viz_commands::calculate_dead_code(&repo_path, ctx, fs, git)
        .await
        .map_err(|e| ApiError::DeadCodeFailed(e.to_string()))?;

    let filtered_result = analysis_result.filter_by_confidence(min_confidence);

    Ok(filtered_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use code_viz_core::mocks::{MockContext, MockGit};
    use crate::contracts::test_utils::RealFileSystem;

    #[tokio::test]
    async fn test_handler_analyze_repository() {
        let ctx = MockContext::new();
        let fs = RealFileSystem::new();
        let current_dir = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let result = analyze_repository_handler(ctx, fs, current_dir, None).await;
        assert!(result.is_ok(), "Handler should succeed: {:?}", result);
    }

    #[tokio::test]
    async fn test_handler_analyze_dead_code() {
        let ctx = MockContext::new();
        let fs = RealFileSystem::new();
        let git = MockGit::new();
        let current_dir = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let result = analyze_dead_code_handler(ctx, fs, git, current_dir, 70, None).await;
        // Dead code analysis may fail if no entry points found, which is acceptable
        match result {
            Ok(_) => {}, // Success case
            Err(ApiError::DeadCodeFailed(msg)) if msg.contains("No entry points") => {}, // Expected
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}
