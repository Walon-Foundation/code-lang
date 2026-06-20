use std::{fs, path::PathBuf, process};

use anyhow::{Result, bail};
use code_lang::{ast::walk::walk_program, lexer::lexer::Lexer, parser::parser::Parser};

use crate::lint_rules::{LintRule, LintSeverity, ShadowedBinding, UnusedImport};

pub fn check_file(files: &[PathBuf]) -> Result<()> {
    let mut total_errors = 0;

    for path in files {
        let ext_ok = path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("cl"))
            .unwrap_or(false);

        if !ext_ok {
            bail!("expect a .cl file, got: {}", path.display())
        };
        
        let src = match fs::read_to_string(path){
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}: cannot read file: {}", path.display(), e);
                total_errors += 1;
                continue;
            }
        };

        let lexer  = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        parser.parse_program();

        if parser.errors.is_empty() {
            
        }else {
            for err in &parser.errors {
                eprintln!(
                    "{}:{}:{}: error: {}",
                   path.display(), err.line, err.column, err.message 
                )
            }
            total_errors += parser.errors.len();
        }
    }

    if total_errors > 0 {
        eprintln!( "\n{} error(s) found", total_errors);
        process::exit(1);
    }

    Ok(())
}



pub fn lint_file(files: &[PathBuf]) -> Result<()> {
    let mut total = 0;

    for path in files {
        let ext_ok = path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("cl"))
            .unwrap_or(false);

        if !ext_ok {
            bail!("expect a .cl file, got: {}", path.display())
        };
        
        let src = fs::read_to_string(path)?;
        let lexer  = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        //run all rules
        let mut unused_import = UnusedImport::new();
        let mut shadowed = ShadowedBinding::new();

        walk_program(&mut unused_import, &program);
        walk_program(&mut shadowed, &program);

        let all_diags:Vec<_> = unused_import.diagnostic().iter().chain(shadowed.diagnostic().iter()).collect();

        for diag in &all_diags {
            let level = match diag.severity {
                LintSeverity::Error   => "error",
                LintSeverity::Warning => "warning",
                LintSeverity::Info    => "info",
            };
            eprintln!(
                "{}:{}:{}: [{}] [{}] {}",
                path.display(), diag.line, diag.colum,
                level, diag.rule, diag.message
            );
        }
       
        total += all_diags.len()
    }

    
    if total > 0 { std::process::exit(1); }
    
    Ok(())
}