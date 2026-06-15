use crate::token::token::{ Token, TokenType};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lexer {
    pub input:Vec<char>,
    pub position:usize,
    pub read_position:usize,
    pub ch:char,
    pub line:usize,
    pub  column:usize,
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
                self.column += 1;
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

    pub fn next_token(&mut self) -> Token{
        let mut token = Token{
            token_type:TokenType::Ident("walon".to_string()),
            literal:"nothing".to_string(),
            line:0,
            colum:0
        };

        //consume whitespace
        self.skip_whitespace();
        
        let current_line = self.line;
        let current_column = self.column;

        match self.ch {
            '=' => {
                if self.peak_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    
                    let mut literal = String::new();
                    literal.push(ch);
                    literal.push(self.ch);
                    
                    token = Token { 
                        token_type: TokenType::EQ, 
                        literal, 
                        line: current_line, 
                        colum: current_column 
                    }
                }else {
                    token = new_token(TokenType::Asign, self.ch, current_line, current_column)
                }
            },
            
            '(' => {
                token = new_token(TokenType::LParan, self.ch, current_line, current_column)
            },

            ')' => {
                token = new_token(TokenType::RParen, self.ch, current_line, current_column)
            }
            _ => {
                
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

fn new_token(token_type:TokenType, ch:char, line:usize, colum:usize) -> Token {
    Token { 
        token_type, 
        literal: ch.to_string(), 
        line, 
        colum 
    }
}


#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    
}