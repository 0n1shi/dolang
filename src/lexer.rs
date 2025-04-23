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

        if current_char.is_alphabetic() {
            return Token::Identifier(self.read_identifier());
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
                Token::Minus
            }
            '*' => {
                self.position += 1;
                Token::Multiply
            }
            '/' => {
                self.position += 1;
                Token::Divide
            }
            '(' => {
                self.position += 1;
                Token::LParen
            }
            ')' => {
                self.position += 1;
                Token::RParen
            }
            '{' => {
                self.position += 1;
                Token::LBrace
            }
            '}' => {
                self.position += 1;
                Token::RBrace
            }
            '[' => {
                self.position += 1;
                Token::LBracket
            }
            ']' => {
                self.position += 1;
                Token::RBracket
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
                    Token::Not
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

        while self.position < self.input.len() && self.current_char().is_alphabetic() {
            self.position += 1;
        }

        self.input[start_pos..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;

        while self.position < self.input.len() && self.current_char().is_digit(10) {
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
