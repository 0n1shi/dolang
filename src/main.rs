use dolang::{lexer, parser, token};
use dolang::eval::env::Env;
use dolang::eval::eval::Evaluator;
use std::io::{self, Write};

const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "i" => repl(),
            "h" | "help" => help(),
            "v" | "version" => version(),
            _ => run(&args[1]),
        }
    } else {
        help();
    }
}

fn run(filename: &str) {
    let source = std::fs::read_to_string(filename).expect("Failed to read file");
    let mut lexer = lexer::Lexer::new(&source);
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
            return;
        }
    };

    let evaluator = Evaluator::new(ast);
    evaluator
        .eval(&mut Env::new(None))
        .unwrap_or_else(|e| {
            eprintln!("Error evaluating input: {}", e);
        });
}

fn repl() {
    println!("Welcome to Dolang :)");
    let mut env = Env::new(None);

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

        let evaluator = Evaluator::new(ast);
        evaluator.eval(&mut env).unwrap_or_else(|e| {
            eprintln!("Error evaluating input: {}", e);
        });
    }
}

fn help() {
    println!("Dolang - A simple programming language");
    println!("Usage:");
    println!("  <filename>: Run a Dolang script");
    println!("  i: Start the Dolang REPL");
    println!("  h, help: Show this help message");
    println!("  v, version: Show the version of Dolang");
}

fn version() {
    println!("version {}", VERSION);
}
