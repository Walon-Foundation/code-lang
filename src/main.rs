mod token;
mod lexer;
mod repl;
mod parser;
mod ast;
mod object;
mod evaluator;

use std::{ fs, path::{Path, PathBuf}};
use anyhow::{Result, bail};
use repl::repl::execute;
use clap::Parser;

#[derive(Parser)]
#[command(name = "code-lang")]
#[command(about = "Interpreter for the code-lang language")]
struct Cli {
    file: PathBuf
}

fn main() -> Result<()>{
    let cli = Cli::parse();
    run_file(&cli.file)?;

    Ok(())
}

fn run_file(path: &Path) -> Result<()>  {
    let ext_ok = path.extension()
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
