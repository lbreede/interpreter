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
                ';' => return Token::Semicolon,
                '(' => return Token::LParen,
                ')' => return Token::RParen,
                ',' => return Token::Comma,
                '{' => return Token::LSquirly,
                '}' => return Token::RSquirly,
                '+' => return Token::Plus,
                '-' => return Token::Minus,
                '*' => return Token::Asterisk,
                '<' => return Token::LessThan,
                '>' => return Token::GreaterThan,
                '/' => return Token::Slash,
                '=' => {
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        return Token::Equal;
                    }
                    return Token::Assign;
                }
                '!' => {
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        return Token::NotEqual;
                    }
                    return Token::Bang;
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.read_identifier(c);
                    return Token::from_str(&identifier).unwrap();
                }
                '0'..='9' => {
                    let number = self.read_number(c);
                    return Token::Int(number);
                }
                ' ' | '\n' | '\t' | '\r' => self.next_token(),
                _ => return Token::Illegal(c.to_string()),
            }
        } else {
            return Token::Eof;
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

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::token::Token::*;

    #[test]
    fn get_next_complete() {
        let input = r#"let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        10 == 10;
        10 != 9;
        "#;

        let mut lex = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LSquirly,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RSquirly,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::GreaterThan,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::RParen,
            Token::LSquirly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RSquirly,
            Token::Else,
            Token::LSquirly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RSquirly,
            Token::Int(String::from("10")),
            Token::Equal,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("10")),
            Token::NotEqual,
            Token::Int(String::from("9")),
            Token::Semicolon,
            Token::Eof,
        ];

        for expected_token in tokens {
            let token = lex.next_token();
            println!("expected: {:?}, received {:?}", expected_token, token);
            assert_eq!(expected_token, token);
        }
    }
}
