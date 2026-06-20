use std::collections::HashMap;

use crate::ast::ast::Program;
use crate::{
    ast::ast::{ElseIF, Expression, LetPattern, Statement, StringSegment, SwitchArm},
    lexer::lexer::Lexer,
    token::token::{StringPart, Token, TokenType},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedences {
    Lowest,
    Assign,
    NullCoalesce,
    Or,
    And,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Postfix,
    Call,
    Index,
    Member,
}

//Error Kind
#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken { expected: TokenType, got: Token },
    UnexpectedEOF,
    UnclosedDelimiter { open: TokenType },
    InvalidLiteral { raw: String },
    MissingExpression,
    IllegalKeyword { keyword: Token },
    InterpolationError { source: Vec<ParseError> },
    Other,
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct Parser {
    l: Lexer,
    pub errors: Vec<ParseError>,
    cur_token: Token,
    peak_token: Token,
}

impl Parser {
    fn precedence_of(tok: &TokenType) -> Precedences {
        match tok {
            TokenType::EQ | TokenType::NOTEQ => Precedences::Equals,
            TokenType::LT
            | TokenType::GT
            | TokenType::LessThanEqual
            | TokenType::GreaterThanEqual => Precedences::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedences::Sum,
            TokenType::SLASH | TokenType::Asterisk | TokenType::Rem => Precedences::Product,
            TokenType::And => Precedences::And,
            TokenType::Or => Precedences::Or,
            TokenType::NullCoalesce => Precedences::NullCoalesce,
            TokenType::LParan => Precedences::Call,
            TokenType::LBracket => Precedences::Index,
            TokenType::Dot => Precedences::Member,
            TokenType::Assign => Precedences::Assign,
            _ => Precedences::Lowest,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peak_token.clone();
        self.peak_token = self.l.next_token()
    }

    fn peak_precedence(&self) -> Precedences {
        Self::precedence_of(&self.peak_token.token_type)
    }

    fn cur_precedence(&self) -> Precedences {
        Self::precedence_of(&self.cur_token.token_type)
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        self.next_token();

        let value = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::Semicolon) {
            return None;
        }

        // cur_token is ';'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Return {
            value,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_const_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let pattern = if self.peak_token_is(&TokenType::LBracket) {
            self.next_token();
            let mut names = Vec::new();
            while !self.peak_token_is(&TokenType::RBracket) {
                self.next_token();
                let name = match self.cur_token.token_type.clone() {
                    TokenType::Ident(v) => v,
                    _ => {
                        self.errors.push(ParseError {
                            kind: ParseErrorKind::UnexpectedToken {
                                expected: TokenType::Ident(String::new()),
                                got: self.cur_token.clone(),
                            },
                            message: format!(
                                "expected identifier in array destructuring pattern, got {:?}",
                                self.cur_token.token_type
                            ),
                            line: self.cur_token.line,
                            column: self.cur_token.column,
                        });
                        return None;
                    }
                };
                names.push(name);
                if self.peak_token_is(&TokenType::Comma) {
                    self.next_token();
                }
            }
            if !self.expect_peak(TokenType::RBracket) {
                return None;
            }
            LetPattern::Array(names)
        } else if self.peak_token_is(&TokenType::LBrace) {
            self.next_token();
            let mut pairs = Vec::new();
            while !self.peak_token_is(&TokenType::RBrace) {
                self.next_token();
                let key = match self.cur_token.token_type.clone() {
                    TokenType::Ident(v) => v,
                    _ => {
                        self.errors.push(ParseError {
                            kind: ParseErrorKind::UnexpectedToken {
                                expected: TokenType::Ident(String::new()),
                                got: self.cur_token.clone(),
                            },
                            message: format!(
                                "expected identifier in hash destructuring pattern, got {:?}",
                                self.cur_token.token_type
                            ),
                            line: self.cur_token.line,
                            column: self.cur_token.column,
                        });
                        return None;
                    }
                };
                let alias = if self.peak_token_is(&TokenType::Colon) {
                    self.next_token();
                    self.next_token();
                    match self.cur_token.token_type.clone() {
                        TokenType::Ident(v) => v,
                        _ => {
                            self.errors.push(ParseError {
                                kind: ParseErrorKind::UnexpectedToken {
                                    expected: TokenType::Ident(String::new()),
                                    got: self.cur_token.clone(),
                                },
                                message: format!(
                                    "expected identifier for destructuring alias, got {:?}",
                                    self.cur_token.token_type
                                ),
                                line: self.cur_token.line,
                                column: self.cur_token.column,
                            });
                            return None;
                        }
                    }
                } else {
                    key.clone()
                };
                pairs.push((key, alias));
                if self.peak_token_is(&TokenType::Comma) {
                    self.next_token();
                }
            }
            if !self.expect_peak(TokenType::RBrace) {
                return None;
            }
            LetPattern::Hash(pairs)
        } else {
            if !self.expect_peak_ident() {
                return None;
            }
            let name = match &self.cur_token.token_type {
                TokenType::Ident(value) => value.clone(),
                _ => return None,
            };
            LetPattern::Ident(name)
        };

        if !self.expect_peak(TokenType::Assign) {
            return None;
        }
        self.next_token();
        let value = self.parse_expression(Precedences::Lowest)?;
        if self.peak_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        // cur_token is ';' or last token of value expression
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Const {
            pattern,
            value,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_integer_literal(&self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let value = match self.cur_token.token_type {
            TokenType::Int(value) => value,
            _ => return None,
        };

        let end_line = line;
        let end_column = column + format!("{}", value).len();
        Some(Expression::Int {
            value,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_float_literal(&self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let value = match self.cur_token.token_type {
            TokenType::Float(value) => value,
            _ => return None,
        };

        let end_line = line;
        let end_column = column + 1; // approximate; raw lexeme not stored
        Some(Expression::Float {
            value,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_while_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        if !self.expect_peak(TokenType::LParan) {
            return None;
        }

        self.next_token();

        let condition = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::RParen) {
            return None;
        }

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        // cur_token is '}' after parse_block_statement
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::While {
            condition: Box::from(condition),
            body: Box::from(body),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_for_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        // '(' already consumed by caller, cur = init statement start
        let init = self.parse_statement()?;

        self.next_token(); // move past ';' to condition

        let condition = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::Semicolon) {
            return None;
        }

        self.next_token(); // move past ';' to post

        let post_exp = self.parse_expression(Precedences::Lowest)?;
        // cur_token is last token of post_exp here
        let post = Statement::Expression {
            expr: post_exp,
            line,
            column,
            end_line: self.cur_token.line,
            end_column: self.cur_token.column + 1,
        };

        if !self.expect_peak(TokenType::RParen) {
            return None;
        }

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        // cur_token is '}' after parse_block_statement
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::For {
            init: Box::from(init),
            condition: Box::from(condition),
            post: Box::from(post),
            body: Box::from(body),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_block_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let mut statements: Vec<Statement> = Vec::new();

        self.next_token(); // advance past '{'

        while !self.cur_token_is(&TokenType::RBrace) && !self.cur_token_is(&TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        // cur_token is '}' (or EOF)
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Block {
            statements,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let exp = self.parse_expression(Precedences::Lowest);
        if !self.expect_peak(TokenType::RParen) {
            return None;
        }

        exp
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        let op = self.cur_token.clone();
        let precedence = self.cur_precedence();

        self.next_token();
        let right = self.parse_expression(precedence)?;

        // cur_token is last token of right operand
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::Infix {
            left: Box::from(left),
            op,
            right: Box::from(right),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_update_prefix_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        let operator = self.cur_token.clone();

        self.next_token();
        let target = self.parse_expression(Precedences::Prefix)?;

        // cur_token is last token of target
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::Update {
            operator,
            target: Box::from(target),
            prefix: true,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_update_postfix_expression(&self, left: Expression) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        // cur_token is '++' or '--' (2 chars)
        let end_line = line;
        let end_column = column + 2;
        Some(Expression::Update {
            operator: self.cur_token.clone(),
            target: Box::from(left),
            prefix: false,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        let op = self.cur_token.clone();

        self.next_token();
        let right = self.parse_expression(Precedences::Prefix)?;

        // cur_token is last token of right operand
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::Prefix {
            op,
            right: Box::from(right),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        match &self.cur_token.token_type {
            TokenType::Ident(_) => self.parse_identifier(), // IDENT
            TokenType::Int(_) => self.parse_integer_literal(), // INT
            TokenType::Float(_) => self.parse_float_literal(), // FLOAT
            TokenType::InterpolatedString(_) => self.parse_string_literal(), // STRING
            TokenType::Char(_) => self.parse_char_literal(), // CHAR
            TokenType::True | TokenType::False => self.parse_boolean(), // TRUE, FALSE
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(), // BANG, MINUS
            TokenType::Inc | TokenType::Dec => self.parse_update_prefix_expression(), // INC, DEC (++x, --x)
            TokenType::LParan => self.parse_grouped_expression(),                     // LPAREN
            TokenType::If => self.parse_if_expression(),                              // IF
            TokenType::Function => self.parse_function_literal(),                     // FUNCTION
            TokenType::LBracket => self.parse_array_literal(),                        // LBRACKET
            TokenType::LBrace => self.parse_hash_literal(),                           // LBRACE
            TokenType::For => {
                if !self.expect_peak(TokenType::LParan) {
                    return None;
                }
                self.next_token(); // cur = first token inside '('
                if matches!(self.cur_token.token_type, TokenType::Let | TokenType::Const) {
                    self.parse_for_expression()
                } else {
                    self.parse_for_in_expression()
                }
            } // FOR
            TokenType::While => self.parse_while_expression(),                        // WHILE
            TokenType::Switch => self.parse_switch_expression(),
            TokenType::Typeof => self.parse_typeof_expression(),
            TokenType::Null => {
                let line = self.cur_token.line;
                let column = self.cur_token.column;
                // 'null' is 4 chars
                Some(Expression::Null {
                    line,
                    column,
                    end_line: line,
                    end_column: column + 4,
                })
            }
            _ => {
                if self.cur_token_is(&TokenType::EOF) {
                    self.errors.push(ParseError {
                        kind: ParseErrorKind::UnexpectedEOF,
                        message: "unexpected end of file".to_string(),
                        line: self.cur_token.line,
                        column: self.cur_token.column,
                    });
                } else {
                    self.errors.push(ParseError {
                        kind: ParseErrorKind::MissingExpression,
                        message: format!("no expression found for {:?}", self.cur_token.token_type),
                        line: self.cur_token.line,
                        column: self.cur_token.column,
                    });
                }
                None
            }
        }
    }

    fn parse_switch_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        // switch (subject) { arms }
        if !self.expect_peak(TokenType::LParan) {
            return None;
        }
        self.next_token(); // cur = subject expression start

        let subject = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::RParen) {
            return None;
        }
        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }

        let mut arms = Vec::new();

        while !self.peak_token_is(&TokenType::RBrace) && !self.peak_token_is(&TokenType::EOF) {
            self.next_token(); // cur = pattern expression start

            let pattern = self.parse_expression(Precedences::Lowest)?;

            if !self.expect_peak(TokenType::FatArrow) {
                return None;
            }

            let body = if self.peak_token_is(&TokenType::LBrace) {
                self.next_token(); // cur = '{'
                self.parse_block_statement()?
            } else {
                self.next_token(); // cur = expression start
                let body_line = self.cur_token.line;
                let body_col = self.cur_token.column;
                let expr = self.parse_expression(Precedences::Lowest)?;
                // cur_token is last token of expr
                Statement::Expression {
                    expr,
                    line: body_line,
                    column: body_col,
                    end_line: self.cur_token.line,
                    end_column: self.cur_token.column + 1,
                }
            };

            arms.push(SwitchArm {
                pattern,
                body: Box::new(body),
            });

            if self.peak_token_is(&TokenType::Comma) {
                self.next_token();
            }
        }

        if !self.expect_peak(TokenType::RBrace) {
            return None;
        }

        // cur_token is '}'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::Switch {
            subject: Box::new(subject),
            arms,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_for_in_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let key = match self.cur_token.token_type.clone() {
            TokenType::Ident(item) => item,
            _ => {
                self.errors.push(ParseError {
                    kind: ParseErrorKind::UnexpectedToken {
                        expected: TokenType::Ident(String::new()),
                        got: self.cur_token.clone(),
                    },
                    message: format!(
                        "expected loop variable name, got {:?}",
                        self.cur_token.token_type
                    ),
                    line: self.cur_token.line,
                    column: self.cur_token.column,
                });
                return None;
            }
        };

        let mut maybe_value = None;
        if self.peak_token_is(&TokenType::Comma) {
            self.next_token(); // cur = ','
            self.next_token(); // cur = value ident
            maybe_value = match self.cur_token.token_type.clone() {
                TokenType::Ident(value) => Some(value),
                _ => {
                    self.errors.push(ParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: TokenType::Ident(String::new()),
                            got: self.cur_token.clone(),
                        },
                        message: format!(
                            "expected second loop variable name, got {:?}",
                            self.cur_token.token_type
                        ),
                        line: self.cur_token.line,
                        column: self.cur_token.column,
                    });
                    return None;
                }
            };
        }

        if !self.expect_peak(TokenType::In) {
            return None;
        }

        self.next_token();
        let iterable = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::RParen) {
            return None;
        }

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }
        let body = self.parse_block_statement()?;

        // cur_token is '}' after parse_block_statement
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::ForIn {
            key,
            value: maybe_value,
            iterable: Box::new(iterable),
            body: Box::new(body),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let element = self.parse_expression_list(TokenType::RBracket)?;

        // cur_token is ']' after parse_expression_list
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::Array {
            element,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_hash_literal(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let mut pairs: Vec<(Expression, Expression)> = Vec::new();

        while !self.peak_token_is(&TokenType::RBrace) {
            self.next_token();

            let key = self.parse_expression(Precedences::Lowest)?;
            if !self.expect_peak(TokenType::Colon) {
                return None;
            }

            self.next_token();
            let value = self.parse_expression(Precedences::Lowest)?;

            pairs.push((key, value));

            if !self.peak_token_is(&TokenType::RBrace) && !self.expect_peak(TokenType::Comma) {
                return None;
            }
        }

        if !self.expect_peak(TokenType::RBrace) {
            return None;
        }

        // cur_token is '}'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::HashLiteral {
            pair: pairs,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let exp = self.parse_function_parameter()?;

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        // cur_token is '}' after parse_block_statement
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::Function {
            parameter: exp,
            body: Box::from(body),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_function_parameter(&mut self) -> Option<Vec<crate::ast::ast::Param>> {
        use crate::ast::ast::Param;
        let mut list = Vec::new();

        if !self.expect_peak(TokenType::LParan) {
            return None;
        }

        if self.peak_token_is(&TokenType::RParen) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        let name = match self.cur_token.token_type.clone() {
            TokenType::Ident(v) => v,
            _ => {
                self.errors.push(ParseError {
                    kind: ParseErrorKind::UnexpectedToken {
                        expected: TokenType::Ident(String::new()),
                        got: self.cur_token.clone(),
                    },
                    message: format!(
                        "expected parameter name, got {:?}",
                        self.cur_token.token_type
                    ),
                    line: self.cur_token.line,
                    column: self.cur_token.column,
                });
                return None;
            }
        };
        let default = if self.peak_token_is(&TokenType::Assign) {
            self.next_token();
            self.next_token();
            Some(Box::new(self.parse_expression(Precedences::Lowest)?))
        } else {
            None
        };
        list.push(Param { name, default });

        while self.peak_token_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();
            let name = match self.cur_token.token_type.clone() {
                TokenType::Ident(v) => v,
                _ => {
                    self.errors.push(ParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: TokenType::Ident(String::new()),
                            got: self.cur_token.clone(),
                        },
                        message: format!(
                            "expected parameter name, got {:?}",
                            self.cur_token.token_type
                        ),
                        line: self.cur_token.line,
                        column: self.cur_token.column,
                    });
                    return None;
                }
            };
            let default = if self.peak_token_is(&TokenType::Assign) {
                self.next_token();
                self.next_token();
                Some(Box::new(self.parse_expression(Precedences::Lowest)?))
            } else {
                None
            };
            list.push(Param { name, default });
        }

        if !self.expect_peak(TokenType::RParen) {
            return None;
        }

        Some(list)
    }

    fn parse_typeof_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        self.next_token();
        let value = self.parse_expression(Precedences::Lowest)?;
        // cur_token is last token of value expression
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::Typeof {
            value: Box::new(value),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_null_coalesce_expression(&mut self, left: Expression) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        self.next_token();
        let right = self.parse_expression(Precedences::NullCoalesce)?;
        // cur_token is last token of right expression
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::NullCoalesce {
            left: Box::new(left),
            right: Box::new(right),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_char_literal(&self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let value = match self.cur_token.token_type.clone() {
            TokenType::Char(value) => value,
            _ => return None,
        };

        // char literals are written as 'x' = 3 chars
        let end_line = line;
        let end_column = column + 3;
        Some(Expression::Char {
            value,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_string_literal(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let raw_parts = match self.cur_token.token_type.clone() {
            TokenType::InterpolatedString(parts) => parts,
            _ => return None,
        };

        let mut segments = Vec::new();
        for part in raw_parts {
            match part {
                StringPart::Literal(s) => segments.push(StringSegment::Literal(s)),
                StringPart::Expr(src) => {
                    let lexer = Lexer::new(src);
                    let mut sub = Parser::new(lexer);
                    let result = sub.parse_expression(Precedences::Lowest);
                    if !sub.errors.is_empty() {
                        self.errors.push(ParseError {
                            kind: ParseErrorKind::InterpolationError { source: sub.errors },
                            message: "invalid expression inside string interpolation".to_string(),
                            line,
                            column,
                        });
                        return None;
                    }
                    let expr = result?;
                    segments.push(StringSegment::Expr(Box::new(expr)));
                }
            }
        }

        // string end is approximate; raw lexeme length not stored
        let end_line = line;
        let end_column = column + 1;
        Some(Expression::InterpolatedString {
            parts: segments,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_identifier(&self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let value = match self.cur_token.token_type.clone() {
            TokenType::Ident(value) => value.clone(),
            _ => return None,
        };

        let end_line = line;
        let end_column = column + value.len();

        Some(Expression::Ident {
            value,
            line,
            column,
            end_column,
            end_line,
        })
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let mut else_if: Vec<ElseIF> = Vec::new();

        if !self.expect_peak(TokenType::LParan) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::RParen) {
            return None;
        }

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }

        let consequences = self.parse_block_statement()?;

        while self.peak_token_is(&TokenType::ElseIf) {
            self.next_token();

            if !self.expect_peak(TokenType::LParan) {
                return None;
            }

            self.next_token();
            let ei_condition = self.parse_expression(Precedences::Lowest)?;

            if !self.expect_peak(TokenType::RParen) {
                return None;
            }

            if !self.expect_peak(TokenType::LBrace) {
                return None;
            }

            let ei_consequences = self.parse_block_statement()?;

            else_if.push(ElseIF {
                condition: ei_condition,
                consequences: ei_consequences,
            });
        }

        let alternative = if self.peak_token_is(&TokenType::Else) {
            self.next_token();
            if !self.expect_peak(TokenType::LBrace) {
                return None;
            }
            Some(Box::from(self.parse_block_statement()?))
        } else {
            None
        };

        // cur_token is '}' from the last block parsed
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::If {
            condition: Box::from(condition),
            consequence: Box::from(consequences),
            alternative,
            if_else: else_if,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_boolean(&self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        // 'true' = 4 chars, 'false' = 5 chars
        let end_line = line;
        let end_column = if self.cur_token_is(&TokenType::True) {
            column + 4
        } else {
            column + 5
        };
        Some(Expression::Boolean {
            value: self.cur_token_is(&TokenType::True),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        match self.cur_token.token_type {
            // all of these registered parseInfixExpression in the Go:
            TokenType::Plus | TokenType::Minus | TokenType::SLASH | TokenType::Asterisk   // PLUS MINUS SLASH ASTERISK
            | TokenType::Rem | TokenType::Square | TokenType::Floor                       // REM SQUARE FLOOR
            | TokenType::EQ | TokenType::NOTEQ                                            // EQ NOT_EQ
            | TokenType::LT | TokenType::GT                                               // LT GT
            | TokenType::GreaterThanEqual | TokenType::LessThanEqual                      // GREATER_THAN_EQUAL LESS_THAN_EQUAL
            | TokenType::And | TokenType::Or                                             // AND OR
            | TokenType::Assign                                                           // ASSIGN
            | TokenType::AddAssign | TokenType::SubAssign | TokenType::MulAssign          // ADD_ASSIGN SUB_ASSIGN MUL_ASSIGN
            | TokenType::QuoAssign | TokenType::RemAssign                                 // QUO_ASSIGN REM_ASSIGN
                => self.parse_infix_expression(left),

            TokenType::LParan   => self.parse_call_expression(left),          // LPAREN  → call
            TokenType::LBracket => self.parse_index_expression(left),         // LBRACKET → index
            TokenType::Dot      => self.parse_member_expression(left),        // DOT → member
            TokenType::Inc | TokenType::Dec => self.parse_update_postfix_expression(left), // INC DEC (x++, x--)
            TokenType::NullCoalesce => self.parse_null_coalesce_expression(left),

            _ => Some(left),   // no infix parser → expression ends, return left unchanged
        }
    }

    fn parse_member_expression(&mut self, left: Expression) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        if !self.expect_peak_ident() {
            return None;
        }

        let value = match &self.cur_token.token_type {
            TokenType::Ident(value) => value.clone(),
            _ => return None,
        };

        // property ends at column + length of its name
        let prop_end_col = self.cur_token.column + value.len();
        let prop_line = self.cur_token.line;
        let prop_col = self.cur_token.column;
        let property = Expression::Ident {
            value,
            line: prop_line,
            column: prop_col,
            end_line: prop_line,
            end_column: prop_end_col,
        };

        let end_line = prop_line;
        let end_column = prop_end_col;
        Some(Expression::Member {
            object: Box::from(left),
            property: Box::from(property),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        self.next_token();
        let index = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::RBracket) {
            return None;
        }

        // cur_token is ']'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::Index {
            left: Box::from(left),
            index: Box::from(index),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_call_expression(&mut self, left: Expression) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let arguments = self.parse_expression_list(TokenType::RParen)?;

        // cur_token is ')' after parse_expression_list
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::Call {
            function: Box::from(left),
            argument: arguments,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Expression>> {
        let mut list: Vec<Expression> = Vec::new();

        if self.peak_token_is(&end) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedences::Lowest)?);

        while self.peak_token_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedences::Lowest)?);
        }

        if !self.expect_peak(end) {
            return None;
        }

        Some(list)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let result = match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Import => self.parse_import_statement(),
            TokenType::Struct => self.parse_struct_statement(),
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
            TokenType::Const => self.parse_const_statement(),
            TokenType::Enum => self.parse_enum_statement(),
            TokenType::Pub => self.parse_pub_statement(),
            _ => self.parse_expression_statement(),
        };

        if result.is_none() {
            self.synchronize();
        }

        result
    }

    fn parse_pub_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        self.next_token();
        let inner = match self.cur_token.token_type.clone() {
            TokenType::Let => self.parse_let_statement()?,
            TokenType::Const => self.parse_const_statement()?,
            _ => {
                self.errors.push(ParseError {
                    kind: ParseErrorKind::IllegalKeyword {
                        keyword: self.cur_token.clone(),
                    },
                    message: format!(
                        "'pub' can only precede 'let' or 'const', got {:?}",
                        self.cur_token.token_type
                    ),
                    line: self.cur_token.line,
                    column: self.cur_token.column,
                });
                return None;
            }
        };

        // cur_token is wherever the inner statement left it
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Pub {
            statement: Box::new(inner),
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_enum_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        self.next_token();
        let name = match self.cur_token.token_type.clone() {
            TokenType::Ident(v) => v,
            _ => {
                self.errors.push(ParseError {
                    kind: ParseErrorKind::UnexpectedToken {
                        expected: TokenType::Ident(String::new()),
                        got: self.cur_token.clone(),
                    },
                    message: format!("expected enum name, got {:?}", self.cur_token.token_type),
                    line: self.cur_token.line,
                    column: self.cur_token.column,
                });
                return None;
            }
        };

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        };

        let mut variants = Vec::new();
        while !self.peak_token_is(&TokenType::RBrace) && !self.peak_token_is(&TokenType::EOF) {
            self.next_token();
            let variant = match self.cur_token.token_type.clone() {
                TokenType::Ident(v) => v,
                _ => {
                    self.errors.push(ParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: TokenType::Ident(String::new()),
                            got: self.cur_token.clone(),
                        },
                        message: format!(
                            "expected enum variant name, got {:?}",
                            self.cur_token.token_type
                        ),
                        line: self.cur_token.line,
                        column: self.cur_token.column,
                    });
                    return None;
                }
            };
            variants.push(variant);
            if self.peak_token_is(&TokenType::Comma) {
                self.next_token();
            }
        }

        if !self.expect_peak(TokenType::RBrace) {
            return None;
        }
        if !self.expect_peak(TokenType::Semicolon) {
            return None;
        }

        // cur_token is ';'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Enum {
            name,
            variant: variants,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_continue_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        self.expect_peak(TokenType::Semicolon);
        // cur_token is ';' if found, otherwise 'continue'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Continue {
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_break_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        self.expect_peak(TokenType::Semicolon);
        // cur_token is ';' if found, otherwise 'break'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Break {
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_struct_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        if !self.expect_peak_ident() {
            return None;
        }

        let name = match &self.cur_token.token_type {
            TokenType::Ident(v) => v.clone(),
            _ => return None,
        };
        let iden_end_col = self.cur_token.column + name.len();
        let iden_line = self.cur_token.line;
        let iden_col = self.cur_token.column;
        let iden = Expression::Ident {
            value: name,
            line: iden_line,
            column: iden_col,
            end_line: iden_line,
            end_column: iden_end_col,
        };

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }

        let mut field = HashMap::new();

        while !self.peak_token_is(&TokenType::RBrace) {
            self.next_token();

            let key = match &self.cur_token.token_type {
                TokenType::Ident(e) => e.clone(),
                _ => {
                    self.errors.push(ParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: TokenType::Ident(String::new()),
                            got: self.cur_token.clone(),
                        },
                        message: format!(
                            "expected struct field name, got {:?}",
                            self.cur_token.token_type
                        ),
                        line: self.cur_token.line,
                        column: self.cur_token.column,
                    });
                    return None;
                }
            };

            if !self.expect_peak(TokenType::Colon) {
                return None;
            }

            self.next_token();
            let value = self.parse_expression(Precedences::Lowest)?;

            field.insert(key, value);

            if self.peak_token_is(&TokenType::Comma) {
                self.next_token();
            }
        }

        if !self.expect_peak(TokenType::RBrace) {
            return None;
        }

        if !self.expect_peak(TokenType::Semicolon) {
            return None;
        }

        // cur_token is ';'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Struct {
            name: Box::new(iden),
            field,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_import_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        if !matches!(self.peak_token.token_type, TokenType::InterpolatedString(_)) {
            self.peak_error(TokenType::InterpolatedString(vec![]));
            return None;
        }
        self.next_token();

        let path = match &self.cur_token.token_type {
            TokenType::InterpolatedString(parts) => match parts.as_slice() {
                [StringPart::Literal(s)] => s.clone(),
                _ => {
                    self.errors.push(ParseError {
                        kind: ParseErrorKind::InvalidLiteral {
                            raw: "import path must be a plain string with no interpolation"
                                .to_string(),
                        },
                        message: "import path must be a plain string".to_string(),
                        line: self.cur_token.line,
                        column: self.cur_token.column,
                    });
                    return None;
                }
            },
            _ => return None,
        };

        if !self.expect_peak(TokenType::Semicolon) {
            return None;
        }

        // cur_token is ';'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Import {
            path,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let pattern = if self.peak_token_is(&TokenType::LBracket) {
            self.next_token();
            let mut names = Vec::new();
            while !self.peak_token_is(&TokenType::RBracket) {
                self.next_token();
                let name = match self.cur_token.token_type.clone() {
                    TokenType::Ident(v) => v,
                    _ => {
                        self.errors.push(ParseError {
                            kind: ParseErrorKind::UnexpectedToken {
                                expected: TokenType::Ident(String::new()),
                                got: self.cur_token.clone(),
                            },
                            message: format!(
                                "expected identifier in array destructuring pattern, got {:?}",
                                self.cur_token.token_type
                            ),
                            line: self.cur_token.line,
                            column: self.cur_token.column,
                        });
                        return None;
                    }
                };
                names.push(name);
                if self.peak_token_is(&TokenType::Comma) {
                    self.next_token();
                }
            }
            if !self.expect_peak(TokenType::RBracket) {
                return None;
            }
            LetPattern::Array(names)
        } else if self.peak_token_is(&TokenType::LBrace) {
            self.next_token();
            let mut pairs = Vec::new();
            while !self.peak_token_is(&TokenType::RBrace) {
                self.next_token();
                let key = match self.cur_token.token_type.clone() {
                    TokenType::Ident(v) => v,
                    _ => {
                        self.errors.push(ParseError {
                            kind: ParseErrorKind::UnexpectedToken {
                                expected: TokenType::Ident(String::new()),
                                got: self.cur_token.clone(),
                            },
                            message: format!(
                                "expected identifier in hash destructuring pattern, got {:?}",
                                self.cur_token.token_type
                            ),
                            line: self.cur_token.line,
                            column: self.cur_token.column,
                        });
                        return None;
                    }
                };
                let alias = if self.peak_token_is(&TokenType::Colon) {
                    self.next_token();
                    self.next_token();
                    match self.cur_token.token_type.clone() {
                        TokenType::Ident(v) => v,
                        _ => {
                            self.errors.push(ParseError {
                                kind: ParseErrorKind::UnexpectedToken {
                                    expected: TokenType::Ident(String::new()),
                                    got: self.cur_token.clone(),
                                },
                                message: format!(
                                    "expected identifier for destructuring alias, got {:?}",
                                    self.cur_token.token_type
                                ),
                                line: self.cur_token.line,
                                column: self.cur_token.column,
                            });
                            return None;
                        }
                    }
                } else {
                    key.clone()
                };
                pairs.push((key, alias));
                if self.peak_token_is(&TokenType::Comma) {
                    self.next_token();
                }
            }
            if !self.expect_peak(TokenType::RBrace) {
                return None;
            }
            LetPattern::Hash(pairs)
        } else {
            if !self.expect_peak_ident() {
                return None;
            }
            let name = match self.cur_token.token_type.clone() {
                TokenType::Ident(n) => n,
                _ => return None,
            };
            LetPattern::Ident(name)
        };

        let value =
            if self.peak_token_is(&TokenType::Semicolon) || self.peak_token_is(&TokenType::EOF) {
                if self.peak_token_is(&TokenType::Semicolon) {
                    self.next_token();
                }
                // uninitialized let — null spans the semicolon position
                Expression::Null {
                    line: self.cur_token.line,
                    column: self.cur_token.column,
                    end_line: self.cur_token.line,
                    end_column: self.cur_token.column + 1,
                }
            } else {
                if !self.expect_peak(TokenType::Assign) {
                    return None;
                }
                self.next_token();
                let v = self.parse_expression(Precedences::Lowest)?;
                if self.peak_token_is(&TokenType::Semicolon) {
                    self.next_token();
                }
                v
            };

        // cur_token is ';' or last token of value
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Let {
            pattern,
            value,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_expression(&mut self, precedences: Precedences) -> Option<Expression> {
        let mut left_exp = self.parse_prefix()?;

        while !self.peak_token_is(&TokenType::Semicolon) && precedences < self.peak_precedence() {
            if !self.has_infix(&self.peak_token.token_type) {
                break;
            }
            self.next_token();
            left_exp = self.parse_infix(left_exp)?;
        }

        // struct literal: only fire after full Pratt parse, and only for bare idents
        if self.peak_token_is(&TokenType::LBrace) && matches!(left_exp, Expression::Ident { .. }) {
            return self.parse_struct_literal(left_exp);
        }

        Some(left_exp)
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let expr = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::Semicolon) {
            return None;
        };

        // cur_token is ';'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Statement::Expression {
            expr,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn parse_struct_literal(&mut self, left: Expression) -> Option<Expression> {
        let (ident_name, line, column) = match left {
            Expression::Ident {
                value,
                line,
                column,
                ..
            } => (value, line, column),
            _ => return None,
        };

        let mut fields: HashMap<String, Expression> = HashMap::new();

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }

        while !self.peak_token_is(&TokenType::RBrace) {
            self.next_token();

            let key = match &self.cur_token.token_type {
                TokenType::Ident(value) => value.clone(),
                _ => {
                    self.errors.push(ParseError {
                        kind: ParseErrorKind::UnexpectedToken {
                            expected: TokenType::Ident(String::new()),
                            got: self.cur_token.clone(),
                        },
                        message: format!(
                            "expected struct field name, got {:?}",
                            self.cur_token.token_type
                        ),
                        line: self.cur_token.line,
                        column: self.cur_token.column,
                    });
                    return None;
                }
            };

            if !self.expect_peak(TokenType::Colon) {
                return None;
            }
            self.next_token();

            let value = self.parse_expression(Precedences::Lowest)?;
            fields.insert(key, value);

            if self.peak_token_is(&TokenType::Comma) {
                self.next_token();
            }
        }

        if !self.expect_peak(TokenType::RBrace) {
            return None;
        }

        // cur_token is '}'
        let end_line = self.cur_token.line;
        let end_column = self.cur_token.column + 1;
        Some(Expression::StructLiteral {
            name: ident_name,
            fields,
            line,
            column,
            end_line,
            end_column,
        })
    }

    fn has_infix(&self, tok: &TokenType) -> bool {
        matches!(
            tok,
            TokenType::Plus
                | TokenType::Minus
                | TokenType::SLASH
                | TokenType::Asterisk
                | TokenType::Rem
                | TokenType::Square
                | TokenType::Floor
                | TokenType::EQ
                | TokenType::NOTEQ
                | TokenType::LT
                | TokenType::GT
                | TokenType::GreaterThanEqual
                | TokenType::LessThanEqual
                | TokenType::And
                | TokenType::Or
                | TokenType::Assign
                | TokenType::AddAssign
                | TokenType::SubAssign
                | TokenType::MulAssign
                | TokenType::QuoAssign
                | TokenType::RemAssign
                | TokenType::LParan
                | TokenType::LBracket
                | TokenType::Dot
                | TokenType::Inc
                | TokenType::Dec
                | TokenType::NullCoalesce
        )
    }

    fn peak_token_is(&self, tok: &TokenType) -> bool {
        self.peak_token.token_type == *tok
    }

    fn cur_token_is(&self, tok: &TokenType) -> bool {
        self.cur_token.token_type == *tok
    }

    fn expect_peak(&mut self, tok: TokenType) -> bool {
        if self.peak_token_is(&tok) {
            self.next_token();
            true
        } else {
            self.peak_error(tok);
            false
        }
    }

    fn expect_peak_ident(&mut self) -> bool {
        if matches!(self.peak_token.token_type, TokenType::Ident(_)) {
            self.next_token();
            true
        } else {
            self.peak_error(TokenType::Ident(String::new()));
            false
        }
    }

    fn peak_error(&mut self, expected: TokenType) {
        self.errors.push(ParseError {
            kind: ParseErrorKind::UnexpectedToken {
                expected: expected.clone(),
                got: self.peak_token.clone(),
            },
            message: format!(
                "expected {:?}, got {:?}",
                expected.clone(),
                self.peak_token.token_type
            ),
            line: self.peak_token.line,
            column: self.peak_token.column,
        });
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.token_type != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn synchronize(&mut self) {
        // keep consuming tokens until we land on something that looks
        // like the start of a statement, or we hit EOF

        loop {
            match self.cur_token.token_type {
                // a semicolon ends the broken statement —
                // consume it and return so the caller can try the next one
                TokenType::Semicolon => {
                    self.next_token();
                    return;
                }

                // EOF — stop, nothing left to parse
                TokenType::EOF => return,

                // these all start a new statement —
                // do NOT consume; return and let parse_statement handle it
                TokenType::Let
                | TokenType::Const
                | TokenType::If
                | TokenType::While
                | TokenType::For
                | TokenType::Return
                | TokenType::Import
                | TokenType::Struct
                | TokenType::Enum
                | TokenType::Pub => return,

                // anything else is part of the broken statement — skip it
                _ => {
                    self.next_token();
                }
            }
        }
    }

    pub fn new(l: Lexer) -> Self {
        let mut p = Self {
            l,
            errors: Vec::new(),
            cur_token: Token {
                token_type: TokenType::EOF,
                line: 0,
                column: 0,
            },
            peak_token: Token {
                token_type: TokenType::EOF,
                line: 0,
                column: 0,
            },
        };

        p.next_token();
        p.next_token();

        p
    }
}
