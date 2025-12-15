use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod config_loader;
mod output;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a directory and generate metrics
    Analyze {
        /// Path to the directory to analyze
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output format (json, csv, text)
        #[arg(long, short, default_value = "text")]
        format: String,

        /// Glob patterns to exclude
        #[arg(long, short)]
        exclude: Vec<String>,

        /// Enable verbose logging
        #[arg(long, short)]
        verbose: bool,

        /// Fail if metrics exceed threshold (e.g., "loc=500")
        #[arg(long)]
        threshold: Option<String>,

        /// Write output to file instead of stdout
        #[arg(long, short)]
        output: Option<PathBuf>,

        /// Compare against a baseline report
        #[arg(long)]
        baseline: Option<PathBuf>,
    },
    /// Watch a directory for changes and re-analyze
    Watch {
        /// Path to the directory to watch
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output format (json, text)
        #[arg(long, short, default_value = "text")]
        format: String,

        /// Enable verbose logging
        #[arg(long, short)]
        verbose: bool,
    },
    /// Compare two analysis reports
    Diff {
        /// Path to the old report
        old: PathBuf,

        /// Path to the new report
        new: PathBuf,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubcommand,
    },
}

#[derive(Subcommand)]
enum ConfigSubcommand {
    /// Initialize a new .code-viz.toml configuration file
    Init,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            path,
            format,
            exclude,
            verbose,
            threshold,
            output,
            baseline,
        } => {
            commands::analyze::run(path, format, exclude, verbose, threshold, output, baseline)?;
        }
        Commands::Watch {
            path,
            format,
            verbose,
        } => {
            commands::watch::run(path, format, verbose)?;
        }
        Commands::Diff { old, new } => {
            commands::diff::run(old, new)?;
        }
        Commands::Config { subcommand } => match subcommand {
            ConfigSubcommand::Init => {
                commands::config::run_init()?;
            }
        },
    }

    Ok(())
}