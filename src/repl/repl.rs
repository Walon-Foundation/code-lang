use rustyline::{Editor, error::ReadlineError};

use crate::evaluator::evaluator::Evaluator;
use crate::lexer::lexer::Lexer;
use crate::object::object::{Environment, Object};
use crate::parser::parser::Parser;

fn get_hint(message: &str) -> Option<&'static str> {
    if message.contains("type mismatch: INTEGER") && message.contains("STRING") {
        Some("use fmt.to_str() to convert a number to string")
    } else if message.contains("type mismatch: STRING") && message.contains("INTEGER") {
        Some("use fmt.to_int() or fmt.to_float() to parse the string")
    } else if message.contains("type mismatch: STRING") && message.contains("FLOAT") {
        Some("use fmt.to_float() to parse the string")
    } else if message.contains("identifier not found") {
        Some("declare it first with 'let name = value'")
    } else if message.contains("cannot reassign constant") {
        Some("use 'let' instead of 'const' if the value needs to change")
    } else if message.contains("wrong number of arguments") {
        Some("check the function signature for the correct parameter count")
    } else if message.contains("out of range") {
        Some("use arrays.len() or strings.len() to check the length before indexing")
    } else if message.contains("not a function") {
        Some("check that the variable holds a function, not a value")
    } else if message.contains("division by zero") {
        Some("guard with 'if divisor != 0' before dividing")
    } else if message.contains("not found in hash")
        || message.contains("key") && message.contains("not found")
    {
        Some("use hash.has_key(h, key) to check before accessing")
    } else if message.contains("maximum call depth exceeded") {
        Some("check for infinite recursion; add a base case to your function")
    } else if message.contains("integer overflow") {
        Some("the value exceeded the 64-bit integer range")
    } else if message.contains("break outside of loop")
        || message.contains("continue outside of loop")
    {
        Some("break and continue are only valid inside while or for loops")
    } else {
        None
    }
}

fn show_error(source: &str, message: &str, line: usize, column: usize) {
    eprintln!("error: {}", message);

    let lines: Vec<&str> = source.lines().collect();
    if line > 0 && line <= lines.len() {
        let src_line = lines[line - 1];
        let line_str = line.to_string();
        let gutter = line_str.len();

        eprintln!(" {}--> {}:{}", " ".repeat(gutter), line, column);
        eprintln!(" {} |", " ".repeat(gutter));
        eprintln!(" {} | {}", line_str, src_line);

        let caret_pos = column.saturating_sub(1);
        eprintln!(" {} | {}^", " ".repeat(gutter), " ".repeat(caret_pos));
    }

    if let Some(hint) = get_hint(message) {
        eprintln!("hint: {}", hint);
    }
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
    evaluator.register_globals(&env);

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let input = line.trim().to_string();
                if input.is_empty() {
                    continue;
                }
                rl.add_history_entry(&input).ok();

                if input == "exit()" || input == "exit" || input == "quit" {
                    println!("Exiting...");
                    break;
                }

                let lexer = Lexer::new(input.clone());
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();

                if !parser.errors.is_empty() {
                    for err in &parser.errors {
                        show_error(&input, &err.message, err.line, err.column);
                        if !evaluator.call_stack.is_empty() {
                            for err in evaluator.call_stack.iter().clone() {
                                show_error(&input, &err.name, err.call_line, err.call_column);
                            }
                        }
                    }
                    continue;
                }


                let result = evaluator.eval(&program, &env);
                match result {
                    Object::Error {
                        ref message,
                        line,
                        column,
                    } => {
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
            show_error(&input, &err.message, err.line, err.column);
        }
        std::process::exit(1);
    }

    let env = Environment::new();
    let mut evaluator = Evaluator::new();
    evaluator.register_globals(&env);
    let result = evaluator.eval(&program, &env);

    match result {
        Object::Error {
            ref message,
            line,
            column,
        } => {
            show_error(&input, message, line, column);
            if !evaluator.call_stack.is_empty(){
                for e in evaluator.call_stack.iter().clone() {
                    show_error(&input, &e.name, e.call_line, e.call_column);
                }
            }
            std::process::exit(1);
        }
        Object::Null => {}
        other => println!("{}", other),
    }
}
