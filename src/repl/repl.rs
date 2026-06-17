use rustyline::{Editor, error::ReadlineError};

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::evaluator::evaluator::Evaluator;
use crate::object::object::{Environment, Object};

fn show_error(source: &str, message: &str, line: usize, column: usize) {
    eprintln!("error: {}", message);

    let lines: Vec<&str> = source.lines().collect();
    if line == 0 || line > lines.len() {
        return;
    }

    let src_line = lines[line - 1];
    let line_str = line.to_string();
    let gutter = line_str.len();

    eprintln!(" {}--> {}:{}", " ".repeat(gutter), line, column);
    eprintln!(" {} |", " ".repeat(gutter));
    eprintln!(" {} | {}", line_str, src_line);

    let caret_pos = column.saturating_sub(1);
    eprintln!(" {} | {}^", " ".repeat(gutter), " ".repeat(caret_pos));
}

pub fn run_repl() {
    let history_path = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(|h| format!("{}/.code_lang_history", h))
        .unwrap_or_else(|_| ".code_lang_history".to_string());

    let mut rl = Editor::<(), _>::new().expect("failed to create line editor");
    let _ = rl.load_history(&history_path);

    let env = Environment::new();
    let mut evaluator = Evaluator::new();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let input = line.trim().to_string();
                if input.is_empty() { continue; }
                rl.add_history_entry(&input).ok();

                if input == "exit()" || input == "exit" || input == "quit" {
                    println!("Exiting...");
                    break;
                }

                let lexer = Lexer::new(input.clone());
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();

                let result = evaluator.eval(&program, &env);
                match result {
                    Object::Error { ref message, line, column } => {
                        show_error(&input, message, line, column);
                    }
                    Object::Null => {}
                    other => println!("{}", other),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting...");
                break;
            }
            Err(ReadlineError::Eof) | Err(_) => break,
        }
    }

    let _ = rl.save_history(&history_path);
}

pub fn execute(input: String) {
    let lexer = Lexer::new(input.clone());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        for err in &parser.errors {
            eprintln!("parse error: {}", err);
        }
        std::process::exit(1);
    }

    let env = Environment::new();
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval(&program, &env);

    match result {
        Object::Error { ref message, line, column } => {
            show_error(&input, message, line, column);
            std::process::exit(1);
        }
        Object::Null => {}
        other => println!("{}", other),
    }
}
