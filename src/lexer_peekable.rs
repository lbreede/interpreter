use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

// #[derive(Debug, PartialEq)]
// enum Token {
//     Let,
//     Ident(String),
//     Assign,
//     Int(String),
//     Semicolon,
//     Illegal(String),
//     Eof,
// }
//
struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }
    fn next_token(&mut self) -> Token {
        if let Some(c) = self.input.next() {
            match c {
                ';' => return Token::Semicolon,
                '(' => return Token::LParen,
                ')' => return Token::RParen,
                ',' => return Token::Comma,
                '{' => return Token::LBrace,
                '}' => return Token::RBrace,
                '+' => return Token::Plus,
                '-' => return Token::Minus,
                '*' => return Token::Asterisk,
                '<' => return Token::LT,
                '>' => return Token::GT,
                '/' => return Token::Slash,
                '=' => {
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        return Token::EQ;
                    }
                    return Token::Assign;
                }
                '!' => {
                    if self.input.peek() == Some(&'=') {
                        self.input.next();
                        return Token::NotEQ;
                    }
                    return Token::Bang;
                }
                'a'..='z' | 'A'..='Z' | '_' => return self.ident(c),
                '0'..='9' => return self.number(c),
                ' ' | '\n' | '\t' | '\r' => self.next_token(),
                _ => return Token::Illegal(c.to_string()),
            }
        } else {
            return Token::Eof;
        }
    }
    fn number(&mut self, c: char) -> Token {
        let mut number = c.to_string();
        while let Some(n) = self.input.peek() {
            match n {
                '0'..='9' => number.push(self.input.next().unwrap()),
                _ => break,
            }
        }
        return Token::Int(number);
    }
    fn ident(&mut self, c: char) -> Token {
        let mut ident = c.to_string();
        while let Some(p) = self.input.peek() {
            match p {
                'a'..='z' | 'A'..='Z' | '_' => ident.push(self.input.next().unwrap()),
                _ => break,
            }
        }
        return match ident.as_str() {
            "let" => Token::Let,
            "fn" => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(ident),
        };
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
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
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
            Token::LT,
            Token::Int(String::from("10")),
            Token::GT,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(String::from("5")),
            Token::LT,
            Token::Int(String::from("10")),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(String::from("10")),
            Token::EQ,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("10")),
            Token::NotEQ,
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
