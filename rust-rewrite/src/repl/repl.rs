use crate::lexer::lexer::{Lexer};

pub fn execute(input:String) {
    let lexer = Lexer::new(input);
    println!("{:?}", lexer)
}

