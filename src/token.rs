use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal(String),
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,

    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "let" => Token::Let,
            "fn" => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(value),
        }
    }
}

impl From<u8> for Token {
    fn from(value: u8) -> Self {
        match value {
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            b'/' => Token::Slash,
            b'=' => Token::Assign,
            b'!' => Token::Bang,
            0 => Token::Eof,
            _ => Token::Illegal(String::from_utf8_lossy(&[value]).to_string()),
        }
    }
}

#[derive(Debug)]
pub struct ParseTokenError;

impl FromStr for Token {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tok = match s {
            "let" => Token::Let,
            "fn" => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(s.to_string()),
        };
        Ok(tok)
    }
}
