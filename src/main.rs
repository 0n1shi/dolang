mod lexer;
mod token;

use std::io::{self, Write};

fn main() {
    println!("Welcome to Dolang :)");
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            continue; // Skip empty lines
        }

        if ["exit", "quit", "q"].contains(&trimmed_input) {
            println!("Exiting Dolang. Goodbye!");
            break; // Exit the loop on "exit"
        }

        let mut lexer = lexer::Lexer::new(trimmed_input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            if token == token::Token::EOF {
                break; // Stop on EOF
            }
            tokens.push(token);
        }

        println!("You entered: {}", trimmed_input);
        println!("Tokens: {:?}", tokens);
    }
}
