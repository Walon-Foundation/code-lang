use anyhow::{Result, bail};
use clap::Parser;
use code_lang::repl::repl::{execute, run_repl};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(name = "code-lang")]
#[command(about = "Interpreter for the code-lang language")]
#[command(version)]
struct Cli {
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.file {
        Some(path) => run_file(&path)?,
        None => run_repl(),
    }
    Ok(())
}

fn run_file(path: &Path) -> Result<()> {
    let ext_ok = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("cl"))
        .unwrap_or(false);

    if !ext_ok {
        bail!("expect a .cl file, got: {}", path.display())
    }

    let file = fs::read_to_string(path)?;
    execute(file);
    Ok(())
}
