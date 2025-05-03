#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,                // let
    If,                 // if
    Then,               // then
    Else,               // else
    For,                // for
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

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self, Token::EOF)
    }
}
