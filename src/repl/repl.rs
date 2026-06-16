use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::evaluator::evaluator::Evaluator;
use crate::object::object::{Environment, Object};

pub fn run_repl() {
    let stdin = io::stdin();
    let env = Environment::new();
    let mut evaluator = Evaluator { loop_depth: 0, module_cache: HashMap::new() };

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }

        let input = line.trim().to_string();
        if input.is_empty() { continue; }
        if input == "exit" || input == "quit" { break; }

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let result = evaluator.eval(&program, &env);
        if !matches!(result, Object::Null) {
            println!("{}", result);
        }
    }
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