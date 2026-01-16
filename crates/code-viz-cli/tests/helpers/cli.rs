use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use anyhow::{Context, Result};

/// CLI test harness for running code-viz commands
pub struct CliTest {
    binary_path: PathBuf,
}

impl CliTest {
    pub fn new() -> Self {
        let binary_path = std::env::current_exe()
            .expect("Failed to get current exe")
            .parent()
            .expect("Failed to get parent dir")
            .parent()
            .expect("Failed to get grandparent")
            .join("code-viz-cli");

        Self { binary_path }
    }

    /// Run code-viz analyze command
    pub fn analyze(&self, path: &Path) -> CliCommand {
        CliCommand::new(&self.binary_path, "analyze", path)
    }
}

impl Default for CliTest {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for CLI commands with fluent API
pub struct CliCommand {
    command: Command,
}

impl CliCommand {
    fn new(binary: &Path, subcommand: &str, path: &Path) -> Self {
        let mut command = Command::new(binary);
        command.arg(subcommand);
        command.arg(path);
        Self { command }
    }

    pub fn format(&mut self, format: &str) -> &mut Self {
        self.command.arg("--format").arg(format);
        self
    }

    pub fn duplicates(&mut self) -> &mut Self {
        self.command.arg("--duplicates");
        self
    }

    pub fn duplicates_min_lines(&mut self, min_lines: usize) -> &mut Self {
        self.command.arg("--duplicates");
        self.command.arg("--min-duplicate-lines").arg(min_lines.to_string());
        self
    }

    pub fn hotspots(&mut self, max: usize) -> &mut Self {
        self.command.arg("--hotspots");
        self.command.arg("--max-hotspots").arg(max.to_string());
        self
    }

    pub fn ai_commits(&mut self) -> &mut Self {
        self.command.arg("--ai-commits");
        self
    }

    pub fn dead_code(&mut self) -> &mut Self {
        self.command.arg("--dead-code");
        self
    }

    pub fn output(&mut self, path: &Path) -> &mut Self {
        self.command.arg("--output").arg(path);
        self
    }

    /// Execute the command and return the output
    pub fn run(&mut self) -> Result<Output> {
        self.command
            .output()
            .context("Failed to execute command")
    }

    /// Execute and expect success
    pub fn expect_success(&mut self) -> Result<String> {
        let output = self.run()?;
        if !output.status.success() {
            anyhow::bail!(
                "Command failed with exit code {:?}\nstdout: {}\nstderr: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
