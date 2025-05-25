use crate::token::{TokenType, Token};

pub fn print_tokens(tokens: &[Token]) {
    for (i, token) in tokens.iter().enumerate() {
        println!("{:>4}: {}", i, format_token(token));
    }
}

fn format_token(token: &Token) -> String {
    let token_type = token.token_type.clone();
    match token_type {
        TokenType::Let => "Let (let)".to_string(),
        TokenType::If => "If (if)".to_string(),
        TokenType::Then => "Then (then)".to_string(),
        TokenType::Else => "Else (else)".to_string(),
        TokenType::For => "For (for)".to_string(),
        TokenType::Fn => "Fn (fn)".to_string(),
        TokenType::In => "In (in)".to_string(),
        TokenType::Match => "Match (match)".to_string(),
        TokenType::Pipe => "Pipe (|)".to_string(),
        TokenType::Underscore => "Underscore (_)".to_string(),
        TokenType::ForwardPipe => "ForwardPipe (|>)".to_string(),
        TokenType::Arrow => "Arrow (->)".to_string(),
        TokenType::Return => "Return (return)".to_string(),
        TokenType::Assign => "Assign (=)".to_string(),
        TokenType::Equal => "Equal (==)".to_string(),
        TokenType::NotEqual => "NotEqual (!=)".to_string(),
        TokenType::LessThan => "LessThan (<)".to_string(),
        TokenType::LessThanOrEqual => "LessThanOrEqual (<=)".to_string(),
        TokenType::GreaterThan => "GreaterThan (>)".to_string(),
        TokenType::GreaterThanOrEqual => "GreaterThanOrEqual (>=)".to_string(),
        TokenType::And => "And (and)".to_string(),
        TokenType::Or => "Or (or)".to_string(),
        TokenType::Not => "Not (not)".to_string(),
        TokenType::True => "True (true)".to_string(),
        TokenType::False => "False (false)".to_string(),
        TokenType::Comma => "Comma (,)".to_string(),
        TokenType::Dot => "Dot (.)".to_string(),
        TokenType::DotDot => "DotDot (..)".to_string(),
        TokenType::Colon => "Colon (:)".to_string(),
        TokenType::LeftParen => "LeftParen (()".to_string(),
        TokenType::RightParen => "RightParen ())".to_string(),
        TokenType::LeftBracket => "LeftBracket ([)".to_string(),
        TokenType::RightBracket => "RightBracket (])".to_string(),
        TokenType::LeftBrace => "LeftBrace ({)".to_string(),
        TokenType::RightBrace => "RightBrace (})".to_string(),
        TokenType::Identifier(s) => format!("Identifier ({s})"),
        TokenType::Number(n) => format!("Number ({n})"),
        TokenType::String(s) => format!("String (\"{s}\")"),
        TokenType::Plus => "Plus (+)".to_string(),
        TokenType::Minus => "Minus (-)".to_string(),
        TokenType::Asterisk => "Asterisk (*)".to_string(),
        TokenType::Slash => "Slash (/)".to_string(),
        TokenType::Percent => "Percent (%)".to_string(),
        TokenType::Invalid => "Invalid".to_string(),
        TokenType::EOF => "EOF".to_string(),
    }
}
