use crate::ast::{Expr, Stmt, AST};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    /**
     * Utilities
     */
    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }
    fn next(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Result<AST, String> {
        let mut stmts = Vec::new();
        while self.current_token() != &Token::EOF {
            match self.parse_statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => return Err(format!("Error parsing statement: {}", e)),
            }
            if self.current_token() == &Token::EOF {
                break;
            }
        }
        Ok(AST { stmts })
    }

    /**
     * Parsing methods
     */
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.current_token() {
            Token::Let => self.parse_let_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }
    fn parse_let_stmt(&mut self) -> Result<Stmt, String> {
        self.next(); // Consume 'let'
        let name = match self.current_token() {
            Token::Identifier(id) => id.clone(),
            _ => return Err("Expected identifier after 'let'".into()),
        };
        self.next(); // Consume identifier
        if self.current_token() != &Token::Assign {
            return Err("Expected '=' after identifier".into());
        }
        self.next(); // Consume '='
        let val = self.parse_expr()?;
        Ok(Stmt::Let { name, val })
    }
    fn parse_expr_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr()?;
        Ok(Stmt::Expr(expr))
    }
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.next(); // Consume number | string | identifier | "true" | "false"
        match self.current_token() {
            Token::Number(n) => Ok(Expr::Number(*n)),
            Token::String(s) => Ok(Expr::String(s.clone())),
            Token::True => Ok(Expr::Boolean(true)),
            Token::False => Ok(Expr::Boolean(false)),
            Token::Identifier(id) => Ok(Expr::Identifier(id.clone())),
            _ => Err("Expected expression".into()),
        }
    }
}
