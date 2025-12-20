//! Web API routes - HTTP wrappers around shared SSOT handlers
//!
//! These routes are thin wrappers around code-viz-api handlers,
//! identical in function to the Tauri commands but using HTTP transport.

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use code_viz_api::{analyze_repository_handler, analyze_dead_code_handler, TreeNode};
use code_viz_dead_code::DeadCodeResult;
use serde::{Deserialize, Serialize};

use crate::context::{WebContext, RealFileSystem, RealGit};

/// Request body for repository analysis
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeRequest {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// Request body for dead code analysis
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeadCodeRequest {
    pub path: String,
    pub min_confidence: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// API error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Web-specific error wrapper (needed to avoid orphan rule)
pub struct WebError(code_viz_api::ApiError);

impl From<code_viz_api::ApiError> for WebError {
    fn from(err: code_viz_api::ApiError) -> Self {
        WebError(err)
    }
}

/// Convert WebError to HTTP response
impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self.0 {
            code_viz_api::ApiError::InvalidPath(_) => (StatusCode::BAD_REQUEST, self.0.to_user_message()),
            code_viz_api::ApiError::AnalysisFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_user_message()),
            code_viz_api::ApiError::DeadCodeFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_user_message()),
            code_viz_api::ApiError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_user_message()),
            code_viz_api::ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_user_message()),
        };

        let body = Json(ErrorResponse {
            error: error_message,
        });

        (status, body).into_response()
    }
}

/// POST /api/analyze - Analyze a repository
///
/// This route is the HTTP equivalent of the Tauri `analyze_repository` command.
/// It uses the EXACT SAME handler from code-viz-api (SSOT).
pub async fn post_analyze(
    Json(req): Json<AnalyzeRequest>,
) -> Result<Json<TreeNode>, WebError> {
    tracing::info!(path = %req.path, request_id = ?req.request_id, "POST /api/analyze");

    let ctx = WebContext::new();
    let fs = RealFileSystem::new();

    // Call the shared SSOT handler (same as Tauri uses)
    let tree = analyze_repository_handler(ctx, fs, req.path, req.request_id).await?;

    Ok(Json(tree))
}

/// POST /api/dead-code - Analyze dead code
///
/// This route is the HTTP equivalent of the Tauri `analyze_dead_code_command` command.
/// It uses the EXACT SAME handler from code-viz-api (SSOT).
pub async fn post_dead_code(
    Json(req): Json<DeadCodeRequest>,
) -> Result<Json<DeadCodeResult>, WebError> {
    tracing::info!(
        path = %req.path,
        min_confidence = %req.min_confidence,
        request_id = ?req.request_id,
        "POST /api/dead-code"
    );

    let ctx = WebContext::new();
    let fs = RealFileSystem::new();
    let git = RealGit::new();

    // Call the shared SSOT handler (same as Tauri uses)
    let result = analyze_dead_code_handler(ctx, fs, git, req.path, req.min_confidence, req.request_id).await?;

    Ok(Json(result))
}

/// GET /health - Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "code-viz-web"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_request_deserialization() {
        let json = r#"{"path": "/test/path", "requestId": "test-123"}"#;
        let req: AnalyzeRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.path, "/test/path");
        assert_eq!(req.request_id, Some("test-123".to_string()));
    }

    #[test]
    fn test_dead_code_request_deserialization() {
        let json = r#"{"path": "/test/path", "minConfidence": 80}"#;
        let req: DeadCodeRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.path, "/test/path");
        assert_eq!(req.min_confidence, 80);
    }

    /// CRITICAL SSOT TEST: Verify web request types match API contracts
    #[test]
    fn test_ssot_request_contract() {
        // Web AnalyzeRequest must serialize to same JSON as Tauri would expect
        let web_req = AnalyzeRequest {
            path: "/test".to_string(),
            request_id: Some("123".to_string()),
        };

        let json = serde_json::to_string(&web_req).unwrap();

        // Should match contract: camelCase fields
        assert!(json.contains("requestId"));
        assert!(!json.contains("request_id")); // snake_case would break contract
    }
}
