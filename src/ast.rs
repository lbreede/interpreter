use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    IntegerLiteral(i64),
    IdentifierLiteral(String),
}
#[derive(Debug, PartialEq, Eq)]
pub struct LetStatement {
    variable: String,
    expression: Expression,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        tokens.reverse();
        Self { tokens }
    }

    fn next(&mut self) -> Token {
        if let Some(token) = self.tokens.pop() {
            token
        } else {
            Token::Eof
        }
    }

    fn peek(&mut self) -> Token {
        if let Some(_token) = self.tokens.last() {
            self.tokens.clone().last().unwrap().clone()
        } else {
            Token::Eof
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        let token = self.peek();
        match token {
            Token::Let => statements.push(self.parse_let_statement()),
            _ => panic!("bad token {:?}", token),
        }
        statements
    }
    fn parse_let_statement(&mut self) -> Statement {
        let token = self.next();
        assert_eq!(token, Token::Let);
        let variable = match self.next() {
            Token::Ident(it) => it,
            _ => panic!("bad token {:?}", token),
        };
        let token = self.next();
        assert_eq!(token, Token::Assign);
        let expression = self.parse_expression();
        let token = self.next();
        assert_eq!(token, Token::Semicolon);
        Statement::Let(LetStatement {
            variable,
            expression,
        })
    }

    fn parse_expression(&mut self) -> Expression {
        match self.next() {
            Token::Int(number) => Expression::IntegerLiteral(number.parse().unwrap()),
            Token::Ident(ident) => Expression::IdentifierLiteral(ident),
            _ => panic!("bad token {:?}", self.next()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut parser = Parser::new("let foo = 5;");
        let statements = parser.parse();
        let expected_statements = vec![Statement::Let(LetStatement {
            variable: "foo".to_string(),
            expression: Expression::IntegerLiteral(5),
        })];
        for (statement, expected_statements) in statements.into_iter().zip(expected_statements) {
            assert_eq!(statement, expected_statements);
        }
    }
}
