use dolang::debugger::ast::print_ast;
use dolang::debugger::token::print_tokens;
use dolang::eval::env::Env;
use dolang::eval::eval::eval;
use dolang::lsp::lsp::Backend;
use dolang::{lexer, parser, token};
use std::io::{self, Write};

use tower_lsp::{LspService, Server};

const VERSION: &str = "0.1.0";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut debug = false;
    let mut file_name = String::new();
    let mut repl = false;
    let mut lsp = false;
    let mut help = false;
    let mut version = false;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "i" => {
                repl = true;
            }
            "h" | "help" => {
                help = true;
            }
            "v" | "version" => {
                version = true;
            }
            "l" | "lsp" => {
                lsp = true;
            }
            "-d" | "--debug" => {
                debug = true;
            }
            _ => {
                file_name = args[1].clone();
            }
        }
    }

    if debug {
        println!("[+] Debug mode enabled");
    }

    if help {
        show_help();
        return;
    }
    if version {
        show_version();
        return;
    }
    if lsp {
        run_lsp().await;
        return;
    }
    if repl {
        run_repl(debug);
        return;
    }

    if file_name.is_empty() {
        eprintln!("[!] No input file provided. Use 'i' for REPL or 'h' for help.");
        return;
    }
    if !file_name.ends_with(".do") {
        eprintln!("[!] Invalid file extension. Please use a .dolang file.");
        return;
    }
    run_file(&file_name, debug);
}

fn run_file(filename: &str, debug: bool) {
    let source = std::fs::read_to_string(filename).expect("[!] Failed to read file");
    let mut lexer = lexer::Lexer::new(&source);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token.token_type == token::TokenType::EOF {
            break; // Stop on EOF
        }
        tokens.push(token);
    }
    if debug {
        print_tokens(&tokens);
    }

    let mut parser = parser::Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("[!] Error parsing input: {}", e);
            return;
        }
    };
    if debug {
        print_ast(&ast);
    }

    eval(ast, &mut Env::new(None)).unwrap_or_else(|e| {
        eprintln!("[!] Error evaluating input: {}", e);
    });
}

fn run_repl(debug: bool) {
    println!("[*] Welcome to Dolang :)");
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
            if token.token_type == token::TokenType::EOF {
                break; // Stop on EOF
            }
            tokens.push(token);
        }
        if debug {
            println!("Tokens: {:?}", tokens);
        }

        let mut parser = parser::Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("Error parsing input: {}", e);
                continue; // Skip to the next iteration on error
            }
        };
        if debug {
            println!("Parsed AST: {:?}", ast);
        }

        eval(ast, &mut env).unwrap_or_else(|e| {
            eprintln!("Error evaluating input: {}", e);
        });
    }
}

async fn run_lsp() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}

fn show_help() {
    println!("Dolang - A simple programming language");
    println!("Usage:");
    println!("  <filename>: Run a Dolang script");
    println!("  i, repl: Start the Dolang REPL (Read-Eval-Print Loop)");
    println!("  l, lsp: Start the Dolang Language Server Protocol (LSP)");
    println!("  h, help: Show this help message");
    println!("  v, version: Show the version of Dolang");
}

fn show_version() {
    println!("version {}", VERSION);
}
