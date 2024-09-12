use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }
    pub fn next_token(&mut self) -> Token {
        if let Some(c) = self.input.next() {
            match c {
                ';' => Token::Semicolon,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                '{' => Token::LSquirly,
                '}' => Token::RSquirly,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Asterisk,
                '<' => Token::LessThan,
                '>' => Token::GreaterThan,
                '/' => Token::Slash,
                '=' => {
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        return Token::Equal;
                    }
                    Token::Assign
                }
                '!' => {
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        return Token::NotEqual;
                    }
                    Token::Bang
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.read_identifier(c);
                    Token::from_str(&identifier).unwrap()
                }
                '0'..='9' => {
                    let number = self.read_number(c);
                    Token::Int(number)
                }
                ' ' | '\n' | '\t' | '\r' => self.next_token(),
                _ => Token::Illegal(c.to_string()),
            }
        } else {
            Token::Eof
        }
    }
    fn read_number(&mut self, c: char) -> String {
        let mut number = c.to_string();
        while let Some(c) = self.input.peek() {
            match c {
                '0'..='9' => number.push(self.input.next().unwrap()),
                _ => break,
            }
        }
        number
    }
    fn read_identifier(&mut self, c: char) -> String {
        let mut ident = c.to_string();
        while let Some(c) = self.input.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => ident.push(self.input.next().unwrap()),
                _ => break,
            }
        }
        ident
    }
}
