use std::collections::HashMap;

use crate::{ast::ast::{ElseIF, Expression, Statement}, lexer::lexer::Lexer, token::token::{Token, TokenType}};
use crate::ast::ast::Program;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedences {
    Lowest,
    Assign,
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
    Member
}


#[derive(Debug)]
pub struct Parser {
    l: Lexer,
    errors: Vec<String>,
    cur_token:Token,
    peak_token:Token,
}


impl Parser {
    fn precedence_of(tok: &TokenType) -> Precedences {
        match tok {
            TokenType::EQ | TokenType::NOTEQ => Precedences::Equals,
            TokenType::LT | TokenType::GT | TokenType::LessThanEqual | TokenType::GreaterThanEqual => Precedences::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedences::Sum,
            TokenType::SLASH | TokenType::Asterisk | TokenType::Rem => Precedences::Product,
            TokenType::And => Precedences::And,
            TokenType::Or => Precedences::Or,
            TokenType::LParan => Precedences::Call,
            TokenType::LBracket => Precedences::Index,
            TokenType::Dot => Precedences::Member,
            TokenType::Asign => Precedences::Assign,
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

        if !self.expect_peak(TokenType::Semicolon){
            return None;
        }

        Some(Statement::Return { value, line, column })
    }

    fn parse_const_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        if !self.expect_peak_ident() {
            return None;
        }

        let name = match &self.cur_token.token_type {
            TokenType::Ident(value) => value.clone(),
            _ => return None
        };

        if !self.expect_peak(TokenType::Asign){
            return None
        }

        self.next_token();

        let value = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::Semicolon){
            return None
        }

        Some(Statement::Const { name, value, line, column })
    }

    fn parse_integer_literal(&self) -> Option<Expression>{
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let value = match self.cur_token.token_type {
            TokenType::Int(value) => value,
            _ => return None,
        };

        Some(Expression::Int { value, line, column })
    }

    fn parse_float_literal(&self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let value = match self.cur_token.token_type {
            TokenType::Float(value) => value,
            _ => return None,
        };

        Some(Expression::Float { value, line, column })
    }

    fn parse_while_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        if !self.expect_peak(TokenType::LParan){
            return None;
        }

        self.next_token();

        let condition = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::RParen){
            return None;
        }

        if !self.expect_peak(TokenType::LBrace){
            return None;
        }

        let body = self.parse_block_statement()?;

        Some(Expression::While { 
            condition: Box::from(condition), 
            body:Box::from(body), 
            line, 
            column 
        })
    }

    fn parse_for_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        if !self.expect_peak(TokenType::LParan) {
            return None;
        }

        self.next_token(); // move to init

        // init — parse_statement consumes its own semicolon, leaving cur = ';'
        let init = self.parse_statement()?;

        self.next_token(); // move past ';' to condition

        let condition = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::Semicolon) {
            return None;
        }

        self.next_token(); // move past ';' to post

        let post_exp = self.parse_expression(Precedences::Lowest)?;
        let post = Statement::Expression { expr: post_exp, line, column };

        if !self.expect_peak(TokenType::RParen) {
            return None;
        }

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement()?;

        Some(Expression::For {
            init: Box::from(init),
            condition: Box::from(condition),
            post: Box::from(post),
            body: Box::from(body),
            line,
            column,
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

        Some(Statement::Block { statements, line, column })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let exp = self.parse_expression(Precedences::Lowest);
        if !self.expect_peak(TokenType::RParen){
            return None;
        }

        exp
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression>{
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        let op = self.cur_token.clone();
        let precedence = self.cur_precedence();

        self.next_token();
        let right = self.parse_expression(precedence)?;

        Some(Expression::Infix {
            left: Box::from(left),
            op,
            right: Box::from(right),
            line,
            column
        })
    }

    fn parse_update_prefix_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        let operator = self.cur_token.clone();

        self.next_token();
        let target = self.parse_expression(Precedences::Prefix)?;

        Some(Expression::Update {
            operator,
            target: Box::from(target),
            prefix: true,
            line, column })
    }

    fn parse_update_postfix_expression(&self, left:Expression) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        Some(Expression::Update { 
            operator: self.cur_token.clone(), 
            target: Box::from(left),
            prefix: false,
            line, column })
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        let op = self.cur_token.clone();

        self.next_token();
        let right = self.parse_expression(Precedences::Prefix)?;

        Some(Expression::Prefix { op, right: Box::from(right), line, column })
    }

    
  
    fn parse_prefix(&mut self) -> Option<Expression> {
        match &self.cur_token.token_type {
            TokenType::Ident(_)      => self.parse_identifier(),              // IDENT
            TokenType::Int(_)        => self.parse_integer_literal(),         // INT
            TokenType::Float(_)      => self.parse_float_literal(),           // FLOAT
            TokenType::StringType(_) => self.parse_string_literal(),          // STRING
            TokenType::Char(_)       => self.parse_char_literal(),            // CHAR
            TokenType::True | TokenType::False => self.parse_boolean(),       // TRUE, FALSE
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),       // BANG, MINUS
            TokenType::Inc | TokenType::Dec    => self.parse_update_prefix_expression(), // INC, DEC (++x, --x)
            TokenType::LParan        => self.parse_grouped_expression(),      // LPAREN
            TokenType::If            => self.parse_if_expression(),           // IF
            TokenType::Function      => self.parse_function_literal(),        // FUNCTION
            TokenType::LBracket      => self.parse_array_literal(),           // LBRACKET
            TokenType::LBrace        => self.parse_hash_literal(),            // LBRACE
            TokenType::For           => self.parse_for_expression(),          // FOR
            TokenType::While         => self.parse_while_expression(),        // WHILE
            _ => None,           // map-miss → noPrefixParseFnError
        }
    }

    fn parse_array_literal(&mut self) -> Option<Expression>{
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let element = self.parse_expression_list(TokenType::RBracket)?;
        Some(Expression::Array { element, line, column })
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
                return None
            }
        }

        if !self.expect_peak(TokenType::RBrace) {
            return None
        }

        Some(Expression::HashLiteral { pair: pairs, line, column })
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let exp = self.parse_function_parameter()?;

        if !self.expect_peak(TokenType::LBrace){
            return None
        }

        let body = self.parse_block_statement()?;

        Some(Expression::Function { 
            parameter: exp,
            body: Box::from(body), 
            line, column })
    }

    fn parse_function_parameter(&mut self) -> Option<Vec<Expression>> {
        let mut list = Vec::new();

        if !self.expect_peak(TokenType::LParan) {
            return None;
        }

        if self.peak_token_is(&TokenType::RParen) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        let line = self.cur_token.line;
        let column = self.cur_token.column;
        let value = match self.cur_token.token_type.clone() {
            TokenType::Ident(v) => v,
            _ => return None,
        };
        list.push(Expression::Ident { value, line, column });

        while self.peak_token_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();
            let line = self.cur_token.line;
            let column = self.cur_token.column;
            let value = match self.cur_token.token_type.clone() {
                TokenType::Ident(v) => v,
                _ => return None,
            };
            list.push(Expression::Ident { value, line, column });
        }

        if !self.expect_peak(TokenType::RParen) {
            return None;
        }

        Some(list)
    }

    fn parse_char_literal(&self) -> Option<Expression>{
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let value = match self.cur_token.token_type.clone() {
            TokenType::Char(value) => value.clone(),
            _ => return None
        };

        Some(Expression::Char { value, line, column })
    }

    fn parse_string_literal(&self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let value = match self.cur_token.token_type.clone() {
            TokenType::StringType(value) => value.clone(),
            _ => return None
        };

        Some(Expression::StringLit { value, line, column })
    }

    fn parse_identifier(&self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let value = match self.cur_token.token_type.clone() {
            TokenType::Ident(value) => value.clone(),
            _ => return None
        };
        
        Some(Expression::Ident { value, line, column })
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

        Some(Expression::If {
            condition: Box::from(condition),
            consequence: Box::from(consequences),
            alternative,
            if_else: else_if,
            line, column
        })
    }

    fn parse_boolean(&self) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        Some(Expression::Boolean { value: self.cur_token_is(&TokenType::True), line, column })
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
            | TokenType::Asign                                                           // ASSIGN
            | TokenType::AddAssign | TokenType::SubAssign | TokenType::MulAssign          // ADD_ASSIGN SUB_ASSIGN MUL_ASSIGN
            | TokenType::QuoAssign | TokenType::RemAssign                                 // QUO_ASSIGN REM_ASSIGN
                => self.parse_infix_expression(left),
    
            TokenType::LParan   => self.parse_call_expression(left),          // LPAREN  → call
            TokenType::LBracket => self.parse_index_expression(left),         // LBRACKET → index
            TokenType::Dot      => self.parse_member_expression(left),        // DOT → member
            TokenType::Inc | TokenType::Dec => self.parse_update_postfix_expression(left), // INC DEC (x++, x--)
    
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

        let property = Expression::Ident { value,line, column };

        Some(Expression::Member { object: Box::from(left), property: Box::from(property), line, column })
    }

    fn parse_index_expression(&mut self, left:Expression) -> Option<Expression>{
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        self.next_token();
        let index = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::RBracket){
            return None;
        }

        Some(Expression::Index { left: Box::from(left), index: Box::from(index), line, column })
    }

    fn parse_call_expression(&mut self, left:Expression) -> Option<Expression> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let arguments = self.parse_expression_list(TokenType::RParen)?;

        Some(Expression::Call { function: Box::from(left), argument: arguments, line, column })
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

        if !self.expect_peak(end){
            return None
        }

        Some(list)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let    => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Import => self.parse_import_statement(),
            TokenType::Struct => self.parse_struct_statement(),
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
            TokenType::Const => self.parse_const_statement(),
            _                 => self.parse_expression_statement(),
        }
    }

    fn parse_continue_statement(&mut self) -> Option<Statement> {
        self.expect_peak(TokenType::Semicolon);
        Some(Statement::Continue)
    }

    fn parse_break_statement(&mut self) -> Option<Statement>{
        self.expect_peak(TokenType::Semicolon);
        Some(Statement::Break)
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
        let iden = Expression::Ident { value: name, line, column };

        if !self.expect_peak(TokenType::LBrace) {
            return None;
        }

        let mut field = HashMap::new();

        while !self.peak_token_is(&TokenType::RBrace) {
            self.next_token();

            let key = match &self.cur_token.token_type {
                TokenType::Ident(e) => e.clone(),
                _ => return None,
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

        Some(Statement::Struct { name: Box::new(iden), field })
    }

    fn parse_import_statement(&mut self) -> Option<Statement> {
        if !matches!(self.peak_token.token_type, TokenType::StringType(_)) {
            self.peak_error(TokenType::StringType(String::new()));
            return None;
        }
        self.next_token();

        let path = match &self.cur_token.token_type {
            TokenType::StringType(s) => s.clone(),
            _ => return None,
        };

        if !self.expect_peak(TokenType::Semicolon) {
            return None;
        }

        Some(Statement::Import { path })
    }
    
    fn parse_let_statement(&mut self) -> Option<Statement> {
        let line = self.cur_token.line;
        let column = self.cur_token.column;
    
        if !self.expect_peak_ident() {
            return None;
        }
        
        let name = match &self.cur_token.token_type {
            TokenType::Ident(n) => n.clone(),
            _ => return None,
        };
    
        if !self.expect_peak(TokenType::Asign) {
            return None;
        }
    
        self.next_token();
        let value = self.parse_expression(Precedences::Lowest)?;
    
        if self.peak_token_is(&TokenType::Semicolon) {
            self.next_token();
        }
    
        Some(Statement::Let { name, value, line, column })
    }

    fn parse_expression(&mut self, precedences: Precedences) -> Option<Expression> {
        let mut left_exp = self.parse_prefix()?;

        if self.peak_token_is(&TokenType::LBrace){
            return self.parse_struct_literal(left_exp);
        }

        while !self.peak_token_is(&TokenType::Semicolon) && precedences < self.peak_precedence() {
            if !self.has_infix(&self.peak_token.token_type){
                return Some(left_exp)
            }

            self.next_token();
            left_exp = self.parse_infix(left_exp)?
        };

        Some(left_exp)
    }

    fn parse_expression_statement(&mut self) -> Option<Statement>{
        let line = self.cur_token.line;
        let column = self.cur_token.column;

        let expr = self.parse_expression(Precedences::Lowest)?;

        if !self.expect_peak(TokenType::Semicolon){
            return None
        };

        Some(Statement::Expression { expr, line, column })
    }

    fn parse_struct_literal(&mut self, left: Expression) -> Option<Expression> {
        let (ident_name, line, column) = match left {
            Expression::Ident { value, line, column } => (value, line, column),
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
                _ => return None,
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
        
        Some(Expression::StructLiteral { name: ident_name, fields, line, column })
    }

    fn has_infix(&self, tok: &TokenType) -> bool {
        matches!(
            tok,
            TokenType::Plus | TokenType::Minus | TokenType::SLASH | TokenType::Asterisk
            | TokenType::Rem | TokenType::Square | TokenType::Floor
            | TokenType::EQ | TokenType::NOTEQ | TokenType::LT | TokenType::GT
            | TokenType::GreaterThanEqual | TokenType::LessThanEqual
            | TokenType::And | TokenType::Or | TokenType::Asign
            | TokenType::AddAssign | TokenType::SubAssign | TokenType::MulAssign
            | TokenType::QuoAssign | TokenType::RemAssign
            | TokenType::LParan | TokenType::LBracket | TokenType::Dot
            | TokenType::Inc | TokenType::Dec
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
    
    fn peak_error(&mut self, tok: TokenType) {
        let error = format!(
            "[Line {}, Column {}]: expected next token to be {:?}, got {:?} instead",
            self.peak_token.line,
            self.peak_token.column,
            tok,
            self.peak_token.token_type
        );
        self.errors.push(error);
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };

        while self.cur_token.token_type != TokenType::EOF {
            match self.parse_statement() {
                Some(statement) => program.statements.push(statement),
                None => self.errors.push("invalid statement".to_string()),
            }

            self.next_token();
        }

        program 
    }

    pub fn new(l: Lexer) -> Self{
        let mut p = Self {
            l,
            errors: Vec::new(),
            cur_token: Token { token_type: TokenType::EOF, line: 0, column: 0 },
            peak_token: Token { token_type: TokenType::EOF, line: 0, column: 0 }
        };

        p.next_token();
        p.next_token();

        p
    }
}