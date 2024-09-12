use std::iter::Peekable;
use std::str::Bytes;
use std::str::FromStr;

use crate::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Bytes<'a>>,
}

impl<'a> Lexer<'a> {
    /// Create a new Lexer.
    ///
    /// The input is a peekable iterator of bytes.
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.bytes().peekable(),
        }
    }
    pub fn next_token(&mut self) -> Token {
        if let Some(b) = self.input.next() {
            match b {
                b';' => Token::Semicolon,
                b'(' => Token::LParen,
                b')' => Token::RParen,
                b',' => Token::Comma,
                b'{' => Token::LSquirly,
                b'}' => Token::RSquirly,
                b'+' => Token::Plus,
                b'-' => Token::Minus,
                b'*' => Token::Asterisk,
                b'<' => Token::LessThan,
                b'>' => Token::GreaterThan,
                b'/' => Token::Slash,
                b'=' => {
                    if self.input.peek() == Some(&b'=') {
                        self.input.next();
                        return Token::Equal;
                    }
                    Token::Assign
                }
                b'!' => {
                    if self.input.peek() == Some(&b'=') {
                        self.input.next();
                        return Token::NotEqual;
                    }
                    Token::Bang
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    let identifier = self.read_token(b, |x| x.is_ascii_alphabetic() || x == b'_');
                    Token::from_str(&identifier).unwrap()
                }
                b'0'..=b'9' => {
                    // let number = self.read_number(b);
                    let number = self.read_token(b, |x| x.is_ascii_digit());
                    Token::Int(number)
                }
                b' ' | b'\n' | b'\t' | b'\r' => self.next_token(),
                _ => Token::Illegal(b.to_string()),
            }
        } else {
            Token::Eof
        }
    }

    /// Continue reading as long as the current symbol matches 0-9
    fn read_number(&mut self, b: u8) -> String {
        let mut number = vec![b];
        while let Some(&n) = self.input.peek() {
            match n {
                b'0'..=b'9' => {
                    number.push(self.input.next().unwrap());
                }
                _ => break,
            }
        }
        String::from_utf8(number).unwrap()
    }

    /// Continue reading as long as the current symbol matches a-z, A-Z, or _
    fn read_identifier(&mut self, b: u8) -> String {
        let mut ident = vec![b];
        while let Some(p) = self.input.peek() {
            match p {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    ident.push(self.input.next().unwrap());
                }
                _ => break,
            }
        }
        String::from_utf8(ident).unwrap()
    }

    /// Continue reading based on a given predicate.
    fn read_token<F>(&mut self, initial: u8, predicate: F) -> String
    where
        F: Fn(u8) -> bool,
    {
        let mut token = vec![initial];
        while let Some(&next) = self.input.peek() {
            if predicate(next) {
                token.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        String::from_utf8(token).unwrap()
    }
}
