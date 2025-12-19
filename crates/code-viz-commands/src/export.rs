use anyhow::Result;
use code_viz_core::traits::{AppContext, FileSystem};
use code_viz_core::models::AnalysisResult;

/// Export analysis results.
pub async fn export_report(
    _result: AnalysisResult,
    _format: &str,
    _ctx: impl AppContext,
    _fs: impl FileSystem,
) -> Result<()> {
    todo!("Implement export_report orchestration")
}