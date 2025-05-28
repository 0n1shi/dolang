use crate::token::{Position, Range, Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    pub line: usize,
    pub column: usize,

    start: Position,
    last: Position,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,

            start: Position { line: 1, column: 1 },
            last: Position { line: 1, column: 1 },
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = Position {
            line: self.line,
            column: self.column,
        };
        self.last = Position {
            line: self.line,
            column: self.column,
        };

        if self.position >= self.input.len() {
            return self.token(TokenType::EOF);
        }

        let current_char = self.current_char();

        if current_char.is_alphabetic() || current_char == '_' {
            let identifier = self.read_identifier();
            return match identifier.as_str() {
                "_" => self.token(TokenType::Underscore),
                "let" => self.token(TokenType::Let),
                "if" => self.token(TokenType::If),
                "fn" => self.token(TokenType::Fn),
                "then" => self.token(TokenType::Then),
                "else" => self.token(TokenType::Else),
                "for" => self.token(TokenType::For),
                "in" => self.token(TokenType::In),
                "match" => self.token(TokenType::Match),
                "return" => self.token(TokenType::Return),
                "and" => self.token(TokenType::And),
                "or" => self.token(TokenType::Or),
                "not" => self.token(TokenType::Not),
                "true" => self.token(TokenType::True),
                "false" => self.token(TokenType::False),
                _ => self.token(TokenType::Identifier(identifier)),
            };
        }

        if current_char.is_ascii_digit() {
            let number = self.read_number();
            return self.token(TokenType::Number(number.parse::<f64>().unwrap()));
        }

        match current_char {
            '+' => {
                self.consume(1);
                self.token(TokenType::Plus)
            }
            '-' => {
                if let Some('>') = self.peek_char() {
                    self.consume(2);
                    self.token(TokenType::Arrow)
                } else if let Some(c) = self.peek_char() {
                    if c.is_ascii_digit() {
                        self.consume(1);
                        let number = self.read_number();
                        self.token(TokenType::Number(number.parse::<f64>().unwrap() * -1.0))
                    } else {
                        self.consume(1);
                        self.token(TokenType::Minus)
                    }
                } else {
                    self.consume(1);
                    self.token(TokenType::Minus)
                }
            }
            '*' => {
                self.consume(1);
                self.token(TokenType::Asterisk)
            }
            '/' => {
                self.consume(1);
                self.token(TokenType::Slash)
            }
            '%' => {
                self.consume(1);
                self.token(TokenType::Percent)
            }
            '(' => {
                self.consume(1);
                self.token(TokenType::LeftParen)
            }
            ')' => {
                self.consume(1);
                self.token(TokenType::RightParen)
            }
            '{' => {
                self.consume(1);
                self.token(TokenType::LeftBrace)
            }
            '}' => {
                self.consume(1);
                self.token(TokenType::RightBrace)
            }
            '[' => {
                self.consume(1);
                self.token(TokenType::LeftBracket)
            }
            ']' => {
                self.consume(1);
                self.token(TokenType::RightBracket)
            }
            ',' => {
                self.consume(1);
                self.token(TokenType::Comma)
            }
            '.' => {
                if let Some('.') = self.peek_char() {
                    self.consume(2);
                    self.token(TokenType::DotDot)
                } else {
                    self.consume(1);
                    self.token(TokenType::Dot)
                }
            }
            ':' => {
                self.consume(1);
                self.token(TokenType::Colon)
            }
            '=' => {
                if let Some('=') = self.peek_char() {
                    self.consume(2);
                    self.token(TokenType::Equal)
                } else {
                    self.consume(1);
                    self.token(TokenType::Assign)
                }
            }
            '!' => {
                if let Some('=') = self.peek_char() {
                    self.consume(2);
                    self.token(TokenType::NotEqual)
                } else {
                    self.consume(1);
                    self.token(TokenType::Invalid)
                }
            }
            '<' => {
                if let Some('=') = self.peek_char() {
                    self.consume(2);
                    self.token(TokenType::LessThanOrEqual)
                } else {
                    self.consume(1);
                    self.token(TokenType::LessThan)
                }
            }
            '>' => {
                if let Some('=') = self.peek_char() {
                    self.consume(2);
                    self.token(TokenType::GreaterThanOrEqual)
                } else {
                    self.consume(1);
                    self.token(TokenType::GreaterThan)
                }
            }
            '|' => {
                if let Some('>') = self.peek_char() {
                    self.consume(2);
                    self.token(TokenType::ForwardPipe)
                } else {
                    self.consume(1);
                    self.token(TokenType::Pipe)
                }
            }
            '#' => {
                // Skip comments
                while self.position < self.input.len() && self.current_char() != '\n' {
                    self.consume(1);
                }
                return self.next_token();
            }
            '"' => {
                self.consume(1);
                let mut result = String::new();

                while self.position < self.input.len() && self.current_char() != '"' {
                    let c = self.current_char();
                    if c == '\\' {
                        // Escape sequence
                        self.consume(1);
                        if self.position >= self.input.len() {
                            return self.token(TokenType::Invalid);
                        }
                        let esc = self.current_char();
                        let escaped_char = match esc {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '\\' => '\\',
                            '"' => '"',
                            '0' => '\0',
                            _ => {
                                return self.token(TokenType::Invalid);
                            }
                        };
                        result.push(escaped_char);
                    } else {
                        result.push(c);
                    }
                    self.consume(1);
                }

                if self.position < self.input.len() {
                    self.consume(1); // Consume the closing quote
                    self.token(TokenType::String(result))
                } else {
                    self.token(TokenType::Invalid)
                }
            }
            _ => {
                self.consume(1);
                self.token(TokenType::Invalid)
            }
        }
    }

    fn current_char(&self) -> char {
        self.input[self.position]
    }
    fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while self.position < self.input.len()
            && (self.current_char().is_alphanumeric() || self.current_char() == '_')
        {
            self.consume(1);
        }

        self.input[start_pos..self.position].iter().collect()
    }
    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        let mut has_dot = false;

        while self.position < self.input.len() {
            let c = self.current_char();
            if c.is_ascii_digit() {
                self.consume(1);
            } else if c == '.' {
                if let Some('.') = self.peek_char() {
                    // Stop before interpreting the range operator `..`
                    break;
                }
                if has_dot {
                    break; // already saw a dot → stop (e.g., "1.2.3")
                }
                has_dot = true;
                self.consume(1);
            } else {
                break;
            }
        }

        self.input[start_pos..self.position].iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && (self.current_char().is_whitespace() || self.current_char() == '\n')
        {
            self.consume(1);
        }
    }

    fn consume(&mut self, num: usize) {
        for _ in 0..num {
            self.last = Position {
                line: self.line,
                column: self.column,
            };

            if self.position < self.input.len() {
                let current_char = self.current_char();
                self.position += 1;

                if current_char == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
            }
        }
    }

    fn token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            range: Range {
                start: self.start.clone(),
                end: self.last.clone(),
            },
        }
    }
}
