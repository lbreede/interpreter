use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let identifier = self.read_identifier();
                return self.lookup_identifier(&identifier);
            }
            b'0'..=b'9' => return Token::Int(self.read_number()),
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'-' => Token::Minus,
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            0 => Token::Eof,
            _ => Token::Illegal(String::from_utf8_lossy(&[self.ch]).to_string()),
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn lookup_identifier(&self, identifier: &str) -> Token {
        match identifier {
            "fn" => Token::Function,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            "return" => Token::Return,
            _ => Token::Ident(identifier.to_string()),
        }
    }
}
