#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,                // let
    Fn,                 // fn
    If,                 // if
    Else,               // else
    For,                // for
    In,                 // in
    Match,              // match
    Pipe,               // |
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
    LParen,             // (
    RParen,             // )
    LBracket,           // [
    RBracket,           // ]
    LBrace,             // {
    RBrace,             // }
    Identifier(String), // variable names
    Number(f64),        // numbers
    String(String),     // strings
    Plus,               // +
    Minus,              // -
    Multiply,           // *
    Divide,             // /
    Modulus,            // %
    EOF,
}
