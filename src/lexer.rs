use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let current_char = self.current_char();

        if current_char.is_alphabetic() || current_char == '_' {
            let identifier = self.read_identifier();
            return match identifier.as_str() {
                "_" => Token::Underscore,
                "let" => Token::Let,
                "if" => Token::If,
                "then" => Token::Then,
                "else" => Token::Else,
                "for" => Token::For,
                "in" => Token::In,
                "match" => Token::Match,
                "return" => Token::Return,
                "and" => Token::And,
                "or" => Token::Or,
                "not" => Token::Not,
                "true" => Token::True,
                "false" => Token::False,
                _ => Token::Identifier(identifier),
            };
        }

        if current_char.is_ascii_digit() {
            let number = self.read_number();
            println!("number: {}", number);
            return Token::Number(number.parse().unwrap());
        }

        match current_char {
            '+' => {
                self.position += 1;
                Token::Plus
            }
            '-' => {
                if let Some('>') = self.peek_char() {
                    self.position += 2;
                    Token::Arrow
                } else if let Some(c) = self.peek_char() {
                    if c.is_ascii_digit() {
                        self.position += 1;
                        let number = self.read_number();
                        return Token::Number(number.parse::<f64>().unwrap() * -1.0);
                    } else {
                        self.position += 1;
                        Token::Minus
                    }
                } else {
                    self.position += 1;
                    Token::Minus
                }
            }
            '*' => {
                self.position += 1;
                Token::Asterisk
            }
            '/' => {
                self.position += 1;
                Token::Slash
            }
            '%' => {
                self.position += 1;
                Token::Percent
            }
            '(' => {
                self.position += 1;
                Token::LeftParen
            }
            ')' => {
                self.position += 1;
                Token::RightParen
            }
            '{' => {
                self.position += 1;
                Token::LeftBrace
            }
            '}' => {
                self.position += 1;
                Token::RightBrace
            }
            '[' => {
                self.position += 1;
                Token::LeftBracket
            }
            ']' => {
                self.position += 1;
                Token::RightBracket
            }
            ',' => {
                self.position += 1;
                Token::Comma
            }
            '.' => {
                if let Some('.') = self.peek_char() {
                    self.position += 2;
                    Token::DotDot
                } else {
                    self.position += 1;
                    Token::Dot
                }
            }
            ':' => {
                self.position += 1;
                Token::Colon
            }
            '=' => {
                if let Some('=') = self.peek_char() {
                    self.position += 2;
                    Token::Equal
                } else {
                    self.position += 1;
                    Token::Assign
                }
            }
            '!' => {
                if let Some('=') = self.peek_char() {
                    self.position += 2;
                    Token::NotEqual
                } else {
                    self.position += 1;
                    Token::Invalid
                }
            }
            '<' => {
                if let Some('=') = self.peek_char() {
                    self.position += 2;
                    Token::LessThanOrEqual
                } else {
                    self.position += 1;
                    Token::LessThan
                }
            }
            '>' => {
                if let Some('=') = self.peek_char() {
                    self.position += 2;
                    Token::GreaterThanOrEqual
                } else {
                    self.position += 1;
                    Token::GreaterThan
                }
            }
            '|' => {
                if let Some('>') = self.peek_char() {
                    self.position += 2;
                    Token::ForwardPipe
                } else {
                    self.position += 1;
                    Token::Pipe
                }
            }
            '"' => {
                self.position += 1;
                let start_pos = self.position;

                while self.position < self.input.len() && self.current_char() != '"' {
                    self.position += 1;
                }

                if self.position < self.input.len() {
                    let string_value = self.input[start_pos..self.position].iter().collect();
                    self.position += 1;
                    return Token::String(string_value);
                } else {
                    return Token::Invalid; // Unterminated string
                }
            }
            _ => {
                self.position += 1;
                Token::Invalid
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
            self.position += 1;
        }

        self.input[start_pos..self.position].iter().collect()
    }
    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        let mut has_dot = false;

        while self.position < self.input.len() {
            let c = self.current_char();
            if c.is_ascii_digit() {
                self.position += 1;
            } else if c == '.' {
                if let Some('.') = self.peek_char() {
                    // Stop before interpreting the range operator `..`
                    break;
                }
                if has_dot {
                    break; // already saw a dot → stop (e.g., "1.2.3")
                }
                has_dot = true;
                self.position += 1;
            } else {
                break;
            }
        }

        self.input[start_pos..self.position].iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.position += 1;
        }
    }
}
