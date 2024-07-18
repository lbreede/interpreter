use crate::token::Token;
use std::iter::Peekable;
use std::str::Bytes;
use std::str::FromStr;

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
                b';' => return Token::Semicolon,
                b'(' => return Token::LParen,
                b')' => return Token::RParen,
                b',' => return Token::Comma,
                b'{' => return Token::LSquirly,
                b'}' => return Token::RSquirly,
                b'+' => return Token::Plus,
                b'-' => return Token::Minus,
                b'*' => return Token::Asterisk,
                b'<' => return Token::LessThan,
                b'>' => return Token::GreaterThan,
                b'/' => return Token::Slash,
                b'=' => {
                    if self.input.peek() == Some(&b'=') {
                        self.input.next();
                        return Token::Equal;
                    }
                    return Token::Assign;
                }
                b'!' => {
                    if self.input.peek() == Some(&b'=') {
                        self.input.next();
                        return Token::NotEqual;
                    }
                    return Token::Bang;
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    let identifier = self.read_token(b, |x| x.is_ascii_alphabetic() || x == b'_');
                    return Token::from_str(&identifier).unwrap();
                }
                b'0'..=b'9' => {
                    // let number = self.read_number(b);
                    let number = self.read_token(b, |x| x.is_ascii_digit());
                    return Token::Int(number);
                }
                b' ' | b'\n' | b'\t' | b'\r' => self.next_token(),
                _ => return Token::Illegal(b.to_string()),
            }
        } else {
            return Token::Eof;
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
