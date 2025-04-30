#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,                // let
    Fn,                 // fn
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
    Multiply,           // *
    Divide,             // /
    Modulus,            // %
    Invalid,            // invalid token
    EOF,
}
