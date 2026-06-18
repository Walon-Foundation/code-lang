
#[derive(Debug, PartialEq, Clone)]
pub enum StringPart{
    Literal(String),
    Expr(String)
}


#[derive(Debug, PartialEq,Clone)]
pub enum TokenType {
    // Keywords
    Const,
    Let,
    Function,
    True,
    False,
    If,
    Else,
    ElseIf,
    While,
    For,
    Return,
    Break,
    Import,
    Struct,
    In,
    Switch,
    Enum,
    Continue,
    Ident(String),

    //Identifiers
    Int(isize),

    //String stuff
    InterpolatedString(Vec<StringPart>),
     
    //Float
    Float(f64),
    
    //char
    Char(char),

    //operators
    Asign,
    Minus,
    Plus,
    Bang,
    Asterisk,

    Rem,
    Square,
    Floor,
    Inc,
    Dec,

    AddAssign,
    SubAssign,
    MulAssign,
    QuoAssign,
    RemAssign,

    //logic operator
    And,
    Or,

    SLASH,
    LT,
    GT,
    GreaterThanEqual,
    LessThanEqual,
    EQ,
    NOTEQ,

    //for match
    FatArrow,

    //Delimitters
    Comma,
    Semicolon,
    Colon,
    LParan,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    //accessor thing
    Dot,

    //comment
    Comment,
    MultiCommentStart,
    MultiCommentEnd,

    //import
    EOF,
    ILLEGAL
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type:TokenType,
    pub line:usize,
    pub column:usize,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn"       => TokenType::Function,
        "let"      => TokenType::Let,
        "true"     => TokenType::True,
        "false"    => TokenType::False,
        "if"       => TokenType::If,
        "else"     => TokenType::Else,
        "elseif"   => TokenType::ElseIf,
        "for"      => TokenType::For,
        "while"    => TokenType::While,
        "return"   => TokenType::Return,
        "break"    => TokenType::Break,
        "in"       => TokenType::In,
        "continue" => TokenType::Continue,
        "import"   => TokenType::Import,
        "struct"   => TokenType::Struct,
        "const"    => TokenType::Const,
        "switch"   => TokenType::Switch,
        "enum"     => TokenType::Enum,
        _          => TokenType::Ident(ident.to_string()),
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lookup_ident_pass(){
        let value = lookup_ident("fn");
        assert_eq!(value, TokenType::Function)
    }

    #[test]
    fn lookup_ident_fail(){
        let value = lookup_ident("go");
        assert_ne!(value, TokenType::Const)
    }
}