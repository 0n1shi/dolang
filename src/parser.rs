use crate::ast::{CompOp, Expr, FactorOp, LogicOp, Stmt, TermOp, UnaryOp, AST};
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
            Token::Print => self.parse_print_stmt(),
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
    fn parse_print_stmt(&mut self) -> Result<Stmt, String> {
        self.next(); // Consume 'print'
        let expr = self.parse_expr()?;
        Ok(Stmt::Print(expr))
    }
    fn parse_expr_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr()?;
        Ok(Stmt::Expr(expr))
    }
    fn parse_expr(&mut self) -> Result<Expr, String> {
        if self.current_token() == &Token::LeftBracket {
            self.next(); // Consume '['
            let mut elements = Vec::new();
            while self.current_token() != &Token::RightBracket {
                let expr = self.parse_expr()?;
                elements.push(expr);
                if self.current_token() == &Token::Comma {
                    self.next(); // Consume ','
                } else {
                    break;
                }
            }
            if self.current_token() != &Token::RightBracket {
                return Err("Expected ']'".into());
            }
            self.next(); // Consume ']'
            return Ok(Expr::List(elements));
        } else if self.current_token() == &Token::LeftParen {
            self.next(); // Consume '('
            let mut elements = Vec::new();
            while self.current_token() != &Token::RightParen {
                let expr = self.parse_expr()?;
                elements.push(expr);
                if self.current_token() == &Token::Comma {
                    self.next(); // Consume ','
                } else {
                    break;
                }
            }
            if self.current_token() != &Token::RightParen {
                return Err("Expected ')'".into());
            }
            self.next(); // Consume ')'
            return Ok(Expr::Tuple(elements));
        } else if self.current_token() == &Token::If {
            self.next(); // Consume 'if'
            let cond = self.parse_expr()?;

            if self.current_token() != &Token::Then {
                return Err("Expected 'then' after 'if' condition".into());
            }
            self.next(); // Consume 'then'
            let then_branch = self.parse_expr()?;

            if self.current_token() != &Token::Else {
                return Err("Expected 'else' after 'then' branch".into());
            }
            self.next(); // Consume 'else'
            let else_branch = self.parse_expr()?;

            return Ok(Expr::If {
                cond: Box::new(cond),
                then: Box::new(then_branch),
                else_: Box::new(else_branch),
            });
        } else {
            return self.parse_logic_expr();
        }
    }
    fn parse_logic_expr(&mut self) -> Result<Expr, String> {
        let left = self.parse_comp_expr()?;

        if self.current_token() == &Token::And || self.current_token() == &Token::Or {
            let op = match self.current_token() {
                Token::And => LogicOp::And,
                Token::Or => LogicOp::Or,
                _ => unreachable!(),
            };
            self.next(); // Consume 'and'

            let right = self.parse_comp_expr()?;

            Ok(Expr::Logic {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }
    fn parse_comp_expr(&mut self) -> Result<Expr, String> {
        let left = self.parse_term_expr()?;
        if [
            Token::Equal,
            Token::NotEqual,
            Token::LessThan,
            Token::LessThanOrEqual,
            Token::GreaterThan,
            Token::GreaterThanOrEqual,
        ]
        .contains(self.current_token())
        {
            let op = match self.current_token() {
                Token::Equal => CompOp::Equal,
                Token::NotEqual => CompOp::NotEqual,
                Token::LessThan => CompOp::LessThan,
                Token::LessThanOrEqual => CompOp::LessThanOrEqual,
                Token::GreaterThan => CompOp::GreaterThan,
                Token::GreaterThanOrEqual => CompOp::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            self.next(); // Consume comparison operator

            let right = self.parse_term_expr()?;

            Ok(Expr::Comp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }
    fn parse_term_expr(&mut self) -> Result<Expr, String> {
        let left = self.parse_factor_expr()?;
        if [Token::Plus, Token::Minus].contains(self.current_token()) {
            let op = match self.current_token() {
                Token::Plus => TermOp::Plus,
                Token::Minus => TermOp::Minus,
                _ => unreachable!(),
            };
            self.next(); // Consume '+' or '-'

            let right = self.parse_factor_expr()?;

            Ok(Expr::Term {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }
    fn parse_factor_expr(&mut self) -> Result<Expr, String> {
        let left = self.parse_unary_expr()?;
        if [Token::Asterisk, Token::Slash, Token::Percent].contains(self.current_token()) {
            let op = match self.current_token() {
                Token::Asterisk => FactorOp::Multiply,
                Token::Slash => FactorOp::Divide,
                Token::Percent => FactorOp::Modulus,
                _ => unreachable!(),
            };
            self.next(); // Consume '*' or '/'

            let right = self.parse_unary_expr()?;

            Ok(Expr::Factor {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }
    fn parse_unary_expr(&mut self) -> Result<Expr, String> {
        if [Token::Minus, Token::Not].contains(self.current_token()) {
            let op = match self.current_token() {
                Token::Minus => UnaryOp::Minus,
                Token::Not => UnaryOp::Not,
                _ => unreachable!(),
            };
            self.next(); // Consume '-' or 'not'

            let right = self.parse_unary_expr()?;

            Ok(Expr::Unary {
                op,
                right: Box::new(right),
            })
        } else {
            self.parse_primary_expr()
        }
    }
    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        let curr_tok = self.current_token().clone();
        match curr_tok {
            Token::Identifier(id) => {
                self.next(); // Consume identifier

                if self.current_token() == &Token::LeftBracket {
                    self.next(); // Consume '['

                    let index = match self.current_token() {
                        Token::Number(n) => *n,
                        _ => return Err("Expected number for list index".into()),
                    };
                    self.next(); // Consume number

                    if self.current_token() != &Token::RightBracket {
                        return Err("Expected ']'".into());
                    }
                    self.next(); // Consume ']'

                    return Ok(Expr::ListAccess {
                        list: Box::new(Expr::Identifier(id.clone())),
                        index,
                    });
                }

                Ok(Expr::Identifier(id.clone()))
            }
            Token::Number(n) => {
                self.next(); // Consume number
                Ok(Expr::Number(n))
            }
            Token::String(s) => {
                self.next(); // Consume string
                Ok(Expr::String(s.clone()))
            }
            Token::True => {
                self.next(); // Consume 'true'
                Ok(Expr::Boolean(true))
            }
            Token::False => {
                self.next(); // Consume 'false'
                Ok(Expr::Boolean(false))
            }
            Token::LeftParen => {
                self.next(); // Consume '('

                let expr = self.parse_expr()?;

                if self.current_token() != &Token::RightParen {
                    return Err("Expected ')'".into());
                }
                self.next(); // Consume ')'

                Ok(expr)
            }
            _ => Err("Expected identifier, number, string, or '('".into()),
        }
    }
}
