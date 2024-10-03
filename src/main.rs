// mod ast;
mod lexer;
mod lexer_book_style;
mod lexer_boxed_slice;
mod lexer_peekable_chars;
mod repl;
mod token;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
    repl::start();
}
