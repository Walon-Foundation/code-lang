use std::collections::HashMap;

use rustyline::{Editor, error::ReadlineError};

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::evaluator::evaluator::Evaluator;
use crate::object::object::{Environment, Object};

pub fn run_repl() {
    let history_path = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(|h| format!("{}/.code_lang_history", h))
        .unwrap_or_else(|_| ".code_lang_history".to_string());

    let mut rl = Editor::<(), _>::new().expect("failed to create line editor");
    let _ = rl.load_history(&history_path);

    let env = Environment::new();
    let mut evaluator = Evaluator { loop_depth: 0, module_cache: HashMap::new() };

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

                let lexer = Lexer::new(input);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();

                let result = evaluator.eval(&program, &env);
                if !matches!(result, Object::Null) {
                    println!("{}", result);
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
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    // stop if parsing failed — don't evaluate a broken tree
    // if !parser.errors.is_empty() {
    //     for err in &parser.errors {
    //         eprintln!("{}", err);
    //     }
    //     return;
    // }

    // set up the evaluator and a fresh global scope
    let env = Environment::new();              // Rc<RefCell<Environment>>
    let mut evaluator = Evaluator { loop_depth: 0, module_cache:HashMap::new() };

    let result = evaluator.eval(&program, &env);

    // show the program's result
    println!("{}", result);          // or however you render Object
}