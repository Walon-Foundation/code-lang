use std::collections::HashMap;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct SwitchArm {
    pub pattern: Expression,
    pub body: Box<Statement>
}

#[derive(Debug, Clone)]
pub enum StringSegment {
    Literal(String),
    Expr(Box<Expression>),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub default: Option<Box<Expression>>
}

#[derive(Debug, Clone)]
pub enum LetPattern {
    Ident(String),
    Array(Vec<String>),
    Hash(Vec<(String, String)>)
}

// What Go split across Node + Statement interfaces becomes one enum.
#[derive(Debug, Clone,)]
pub enum Statement {
    Let { pattern: LetPattern, value: Expression, line: usize, column: usize, end_line: usize, end_column: usize },
    Const { pattern: LetPattern, value: Expression, line: usize, column: usize, end_line: usize, end_column: usize },
    Return { value: Expression, line: usize, column: usize, end_line: usize, end_column: usize },
    Expression { expr: Expression, line: usize, column: usize, end_line: usize, end_column: usize },
    Block { statements: Vec<Statement>, line: usize, column: usize, end_line: usize, end_column: usize },
    Struct { name: Box<Expression>, field: HashMap<String, Expression>, line: usize, column: usize, end_line: usize, end_column: usize },
    Import { path: String, line: usize, column: usize, end_line: usize, end_column: usize },
    Break { line: usize, column: usize, end_line: usize, end_column: usize },
    Continue { line: usize, column: usize, end_line: usize, end_column: usize },
    Enum { name: String, variant: Vec<String>, line: usize, column: usize, end_line: usize, end_column: usize },
    Pub { statement: Box<Statement>, line: usize, column: usize, end_line: usize, end_column: usize },
}

#[derive(Debug, Clone)]
pub struct ElseIF {
    pub condition: Expression,
    pub consequences: Statement
}

// Same for Expression — one enum, one variant per node kind.
#[derive(Debug, Clone)]
pub enum Expression {
    Ident { value: String, line: usize, column: usize, end_line: usize, end_column: usize },
    Int { value: isize, line: usize, column: usize, end_line: usize, end_column: usize },
    Float { value: f64, line: usize, column: usize, end_line: usize, end_column: usize },
    Char { value: char, line: usize, column: usize, end_line: usize, end_column: usize },
    Boolean { value: bool, line: usize, column: usize, end_line: usize, end_column: usize },
    InterpolatedString {
        parts: Vec<StringSegment>, line: usize, column: usize, end_line: usize, end_column: usize,
    },
    Prefix { op: Token, right: Box<Expression>, line: usize, column: usize, end_line: usize, end_column: usize },
    Infix { left: Box<Expression>, op: Token, right: Box<Expression>, line: usize, column: usize, end_line: usize, end_column: usize },
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
        if_else: Vec<ElseIF>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    StructLiteral {
        name: String,
        fields: HashMap<String, Expression>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    For {
        init: Box<Statement>,
        condition: Box<Expression>,
        post: Box<Statement>,
        body: Box<Statement>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    Update {
        operator: Token,
        target: Box<Expression>,
        prefix: bool,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    Call {
        function: Box<Expression>,
        argument: Vec<Expression>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    Index {
        left: Box<Expression>,
        index: Box<Expression>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    Member {
        object: Box<Expression>,
        property: Box<Expression>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    Function {
        parameter: Vec<Param>,
        body: Box<Statement>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    HashLiteral {
        pair: Vec<(Expression, Expression)>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    Array {
        element: Vec<Expression>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    ForIn {
        key: String,
        value: Option<String>,
        iterable: Box<Expression>,
        body: Box<Statement>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    Switch {
        subject: Box<Expression>,
        arms: Vec<SwitchArm>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    Typeof {
        value: Box<Expression>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    NullCoalesce {
        left: Box<Expression>,
        right: Box<Expression>,
        line: usize,
        column: usize,
        end_line: usize,
        end_column: usize,
    },
    Null { line: usize, column: usize, end_line: usize, end_column: usize },
}