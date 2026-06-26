mod commands;
mod lint_rules;
mod util;
mod formatter;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{check_file, format_file, lint_file};

#[derive(Parser)]
#[command(name = "code-lang-fmt")]
#[command(about = "Formatter, checker, and linter for code-lang (.cl) files")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    files: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check files for syntax errors without modifying them
    Check { files: Vec<PathBuf> },
    /// Lint files for style issues
    Lint {
        files: Vec<PathBuf>,
        #[arg(long)]
        fix: bool,
    },

    Format {
        files: Vec<PathBuf>,

        #[arg(long)]
        stdout:bool
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Check { files }) => {
            check_file(&files)?;
        }
        Some(Commands::Lint { files, fix }) => {
            lint_file(&files, fix)?;
        }
        Some(Commands::Format { files, stdout }) => {
            format_file(&files, stdout)?;
        },
        None => {}
    }
    Ok(())
}
