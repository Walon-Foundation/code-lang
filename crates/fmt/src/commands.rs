use std::{fs, path::PathBuf, process};

use anyhow::{Result, bail};
use code_lang::{
    analysis::scope::ScopeAnalyzer, ast::walk::walk_program, lexer::lexer::Lexer,
    parser::parser::Parser,
};

use crate::lint_rules::{
    ConstReassignment, DeadCode, EmptyBlock, LintFix, LintRule, LintSeverity, ShadowedBinding,
    UndefinedVariable, UnusedImport, UnusedVariable,
};
use crate::util::print_caret;

pub fn check_file(files: &[PathBuf]) -> Result<()> {
    let mut total_errors = 0;

    for path in files {
        let ext_ok = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("cl"))
            .unwrap_or(false);

        if !ext_ok {
            bail!("expect a .cl file, got: {}", path.display())
        }

        let src = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}: cannot read file: {}", path.display(), e);
                total_errors += 1;
                continue;
            }
        };

        let lines: Vec<&str> = src.lines().collect();
        let lexer = Lexer::new(src.clone());
        let mut parser = Parser::new(lexer);
        parser.parse_program();

        if parser.errors.is_empty() {
            println!("{}: ok", path.display());
        } else {
            for err in &parser.errors {
                print_caret(
                    &lines,
                    path,
                    err.line,
                    err.column,
                    &LintSeverity::Error,
                    None,
                    &err.message,
                );
            }
            total_errors += parser.errors.len();
        }
    }

    if total_errors > 0 {
        eprintln!("\n{} error(s) found", total_errors);
        process::exit(1);
    }

    Ok(())
}

pub fn lint_file(files: &[PathBuf], fix_mode: bool) -> Result<()> {
    let mut total = 0;

    for path in files {
        let ext_ok = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("cl"))
            .unwrap_or(false);

        if !ext_ok {
            bail!("expect a .cl file, got: {}", path.display())
        }

        let src = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}: cannot read file: {}", path.display(), e);
                total += 1;
                continue;
            }
        };

        let lines: Vec<&str> = src.lines().collect();
        let lexer = Lexer::new(src.clone());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        if !parser.errors.is_empty() {
            for err in &parser.errors {
                print_caret(
                    &lines,
                    path,
                    err.line,
                    err.column,
                    &LintSeverity::Error,
                    None,
                    &err.message,
                );
            }
            total += parser.errors.len();
            continue;
        }

        // build scope tree once — shared by UndefinedVariable
        let scope_tree = ScopeAnalyzer::analyze(&program);

        let mut unused_import = UnusedImport::new();
        let mut shadowed = ShadowedBinding::new();
        let mut unused_var = UnusedVariable::new();
        let mut const_reassign = ConstReassignment::new();
        let mut dead_code = DeadCode::new();
        let mut empty_block = EmptyBlock::new();
        let mut undefined_var = UndefinedVariable::new(scope_tree);

        walk_program(&mut unused_import, &program);
        walk_program(&mut shadowed, &program);
        walk_program(&mut unused_var, &program);
        walk_program(&mut const_reassign, &program);
        walk_program(&mut dead_code, &program);
        walk_program(&mut empty_block, &program);
        walk_program(&mut undefined_var, &program);

        let all_diags: Vec<_> = unused_import
            .diagnostic()
            .iter()
            .chain(shadowed.diagnostic())
            .chain(unused_var.diagnostic())
            .chain(const_reassign.diagnostic())
            .chain(dead_code.diagnostic())
            .chain(empty_block.diagnostic())
            .chain(undefined_var.diagnostic())
            .collect();

        if fix_mode {
            let fixable: Vec<&LintFix> = all_diags.iter().filter_map(|d| d.fix.as_ref()).collect();

            if !fixable.is_empty() {
                apply_fixes(path, src.clone(), fixable);
                println!("{}: applied fix(es)", path.display());
            }

            // still print unfixable diagnostics
            for diag in all_diags.iter().filter(|d| d.fix.is_none()) {
                print_caret(
                    &lines,
                    path,
                    diag.line,
                    diag.colum,
                    &diag.severity,
                    Some(diag.rule),
                    &diag.message,
                );
            }
        } else {
            for diag in &all_diags {
                print_caret(
                    &lines,
                    path,
                    diag.line,
                    diag.colum,
                    &diag.severity,
                    Some(diag.rule),
                    &diag.message,
                );
            }
            total += all_diags.len()
        }
    }

    if total > 0 {
        process::exit(1);
    }

    Ok(())
}

fn apply_fixes(path: &PathBuf, src: String, fixes: Vec<&LintFix>) {
    let mut lines: Vec<String> = src.split('\n').map(|l| l.to_string()).collect();

    // sort descending by line then col so bottom-up edits don't shift earlier positions
    let mut sorted = fixes;
    sorted.sort_by(|a, b| {
        b.start_line
            .cmp(&a.start_line)
            .then(b.start_col.cmp(&a.start_col))
    });

    for fix in sorted {
        let line_idx = fix.start_line.saturating_sub(1);
        if line_idx >= lines.len() {
            continue;
        }

        if fix.start_col == 0 && fix.end_col == usize::MAX {
            // whole-line deletion
            lines.remove(line_idx);
        } else {
            let line = &lines[line_idx];
            let start = fix.start_col.min(line.len());
            let end = fix.end_col.min(line.len());
            let new_line = format!("{}{}{}", &line[..start], fix.replacement, &line[end..]);
            lines[line_idx] = new_line;
        }
    }

    let result = lines.join("\n");
    if let Err(e) = fs::write(path, result) {
        eprintln!("{}: failed to write fixes: {}", path.display(), e);
    }
}
