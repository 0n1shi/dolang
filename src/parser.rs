use crate::ast::{Case, CompOp, Expr, FactorOp, LogicOp, Pattern, Stmt, TermOp, UnaryOp, AST};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }

    /**
     * Utilities
     */
    pub fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }
    fn current_token_type(&self) -> &TokenType {
        self.tokens.get(self.position).map_or(&TokenType::EOF, |t| &t.token_type)
    }
    fn next(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Result<AST, String> {
        let mut stmts = Vec::new();
        while self.current_token_type() != &Token::EOF {
            match self.parse_statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => return Err(format!("Error parsing statement: {}", e)),
            }
            if self.current_token_type() == &Token::EOF {
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
                     //
        let name = match self.current_token_type() {
            TokenType::Identifier(id) => id.clone(),
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
        if self.current_token() == &Token::If {
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
        } else if self.current_token() == &Token::Match {
            self.next(); // Consume 'match'

            let cond = self.parse_expr()?;

            let mut cases = Vec::new();
            while self.current_token() == &Token::Pipe {
                self.next(); // Consume '|'

                let pattern = match self.current_token() {
                    Token::Number(n) => Pattern::Number(*n),
                    Token::String(s) => Pattern::String(s.clone()),
                    Token::True => Pattern::Boolean(true),
                    Token::False => Pattern::Boolean(false),
                    Token::Underscore => Pattern::Wildcard,
                    _ => return Err("Expected pattern after '|'".into()),
                };
                self.next(); // Consume pattern

                if self.current_token() != &Token::Arrow {
                    return Err("Expected '->' after pattern".into());
                }
                self.next(); // Consume '->'

                let body = self.parse_expr()?;
                cases.push(Case { pattern, body });
            }
            if cases.is_empty() {
                return Err("Expected at least one case after 'match'".into());
            }
            return Ok(Expr::Match {
                cond: Box::new(cond),
                cases,
            });
        } else if self.current_token() == &Token::Fn {
            self.next(); // Consume 'fn'

            if self.current_token() == &Token::Underscore {
                self.next(); // Consume '_'
            }

            let mut params = Vec::new();
            while self.current_token() != &Token::Arrow {
                match self.current_token() {
                    Token::Identifier(id) => {
                        params.push(id.clone());
                        self.next(); // Consume identifier
                    }
                    _ => return Err("Expected identifier in function arguments".into()),
                }
                if self.current_token() == &Token::Comma {
                    self.next(); // Consume ','
                } else {
                    break;
                }
            }

            if self.current_token() != &Token::Arrow {
                return Err("Expected '->' after function arguments".into());
            }
            self.next(); // Consume '->'

            let body = self.parse_expr()?;
            return Ok(Expr::Func {
                params,
                body: Box::new(body),
            });
        } else {
            return self.parse_pipe_expr();
        }
    }
    fn parse_pipe_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_logic_expr()?;

        while self.current_token() == &Token::ForwardPipe {
            self.next(); // Consume '|>'

            let right = self.parse_logic_expr()?;

            expr = Expr::Pipe {
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
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
        let left = self.parse_range_expr()?;
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

            let right = self.parse_range_expr()?;

            Ok(Expr::Comp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }
    fn parse_range_expr(&mut self) -> Result<Expr, String> {
        let start = self.parse_term_expr()?;
        if self.current_token() == &Token::DotDot {
            self.next(); // Consume '..'

            let end = self.parse_term_expr()?;

            Ok(Expr::Range {
                start: Box::new(start),
                end: Box::new(end),
            })
        } else {
            Ok(start)
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

                match self.current_token() {
                    // list access
                    Token::LeftBracket => {
                        self.next(); // Consume '['

                        let mut start = None;
                        let mut has_dots = false;
                        let mut end = None;

                        while self.current_token() != &Token::RightBracket {
                            match self.current_token() {
                                Token::DotDot => {
                                    has_dots = true;
                                    self.next(); // Consume '..'
                                }
                                _ => {
                                    if has_dots {
                                        end = Some(Box::new(self.parse_primary_expr().map_err(
                                            |err| format!("Expected expression: {}", err),
                                        )?));
                                    } else {
                                        start = Some(Box::new(self.parse_primary_expr().map_err(
                                            |err| format!("Expected expression: {}", err),
                                        )?));
                                    }
                                }
                            }
                        }
                        if self.current_token() != &Token::RightBracket {
                            return Err("Expected ']'".into());
                        }
                        self.next(); // Consume ']'

                        let list_access = if has_dots {
                            if start.is_none() && end.is_none() {
                                return Err("Expected start or end for slice".into());
                            }
                            Expr::Slice {
                                list: Box::new(Expr::Identifier(id.clone())),
                                start,
                                end,
                            }
                        } else {
                            Expr::Index {
                                list: Box::new(Expr::Identifier(id.clone())),
                                index: start.ok_or_else(|| "Expected index for list access")?,
                            }
                        };
                        return Ok(list_access);
                    }
                    // function call
                    Token::LeftParen => {
                        self.next(); // Consume '('

                        let mut args = Vec::new();
                        while self.current_token() != &Token::RightParen {
                            let arg = self.parse_expr()?;
                            args.push(arg);
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

                        return Ok(Expr::Call {
                            name: Box::new(Expr::Identifier(id.clone())),
                            args,
                        });
                    }
                    // record access
                    Token::Dot => {
                        self.next(); // Consume '.'

                        let field = match self.current_token() {
                            Token::Identifier(field_name) => field_name.clone(),
                            _ => return Err("Expected identifier after '.'".into()),
                        };
                        self.next(); // Consume identifier

                        return Ok(Expr::Access {
                            record: Box::new(Expr::Identifier(id.clone())),
                            field,
                        });
                    }
                    _ => {
                        return Ok(Expr::Identifier(id.clone()));
                    }
                }
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
            Token::LeftBracket => {
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

                Ok(Expr::List(elements))
            }
            // record
            Token::LeftBrace => {
                self.next(); // Consume '{'

                let mut fields = Vec::new();
                while self.current_token() != &Token::RightBrace {
                    let curr_tok = self.current_token().clone();
                    match curr_tok {
                        Token::Identifier(field_name) => {
                            self.next(); // Consume field name
                            if self.current_token() != &Token::Colon {
                                return Err("Expected ':' after field name".into());
                            }
                            self.next(); // Consume ':'

                            let field_value = self.parse_expr()?;
                            fields.push((field_name.clone(), field_value));
                        }
                        _ => return Err("Expected identifier for field name".into()),
                    }
                    if self.current_token() == &Token::Comma {
                        self.next(); // Consume ','
                    } else {
                        break;
                    }
                }
                if self.current_token() != &Token::RightBrace {
                    return Err("Expected '}'".into());
                }
                self.next(); // Consume '}'

                return Ok(Expr::Record(fields));
            }
            _ => Err(format!(
                "Expected identifier, number, string, true, false, or '(' but found: {:?}",
                self.current_token()
            )),
        }
    }
}
