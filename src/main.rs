use dolang::{eval, lexer, parser, token};
use std::io::{self, Write};

fn main() {
    println!("Welcome to Dolang :)");
    let mut env = eval::Env::new(None);

    loop {
        print!("repl> ");
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

        let mut parser = parser::Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("Error parsing input: {}", e);
                continue; // Skip to the next iteration on error
            }
        };

        let evaluator = eval::Evaluator::new(ast);
        evaluator.eval(&mut env).unwrap_or_else(|e| {
            eprintln!("Error evaluating input: {}", e);
        });
    }
}
