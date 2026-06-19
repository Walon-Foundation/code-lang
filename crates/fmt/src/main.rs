use clap::{Parser, Subcommand};

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
    Check { files: Vec<String> },
    /// Lint files for style issues
    Lint { files: Vec<String> },
}

fn main() {
    let _cli = Cli::parse();
}
