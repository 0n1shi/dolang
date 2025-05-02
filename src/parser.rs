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

    fn current_token(&self) -> &Token {
        self.tokens.get(self.opsition).unwrap_or(&Token::EOF)
    }
    fn peek_token(&self) -> &Token {
        self.tokens.get(self.opsition + 1).unwrap_or(&Token::EOF)
    }
    fn next(&mut self) {
        self.opsition += 1;
    }

    fn parse_atom(&mut self) -> Option<Expression> {
        match self.current_token() {
            Token::Number(n) => {
                let num = Expression::Number(*n);
                self.next();
                Some(num)
            }
            Token::Identifier(name) => {
                let id = Expression::Identifier(name.clone());
                self.next();
                Some(id)
            }
            Token::String(s) => {
                let str_expr = Expression::String(s.clone());
                self.next();
                Some(str_expr)
            }
            _ => None,
        }
    }
}

