use interpreter::lexer::Lexer;
use interpreter::lexer_book_style::Lexer as LexerBookStyle;
use interpreter::lexer_peekable_bytes::Lexer as LexerPeekableBytes;
use interpreter::token::Token;

pub trait Lexable {
    fn next_token(&mut self) -> Token;
}

impl<'a> Lexable for Lexer<'a> {
    fn next_token(&mut self) -> Token {
        self.next_token()
    }
}
impl<'a> Lexable for LexerPeekableBytes<'a> {
    fn next_token(&mut self) -> Token {
        self.next_token()
    }
}
impl Lexable for LexerBookStyle {
    fn next_token(&mut self) -> Token {
        // Delegate to the existing implementation
        self.next_token()
    }
}

fn test_lexer<L: Lexable>(lex: &mut L, tokens: &[Token]) {
    for expected_token in tokens {
        let token = lex.next_token();
        println!("expected: {:?}, received {:?}", expected_token, token);
        assert_eq!(*expected_token, token);
    }
}

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

    test_lexer(&mut Lexer::new(input), &tokens);
    test_lexer(&mut LexerBookStyle::new(input.to_string()), &tokens);
    test_lexer(&mut LexerPeekableBytes::new(input), &tokens);
}
