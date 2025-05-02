use crate::ast::Expr;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    opsition: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            opsition: 0,
        }
    }

    /**
     * Utilities
     */
    fn current_token(&self) -> &Token {
        self.tokens.get(self.opsition).unwrap_or(&Token::EOF)
    }
    fn next(&mut self) {
        self.opsition += 1;
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.current_token() {
            Token::Number(_) | Token::String(_) | Token::True | Token::False => self.literal(),
            Token::Identifier(_) => self.identifier(),
            Token::LeftParen => {
                self.next(); // consume '('
                let expr = self.expression();
                if matches!(self.current_token(), Token::RightParen) {
                    self.next(); // consume ')'
                    expr
                } else {
                    None // error: missing ')'
                }
            }
            _ => None,
        }
    }

    /**
     * Parser methods
     */
    fn expression(&mut self) -> Option<Expr> {
        match self.current_token() {
            Token::Number(n) => {
                let num = Expr::Number(*n);
                self.next();
                Some(num)
            }
            Token::String(s) => {
                let str_expr = Expr::String(s.clone());
                self.next();
                Some(str_expr)
            }
            Token::True => {
                let bool_expr = Expr::Boolean(true);
                self.next();
                Some(bool_expr)
            }
            Token::False => {
                let bool_expr = Expr::Boolean(false);
                self.next();
                Some(bool_expr)
            }
            _ => None,
        }
    }
    fn literal(&mut self) -> Option< Expr> {
        match self.current_token() {
            Token::Number(n) => {
                let num = Expr::Number(*n);
                self.next();
                Some(num)
            }
            Token::String(s) => {
                let str_expr = Expr::String(s.clone());
                self.next();
                Some(str_expr)
            }
            Token::True => {
                let bool_expr = Expr::Boolean(true);
                self.next();
                Some(bool_expr)
            }
            Token::False => {
                let bool_expr = Expr::Boolean(false);
                self.next();
                Some(bool_expr)
            }
            _ => None,
        }
    }
    fn identifier(&mut self) -> Option< Expr> {
        match self.current_token() {
            Token::Identifier(id) => {
                let id_expr = Expr::Identifier(id.clone());
                self.next();
                Some(id_expr)
            }
            _ => None,
        }
    }
}
