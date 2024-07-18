use crate::token::Token;
use std::iter::Peekable;
use std::str::Bytes;
use std::str::Chars;
use std::str::FromStr;

struct Lexer<'a> {
    input: Peekable<Bytes<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input: input.bytes().peekable(),
        }
    }
    fn next_token(&mut self) -> Token {
        if let Some(c) = self.input.next() {
            match c {
                b';' => return Token::Semicolon,
                b'(' => return Token::LParen,
                b')' => return Token::RParen,
                b',' => return Token::Comma,
                b'{' => return Token::LBrace,
                b'}' => return Token::RBrace,
                b'+' => return Token::Plus,
                b'-' => return Token::Minus,
                b'*' => return Token::Asterisk,
                b'<' => return Token::LT,
                b'>' => return Token::GT,
                b'/' => return Token::Slash,
                b'=' => {
                    if self.input.peek() == Some(&b'=') {
                        self.input.next();
                        return Token::EQ;
                    }
                    return Token::Assign;
                }
                b'!' => {
                    if self.input.peek() == Some(&b'=') {
                        self.input.next();
                        return Token::NotEQ;
                    }
                    return Token::Bang;
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => return self.ident(c),
                b'0'..=b'9' => return self.number(c),
                b' ' | b'\n' | b'\t' | b'\r' => self.next_token(),
                _ => return Token::Illegal(c.to_string()),
            }
        } else {
            return Token::Eof;
        }
    }
    fn number(&mut self, c: u8) -> Token {
        let mut number = c.to_string();
        while let Some(n) = self.input.peek() {
            match n {
                b'0'..=b'9' => number.push(self.input.next().unwrap()),
                _ => break,
            }
        }
        return Token::Int(number);
    }
    fn ident(&mut self, c: u8) -> Token {
        let mut ident = c.to_string();
        while let Some(p) = self.input.peek() {
            match p {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => ident.push(self.input.next().unwrap()),
                _ => break,
            }
        }
        return Token::from_str(&ident).unwrap();
    }
}
