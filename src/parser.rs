use crate::ast::{AdditionOperator, ComparisonOperator, Expression, MultiplicationOperator};
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

    pub fn parse(&mut self) -> Result<Expression, String> {
        let expr = self.parse_expression()?;
        if self.current_token().is_eof() {
            Ok(expr)
        } else {
            Err(format!("Unexpected token: {:?}", self.current_token()))
        }
    }

    /**
     * Parsing methods
     */
    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_term()?;
}
