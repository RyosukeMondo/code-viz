use anyhow::{Context, Result};
use code_viz_core::models::{
    BranchComparison, FileMetricComparison, FileMetrics,
};
use code_viz_core::{metrics, parser};
use code_viz_core::traits::GitProvider;
use code_viz_core::traits::git_provider::{ChangedFile, ChangeType};
use futures::future::try_join_all;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CompareArgs {
    pub spec: String,
    pub base: Option<String>,
    pub head: Option<String>,
}

pub async fn compare_branches(
    args: &CompareArgs,
    git: &impl GitProvider,
) -> Result<BranchComparison> {
    let base = args.base.as_deref().context("Base revision is required")?;
    let head = args.head.as_deref().context("Head revision is required")?;

    let changed_files = git.get_changed_files(base, head).await?;

    let futures = changed_files
        .into_iter()
        .map(|file| process_changed_file(git, file, base, head));

    let files: Vec<FileMetricComparison> = try_join_all(futures).await?;

    Ok(BranchComparison { files })
}

async fn process_changed_file(
    git: &(impl GitProvider + ?Sized),
    file: ChangedFile,
    base: &str,
    head: &str,
) -> Result<FileMetricComparison> {
    let path = file.path;
    let (base_metrics, head_metrics) = match file.change_type {
        ChangeType::Added => {
            let content = git.get_file_content_at_revision(&path, head).await?;
            let metrics = calculate_metrics_for_content(&path, &content).await?;
            (None, Some(metrics))
        }
        ChangeType::Deleted => {
            let content = git.get_file_content_at_revision(&path, base).await?;
            let metrics = calculate_metrics_for_content(&path, &content).await?;
            (Some(metrics), None)
        }
        ChangeType::Modified | ChangeType::Renamed => {
            let base_content_future = git.get_file_content_at_revision(&path, base);
            let head_content_future = git.get_file_content_at_revision(&path, head);

            let (base_content_res, head_content_res) =
                tokio::join!(base_content_future, head_content_future);

            let base_metrics = match base_content_res {
                Ok(content) => calculate_metrics_for_content(&path, &content).await.ok(),
                Err(_) => None,
            };
            let head_metrics = match head_content_res {
                Ok(content) => calculate_metrics_for_content(&path, &content).await.ok(),
                Err(_) => None,
            };

            (base_metrics, head_metrics)
        }
    };

    Ok(FileMetricComparison {
        path: path.clone(),
        base: base_metrics,
        head: head_metrics,
    })
}


async fn calculate_metrics_for_content(
    path: &Path,
    source: &str,
) -> Result<FileMetrics> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .context("File has no extension")?;

    let language_key = match extension {
        "ts" => "typescript",
        "tsx" => "tsx",
        "js" => "javascript",
        "jsx" => "javascript",
        "rs" => "rust",
        "py" => "python",
        "go" => "go",
        "cpp" | "cxx" | "cc" | "hpp" | "h" => "cpp",
        ext => ext,
    };

    let parser = parser::get_parser(language_key)
        .with_context(|| format!("Failed to get parser for language: {}", language_key))?;

    let metrics = metrics::calculate_metrics(path, source, parser.as_ref(), None)
        .with_context(|| format!("Failed to calculate metrics for: {}", path.display()))?;

    Ok(metrics)
}
