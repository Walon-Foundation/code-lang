use crate::token::token::{ StringPart, Token, TokenType, lookup_ident};


#[derive(Debug)]
pub struct Lexer {
    pub input:Vec<char>,
    pub position:usize,
    pub read_position:usize,
    pub ch:char,
    pub line:usize,
    pub column:usize,
}

impl Lexer {
    fn read_char(&mut self){
        if self.read_position >= self.input.len() {
            self.ch = '\0';
            self.column += 1;
        }else {
            self.ch = self.input[self.read_position];
            if self.ch == '\n'{
                self.line += 1;
                self.column = 0;
            }else {
                self.column += 1;
            }
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self){
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peak_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0'
        }else {
            self.input[self.read_position]
        }
    }

    fn skip_single_line_comment(&mut self){
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
    }

    fn skip_multi_line_comment(&mut self) {
        loop {
            if self.ch == '\0' {
                break;
            }

            if self.ch == '*' && self.peak_char() == '/' {
                self.read_char();
                self.read_char();
                break
            }

            self.read_char();
        }
    }

    fn read_string(&mut self) -> Vec<StringPart> {
        let mut parts = Vec::new();
        let mut current_literal = String::new();

        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }

            if self.ch == '$' && self.peak_char() == '{' {
                if !current_literal.is_empty() {
                    parts.push(StringPart::Literal(current_literal.clone()));
                    current_literal.clear();
                }
                self.read_char(); // consume '$' → now on '{'
                self.read_char(); // consume '{' → now inside expr

                let mut exp_src = String::new();
                let mut depth = 1;
                loop {
                    if self.ch == '{' { depth += 1; }
                    if self.ch == '}' { depth -= 1; }
                    if depth == 0 { break; }
                    exp_src.push(self.ch);
                    self.read_char();
                }
                parts.push(StringPart::Expr(exp_src));
            } else {
                current_literal.push(self.ch);
            }
        }

        // flush any trailing literal after the closing quote
        if !current_literal.is_empty() {
            parts.push(StringPart::Literal(current_literal));
        }

        parts
    }

    fn read_char_type(&mut self) -> TokenType {
        self.read_char();
        if self.ch == '\0' || self.ch == '\'' {
            return TokenType::ILLEGAL
        }

        let value = self.ch;
        self.read_char();

        if self.ch != '\'' {
            self.read_char();
            return TokenType::ILLEGAL
        }

        TokenType::Char(value)
    }

    fn read_float(&mut self) -> TokenType {
        let position = self.position;
        self.read_char();

        while is_digit(self.ch) {
            self.read_char();
        };

        self.input[position..self.position]
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .map(TokenType::Float)
            .unwrap_or(TokenType::ILLEGAL)
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(self.ch) || is_digit(self.ch){
            self.read_char();
        }

        self.input[position..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        
    	while is_digit(self.ch) {
    		self.read_char()
    	}
        
    	if self.ch == '.' {
    		self.read_char();
    		while is_digit(self.ch) {
    			self.read_char()
    		}
    	}
        
    	self.input[position..self.position].iter().collect()
    }

    pub fn next_token(&mut self) -> Token{
        self.skip_whitespace();

        let current_line = self.line;
        let current_column = self.column;

        let mut token = Token { token_type: TokenType::ILLEGAL, line: current_line, column: current_column };

        match self.ch {
            '=' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    token = Token { token_type: TokenType::EQ, line: current_line, column: current_column };
                }else if self.peak_char() == '>' {
                    self.read_char();
                    token = Token { token_type: TokenType::FatArrow, line: current_line, column: current_column }
                } else {
                    token = new_token(TokenType::Assign, current_line, current_column);
                }
            },

            '(' => token = new_token(TokenType::LParan, current_line, current_column),
            ')' => token = new_token(TokenType::RParen, current_line, current_column),
            '{' => token = new_token(TokenType::LBrace, current_line, current_column),
            '}' => token = new_token(TokenType::RBrace, current_line, current_column),
            '[' => token = new_token(TokenType::LBracket, current_line, current_column),
            ']' => token = new_token(TokenType::RBracket, current_line, current_column),
            ';' => token = new_token(TokenType::Semicolon, current_line, current_column),
            ',' => token = new_token(TokenType::Comma, current_line, current_column),
            ':' => token = new_token(TokenType::Colon, current_line, current_column),

            '+' => {
                if self.peak_char() == '+' {
                    self.read_char();
                    token = Token { token_type: TokenType::Inc, line: current_line, column: current_column };
                } else if self.peak_char() == '=' {
                    self.read_char();
                    token = Token { token_type: TokenType::AddAssign, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::Plus, current_line, current_column);
                }
            }

            '-' => {
                if self.peak_char() == '-' {
                    self.read_char();
                    token = Token { token_type: TokenType::Dec, line: current_line, column: current_column };
                } else if self.peak_char() == '=' {
                    self.read_char();
                    token = Token { token_type: TokenType::SubAssign, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::Minus, current_line, current_column);
                }
            }

            '#' => {
                self.skip_single_line_comment();
                return self.next_token();
            }

            '!' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    token = Token { token_type: TokenType::NOTEQ, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::Bang, current_line, current_column);
                }
            }

            '/' => {
                if self.peak_char() == '/' {
                    self.read_char();
                    token = Token { token_type: TokenType::Floor, line: current_line, column: current_column };
                } else if self.peak_char() == '*' {
                    self.read_char();
                    self.read_char();
                    self.skip_multi_line_comment();
                    return self.next_token();
                } else if self.peak_char() == '=' {
                    self.read_char();
                    token = Token { token_type: TokenType::QuoAssign, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::SLASH, current_line, current_column);
                }
            }

            '*' => {
                if self.peak_char() == '*' {
                    self.read_char();
                    token = Token { token_type: TokenType::Square, line: current_line, column: current_column };
                } else if self.peak_char() == '=' {
                    self.read_char();
                    token = Token { token_type: TokenType::MulAssign, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::Asterisk, current_line, current_column);
                }
            }

            '<' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    token = Token { token_type: TokenType::LessThanEqual, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::LT, current_line, current_column);
                }
            }

            '>' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    token = Token { token_type: TokenType::GreaterThanEqual, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::GT, current_line, current_column);
                }
            }

            '%' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    token = Token { token_type: TokenType::RemAssign, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::Rem, current_line, current_column);
                }
            }

            '|' => {
                if self.peak_char() == '|' {
                    self.read_char();
                    token = Token { token_type: TokenType::Or, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::ILLEGAL, current_line, current_column);
                }
            }

            '&' => {
                if self.peak_char() == '&' {
                    self.read_char();
                    token = Token { token_type: TokenType::And, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::ILLEGAL, current_line, current_column);
                }
            }

            '"' => {
                token.token_type = TokenType::InterpolatedString(self.read_string());
                token.line = current_line;
                token.column = current_column;
            }

            '\'' => {
                token.token_type = self.read_char_type();
                token.line = current_line;
                token.column = current_column;
            }

            '.' => {
                if is_digit(self.peak_char()) {
                    token.token_type = self.read_float();
                    token.line = current_line;
                    token.column = current_column;
                    return token;
                } else {
                    token = new_token(TokenType::Dot, current_line, current_column);
                }
            }

            '\0' => {
                token.token_type = TokenType::EOF;
                token.line = current_line;
                token.column = current_column;
            }

            '?' => {
                if self.peak_char() == '?' {
                    self.read_char();
                    token = Token { token_type: TokenType::NullCoalesce, line: current_line, column: current_column };
                } else {
                    token = new_token(TokenType::ILLEGAL, current_line, current_column);
                }
            }

            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_identifier();
                    token.token_type = lookup_ident(&ident);
                    token.line = current_line;
                    token.column = current_column;
                    return token;
                } else if is_digit(self.ch) {
                    let value = self.read_number();
                    if value.contains('.') {
                        token.token_type = value.parse::<f64>()
                            .map(TokenType::Float)
                            .unwrap_or(TokenType::ILLEGAL);
                    } else {
                        token.token_type = value.parse::<isize>()
                            .map(TokenType::Int)
                            .unwrap_or(TokenType::ILLEGAL);
                    }
                    token.line = current_line;
                    token.column = current_column;
                    return token;
                } else {
                    token = new_token(TokenType::ILLEGAL, current_line, current_column);
                }
            }
        };

        self.read_char();
        token
    }

    pub fn new(input:String) -> Self {
        let input:Vec<char> = input.chars().collect();
        
        let mut  l = Lexer { 
            input, 
            position: 0, 
            read_position: 0, 
            ch: '\0', 
            line: 1, 
            column: 0 
        };

        l.read_char();
        l
    }
}

fn new_token(token_type:TokenType, line:usize, column:usize) -> Token {
    Token { 
        token_type,  
        line, 
        column 
    }
}

fn is_digit(ch:char) -> bool {
    ch.is_ascii_digit()
}

fn is_letter(ch: char) -> bool {
	ch.is_ascii_alphabetic() || ch == '_'
}


#[cfg(test)]
mod test {
    use super::*;

    fn tokenize(input: &str) -> Vec<TokenType> {
        let mut lexer = Lexer::new(input.to_string());
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            let is_eof = tok.token_type == TokenType::EOF;
            tokens.push(tok.token_type);
            if is_eof { break; }
        }
        tokens
    }

    #[test]
    fn test_single_char_tokens() {
        assert_eq!(
            tokenize("(){}[];,.:"),
            vec![
                TokenType::LParan, TokenType::RParen,
                TokenType::LBrace, TokenType::RBrace,
                TokenType::LBracket, TokenType::RBracket,
                TokenType::Semicolon, TokenType::Comma,
                TokenType::Dot, TokenType::Colon,
                TokenType::EOF,
            ]
        );
    }

    #[test]
    fn test_arithmetic_operators() {
        assert_eq!(
            tokenize("+ - * / %"),
            vec![
                TokenType::Plus, TokenType::Minus,
                TokenType::Asterisk, TokenType::SLASH,
                TokenType::Rem, TokenType::EOF,
            ]
        );
    }

    #[test]
    fn test_compound_operators() {
        assert_eq!(
            tokenize("++ -- += -= *= /= %="),
            vec![
                TokenType::Inc, TokenType::Dec,
                TokenType::AddAssign, TokenType::SubAssign,
                TokenType::MulAssign, TokenType::QuoAssign,
                TokenType::RemAssign, TokenType::EOF,
            ]
        );
    }

    #[test]
    fn test_comparison_operators() {
        assert_eq!(
            tokenize("== != < > <= >="),
            vec![
                TokenType::EQ, TokenType::NOTEQ,
                TokenType::LT, TokenType::GT,
                TokenType::LessThanEqual, TokenType::GreaterThanEqual,
                TokenType::EOF,
            ]
        );
    }

    #[test]
    fn test_assign_and_bang() {
        assert_eq!(
            tokenize("= !"),
            vec![TokenType::Assign, TokenType::Bang, TokenType::EOF]
        );
    }

    #[test]
    fn test_logical_operators() {
        assert_eq!(
            tokenize("&& ||"),
            vec![TokenType::And, TokenType::Or, TokenType::EOF]
        );
    }

    #[test]
    fn test_floor_and_power() {
        assert_eq!(
            tokenize("// **"),
            vec![TokenType::Floor, TokenType::Square, TokenType::EOF]
        );
    }

    #[test]
    fn test_keywords() {
        assert_eq!(
            tokenize("fn let const true false if else elseif for while return break continue import struct"),
            vec![
                TokenType::Function, TokenType::Let, TokenType::Const,
                TokenType::True, TokenType::False,
                TokenType::If, TokenType::Else, TokenType::ElseIf,
                TokenType::For, TokenType::While,
                TokenType::Return, TokenType::Break, TokenType::Continue,
                TokenType::Import, TokenType::Struct,
                TokenType::EOF,
            ]
        );
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            tokenize("foo _bar baz42"),
            vec![
                TokenType::Ident("foo".to_string()),
                TokenType::Ident("_bar".to_string()),
                TokenType::Ident("baz42".to_string()),
                TokenType::EOF,
            ]
        );
    }

    #[test]
    fn test_integer_literal() {
        assert_eq!(
            tokenize("0 42 100"),
            vec![TokenType::Int(0), TokenType::Int(42), TokenType::Int(100), TokenType::EOF]
        );
    }

    #[test]
    fn test_float_literal() {
        assert_eq!(
            tokenize("3.14 0.5"),
            vec![TokenType::Float(3.14), TokenType::Float(0.5), TokenType::EOF]
        );
    }

    // #[test]
    // fn test_string_literal() {
    //     assert_eq!(
    //         tokenize("\"hello\" \"world\""),
    //         vec![
    //             TokenType::InterpolatedString(vec![StringPart::Literal("hello".to_string())]),
    //             TokenType::InterpolatedString(vec![StringPart::Literal("world".to_string())]),
    //             TokenType::EOF,
    //         ]
    //     );
    // }

    #[test]
    fn test_char_literal() {
        assert_eq!(
            tokenize("'a' 'z'"),
            vec![TokenType::Char('a'), TokenType::Char('z'), TokenType::EOF]
        );
    }

    #[test]
    fn test_single_line_comment_skipped() {
        assert_eq!(
            tokenize("42 # this is ignored\n99"),
            vec![TokenType::Int(42), TokenType::Int(99), TokenType::EOF]
        );
    }

    #[test]
    fn test_multi_line_comment_skipped() {
        assert_eq!(
            tokenize("1 /* skip this */ 2"),
            vec![TokenType::Int(1), TokenType::Int(2), TokenType::EOF]
        );
    }

    #[test]
    fn test_eof_on_empty_input() {
        assert_eq!(tokenize(""), vec![TokenType::EOF]);
    }

    #[test]
    fn test_simple_expression() {
        assert_eq!(
            tokenize("let x = 10 + 2;"),
            vec![
                TokenType::Let,
                TokenType::Ident("x".to_string()),
                TokenType::Assign,
                TokenType::Int(10),
                TokenType::Plus,
                TokenType::Int(2),
                TokenType::Semicolon,
                TokenType::EOF,
            ]
        );
    }
}