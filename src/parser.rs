use crate::ast::Expression;
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
    fn peek_token(&self) -> &Token {
        self.tokens.get(self.opsition + 1).unwrap_or(&Token::EOF)
    }
    fn next(&mut self) {
        self.opsition += 1;
    }


    /**
     * Parser methods
     */
    fn pattern(&mut self) -> Option<Expression> {
        match self.current_token() {
            Token::Number(_) | Token::String(_) | Token::True | Token::False => {
                self.literal()
            }
            Token::Identifier(_) => self.identifier(),
            Token::Underscore
        }
    }
    fn literal(&mut self) -> Option<Expression> {
        match self.current_token() {
            Token::Number(n) => {
                let num = Expression::Number(*n);
                self.next();
                Some(num)
            }
            Token::String(s) => {
                let str_expr = Expression::String(s.clone());
                self.next();
                Some(str_expr)
            }
            Token::True => {
                let bool_expr = Expression::Boolean(true);
                self.next();
                Some(bool_expr)
            }
            Token::False => {
                let bool_expr = Expression::Boolean(false);
                self.next();
                Some(bool_expr)
            }
            _ => None,
        }
    }
    fn identifier(&mut self) -> Option<Expression> {
        match self.current_token() {
            Token::Identifier(id) => {
                let id_expr = Expression::Identifier(id.clone());
                self.next();
                Some(id_expr)
            }
            _ => None,
        }
    }
}
