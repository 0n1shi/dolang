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

        if current_char.is_alphabetic() || self.current_char() == '_' {
            let identifier = self.read_identifier();
            match identifier.as_str() {
                "_" => return Token::Underscore,
                "let" => return Token::Let,
                "fn" => return Token::Fn,
                "if" => return Token::If,
                "then" => return Token::Then,
                "else" => return Token::Else,
                "for" => return Token::For,
                "in" => return Token::In,
                "match" => return Token::Match,
                "return" => return Token::Return,
                "and" => return Token::And,
                "or" => return Token::Or,
                "not" => return Token::Not,
                "true" => return Token::True,
                "false" => return Token::False,
                _ => return Token::Identifier(identifier),
            }
        }

        if current_char.is_digit(10) {
            let number = self.read_number();
            return Token::Number(number.parse().unwrap());
        }

        match current_char {
            '+' => {
                self.position += 1;
                Token::Plus
            }
            '-' => {
                self.position += 1;
                if self.position < self.input.len() && self.current_char() == '>' {
                    self.position += 1;
                    Token::Arrow
                } else if self.position < self.input.len() && self.current_char().is_digit(10) {
                    let number = self.read_number();
                    return Token::Number(number.parse::<f64>().unwrap() * -1.0);
                } else {
                    Token::Minus
                }
            }
            '*' => {
                self.position += 1;
                Token::Multiply
            }
            '/' => {
                self.position += 1;
                Token::Divide
            }
            '%' => {
                self.position += 1;
                Token::Modulus
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
                self.position += 1;
                Token::Dot
            }
            '=' => {
                self.position += 1;
                if self.position < self.input.len() && self.current_char() == '=' {
                    self.position += 1;
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            '!' => {
                self.position += 1;
                if self.position < self.input.len() && self.current_char() == '=' {
                    self.position += 1;
                    Token::NotEqual
                } else {
                    Token::Invalid // Handle invalid token
                }
            }
            '<' => {
                self.position += 1;
                if self.position < self.input.len() && self.current_char() == '=' {
                    self.position += 1;
                    Token::LessThanOrEqual
                } else {
                    Token::LessThan
                }
            }
            '>' => {
                self.position += 1;
                if self.position < self.input.len() && self.current_char() == '=' {
                    self.position += 1;
                    Token::GreaterThanOrEqual
                } else {
                    Token::GreaterThan
                }
            }
            '|' => {
                self.position += 1;
                if self.position < self.input.len() && self.current_char() == '>' {
                    self.position += 1;
                    Token::ForwardPipe
                } else {
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
                    self.position += 1; // Skip closing quote
                    return Token::String(string_value);
                } else {
                    return Token::Invalid; // Handle unterminated string
                }
            }
            _ => {
                self.position += 1;
                Token::EOF // Handle unknown characters
            }
        }
    }

    fn current_char(&self) -> char {
        self.input[self.position]
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

        while self.position < self.input.len()
            && (self.current_char().is_digit(10) || self.current_char() == '.')
        {
            self.position += 1;
        }

        self.input[start_pos..self.position].iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.position += 1;
        }
    }
}
