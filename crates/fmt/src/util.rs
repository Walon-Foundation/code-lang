use std::path::Path;

use crate::lint_rules::LintSeverity;

pub fn print_caret(
    lines: &[&str],
    path: &Path,
    line: usize,
    col: usize,
    level: &LintSeverity,
    rule: Option<&str>,
    message: &str,
) {
    let level_str = match level {
        LintSeverity::Error => "error",
        LintSeverity::Warning => "warning",
        LintSeverity::Info => "info",
    };

    let rule_str = match rule {
        Some(r) => format!("[{}] ", r),
        None => String::new(),
    };

    eprintln!(
        "{}:{}:{}: {}: {}{}",
        path.display(),
        line,
        col,
        level_str,
        rule_str,
        message
    );

    // source line (1-indexed, guard against out-of-bounds)
    if line > 0 && line <= lines.len() {
        let src_line = lines[line - 1];
        let line_num = line.to_string();
        let pad = " ".repeat(line_num.len());

        eprintln!("{} |", pad);
        eprintln!("{} | {}", line_num, src_line);

        // caret — col is 1-indexed; guard so we never go negative
        let arrow_offset = if col > 0 { col - 1 } else { 0 };
        eprintln!("{} | {}^", pad, " ".repeat(arrow_offset));
    }
}
