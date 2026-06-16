use std::collections::HashMap;

use crate::token::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// What Go split across Node + Statement interfaces becomes one enum.
#[derive(Debug, Clone,)]
pub enum Statement {
    Let { name: String, value: Expression, line: usize, column: usize },
    Const {name: String, value: Expression, line:usize, column:usize},
    Return { value: Expression, line: usize, column: usize },
    Expression { expr: Expression, line: usize, column: usize },
    Block { statements: Vec<Statement>, line: usize, column: usize },
    Struct { name: Box<Expression>, field:HashMap<String, Expression>},
    Import { path:String,  },
    Break,
    Continue
}

#[derive(Debug, Clone)]
pub struct ElseIF {
    pub condition: Expression,
    pub consequences: Statement
}

// Same for Expression — one enum, one variant per node kind.
#[derive(Debug, Clone)]
pub enum Expression {
    Ident { value: String, line: usize, column: usize },
    Int { value: isize, line: usize, column: usize },
    Float { value: f64, line: usize, column: usize },
    Char {value:char, line: usize, column: usize },
    Boolean { value: bool, line: usize, column: usize },
    StringLit { value: String, line: usize, column: usize },
    Prefix { op: Token, right: Box<Expression>, line: usize, column: usize },
    Infix { left: Box<Expression>, op: Token, right: Box<Expression>, line: usize, column: usize },
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,       
        alternative: Option<Box<Statement>>,
        if_else: Vec<ElseIF>,
        line: usize, column: usize,
    },
    StructLiteral {
        name: String,
        fields: HashMap<String, Expression>,
        line:usize,
        column: usize
    },
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
        line: usize,
        column: usize
    },
    For {
        init: Box<Statement>,
        condition: Box<Expression>,
        post: Box<Statement>,
        body: Box<Statement>,
        line: usize,
        column: usize
    },
    Update {
        operator: Token,
        target: Box<Expression>,
        prefix: bool,
        line: usize,
        column: usize
    },
    Call {
        function: Box<Expression>,
        argument: Vec<Expression>,
        line: usize,
        column: usize
    },
    Index {
        left:Box<Expression>,
        index: Box<Expression>,
        line: usize,
        column: usize
    },
    Member {
        object: Box<Expression>,
        property: Box<Expression>,
        line: usize,
        column: usize 
    },
    Function {
        parameter: Vec<Expression>,
        body: Box<Statement>,
        line: usize,
        column: usize  
    },
    HashLiteral {
        pair: Vec<(Expression, Expression)>,
        line: usize,
        column: usize
    },
    Array {
        element: Vec<Expression>,
        line: usize,
        column: usize
    }
}