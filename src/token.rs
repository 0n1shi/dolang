#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub char: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Let,                // let
    If,                 // if
    Then,               // then
    Else,               // else
    For,                // for
    Fn,                 // fn
    In,                 // in
    Match,              // match
    Pipe,               // |
    Underscore,         // _
    ForwardPipe,        // |>
    Arrow,              // ->
    Return,             // return
    Assign,             // =
    Equal,              // ==
    NotEqual,           // !=
    LessThan,           // <
    LessThanOrEqual,    // <=
    GreaterThan,        // >
    GreaterThanOrEqual, // >=
    And,                // and
    Or,                 // or
    Not,                // not
    True,               // true
    False,              // false
    Comma,              // ,
    Dot,                // .
    DotDot,             // ..
    Colon,              // :
    LeftParen,          // (
    RightParen,         // )
    LeftBracket,        // [
    RightBracket,       // ]
    LeftBrace,          // {
    RightBrace,         // }
    Identifier(String), // variable names
    Number(f64),        // numbers
    String(String),     // strings
    Plus,               // +
    Minus,              // -
    Asterisk,           // *
    Slash,              // /
    Percent,            // %
    Invalid,            // invalid token
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub range: Range,
}

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self.token_type, TokenType::EOF)
    }
}
