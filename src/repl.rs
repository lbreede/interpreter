use crate::lexer::Lexer;
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

        let mut lexer = Lexer::new(input);
        while let Some(token) = lexer.next_token() {
            println!("{:?}", token);
        }
        println!("Eof");

        // let mut parser = Parser::new(input);
        // println!("{:?}", parser.parse());
    }
}
