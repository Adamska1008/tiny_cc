
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: literal.to_string()
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Eof,
    Illegal,

    Ident,
    Number,
    String,

    Read,
    If,
    Then,
    Repeat,
    Until,
    Write,
    End,
    
    LessThan,
    Assign,
    EqualLessThan,
    Equal,

    Plus,
    Minus,

    SemiColon,
}

pub fn look_up_keywords(ident: &str) -> TokenType {
    match ident {
        "read" => TokenType::Read,
        "if" => TokenType::If,
        "then" => TokenType::Then,
        "repeat" => TokenType::Repeat,
        "until" => TokenType::Until,
        "write" => TokenType::Write,
        "end" => TokenType::End,
        _ => TokenType::Ident,
    }
}

