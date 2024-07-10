use crate::lexer::Lexer;
use crate::token::Token;
use std::io;
use std::io::Write;

pub fn start() {
    loop {
        let mut input = String::new();
        print!(">> ");
        let _ = io::stdout().flush();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }
        // Trim the input to handle extra newlines and spaces
        let input = input.trim();

        // Add an exit condition
        if input == "exit" {
            println!("Exiting...");
            break;
        }

        let mut lex = Lexer::new(input.to_string());
        let mut tok = lex.next_token();

        while tok != Token::Eof {
            println!("{:?}", tok);
            tok = lex.next_token();
        }
        println!("{:?}", tok);
    }
}
