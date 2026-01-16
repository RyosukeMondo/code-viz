use anyhow::Result;
use clap::Parser;
use cli_table::{print_stdout, Table};
use code_viz_commands::compare::{compare_branches, CompareArgs};
use code_viz_core::context::RealGit;
use colored::Colorize;

#[derive(Parser, Debug)]
pub struct CompareCommand {
    /// The base and head to compare, e.g. main..HEAD
    #[clap(required = true)]
    pub spec: String,
}

impl From<&CompareCommand> for CompareArgs {
    fn from(args: &CompareCommand) -> Self {
        let parts: Vec<&str> = args.spec.split("..").collect();
        let base = parts.first().map(|s| s.to_string());
        let head = parts.get(1).map(|s| s.to_string());
        CompareArgs {
            base,
            head,
            spec: args.spec.clone(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CompareError {
    #[error(transparent)]
    Core(#[from] anyhow::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub async fn handle_compare(args: &CompareCommand) -> Result<(), CompareError> {
    let git = RealGit::new();
    let compare_args: CompareArgs = args.into();
    let comparison = compare_branches(&compare_args, &git).await?;

    let mut table_data = Vec::new();

    for file in comparison.files {
        let path = file.path.to_string_lossy().to_string();
        let (base_loc, head_loc) = (
            file.base.as_ref().map(|m| m.loc),
            file.head.as_ref().map(|m| m.loc),
        );

        let loc_change = match (base_loc, head_loc) {
            (Some(b), Some(h)) => format_change(b, h),
            (None, Some(h)) => format!("+{}", h),
            (Some(b), None) => format!("-{}", b),
            (None, None) => "-".to_string(),
        };

        table_data.push(vec![path, loc_change]);
    }

    let table = table_data.table().title(vec![
        "File".bold().to_string(),
        "LoC Change".bold().to_string(),
    ]);

    print_stdout(table)?;

    Ok(())
}

fn format_change(base: usize, head: usize) -> String {
    if head > base {
        format!("+{}", head - base)
    } else if base > head {
        format!("-{}", base - head)
    } else {
        "0".to_string()
    }
}
