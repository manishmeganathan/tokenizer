mod lexer;
mod token;
mod utils;
mod readers;

use std::io::{self, Write};

fn main() {
    print!("Welcome to the Tokenizer. Input any series of characters.\n");
    loop {
        print!("lexer >: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");

        let mut l = lexer::Lexer::new(input.chars().collect());
        let tokens = l.lex();

        for token in tokens.into_iter() {
            println!("{}", token);
        }
    }    
}