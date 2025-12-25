use anyhow::Result;
use code_viz_core::traits::GitProvider;
use std::path::PathBuf;

pub async fn run(
    file: PathBuf,
    _since: Option<String>,
    git: impl GitProvider,
) -> Result<()> {
    let result = code_viz_commands::timeline::generate_timeline(&file, &git).await?;
    let output = serde_json::to_string_pretty(&result)?;
    println!("{}", output);
    Ok(())
}
