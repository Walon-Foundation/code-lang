use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::evaluator::evaluator::Evaluator;
use crate::object::object::Environment;

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
    let evaluator = Evaluator { loop_depth: 0 };

    let result = evaluator.eval(&program, &env);

    // show the program's result
    println!("{}", result);          // or however you render Object
}