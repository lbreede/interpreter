mod lexer_peekable_chars;
mod repl;
mod token;

use whoami;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
    repl::start();
}
