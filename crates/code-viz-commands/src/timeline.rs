use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use code_viz_core::traits::GitProvider;
use code_viz_core::models::FileMetrics;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct TimelineResult {
    pub timestamps: Vec<DateTime<Utc>>,
    pub metrics: Vec<FileMetrics>,
}

pub async fn generate_timeline(
    file_path: &Path,
    git: &impl GitProvider,
) -> Result<TimelineResult> {
    let mut history = git.get_history(file_path).await?;
    history.reverse();
    let mut timestamps = Vec::new();
    let mut metrics = Vec::new();

    for commit in history {
        let content = git.get_file_content_at_revision(file_path, &commit.sha).await?;
        let language_key = "rust";
        let parser = code_viz_core::parser::get_parser(language_key)
            .with_context(|| format!("Failed to get parser for language: {}", language_key))?;
        let file_metrics = code_viz_core::metrics::calculate_metrics(file_path, &content, parser.as_ref(), None)?;
        
        timestamps.push(DateTime::from_timestamp(commit.timestamp, 0).unwrap_or_default());
        metrics.push(file_metrics);
    }

    Ok(TimelineResult {
        timestamps,
        metrics,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use code_viz_core::mocks::MockGit;
    use code_viz_core::traits::Commit;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_generate_timeline() {
        let file_path = PathBuf::from("test.rs");
        let mock_git = MockGit::new()
            .with_commit(Commit {
                sha: "sha1".to_string(),
                author: "author1".to_string(),
                timestamp: 1672531200,
                message: "Initial commit".to_string(),
            })
            .with_commit(Commit {
                sha: "sha2".to_string(),
                author: "author2".to_string(),
                timestamp: 1675209600,
                message: "Second commit".to_string(),
            })
            .with_file_content("sha1", "test.rs", "fn main() {}")
            .with_file_content("sha2", "test.rs", "fn main() {\n    println!(\"hello\");\n}");

        let result = generate_timeline(&file_path, &mock_git).await.unwrap();

        assert_eq!(result.timestamps.len(), 2);
        assert_eq!(result.metrics.len(), 2);
        assert_eq!(result.metrics[0].loc, 3);
        assert_eq!(result.metrics[1].loc, 1);
    }
}
