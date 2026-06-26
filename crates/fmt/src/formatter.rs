use code_lang::ast::ast::{Expression, Program, Statement};

pub struct Formatter {
    indent_level: usize,
    output: String
}

const IDENT:&str = "    "; // 4 spaces

impl Formatter {
    pub fn format(program: &Program){ }
    pub fn fmt_statement(&mut self, stmt:&Statement){}
    pub fn fmt_expression(&mut self, expr:&Expression){}
}