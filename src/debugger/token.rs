use crate::token::Token;

pub fn print_tokens(tokens: &[Token]) {
    for (i, token) in tokens.iter().enumerate() {
        println!("{:>4}: {}", i, format_token(token));
    }
}

fn format_token(token: &Token) -> String {
    use Token::*;
    match token {
        Let => "Let (let)".to_string(),
        If => "If (if)".to_string(),
        Then => "Then (then)".to_string(),
        Else => "Else (else)".to_string(),
        For => "For (for)".to_string(),
        Fn => "Fn (fn)".to_string(),
        In => "In (in)".to_string(),
        Match => "Match (match)".to_string(),
        Pipe => "Pipe (|)".to_string(),
        Underscore => "Underscore (_)".to_string(),
        ForwardPipe => "ForwardPipe (|>)".to_string(),
        Arrow => "Arrow (->)".to_string(),
        Return => "Return (return)".to_string(),
        Assign => "Assign (=)".to_string(),
        Equal => "Equal (==)".to_string(),
        NotEqual => "NotEqual (!=)".to_string(),
        LessThan => "LessThan (<)".to_string(),
        LessThanOrEqual => "LessThanOrEqual (<=)".to_string(),
        GreaterThan => "GreaterThan (>)".to_string(),
        GreaterThanOrEqual => "GreaterThanOrEqual (>=)".to_string(),
        And => "And (and)".to_string(),
        Or => "Or (or)".to_string(),
        Not => "Not (not)".to_string(),
        True => "True (true)".to_string(),
        False => "False (false)".to_string(),
        Comma => "Comma (,)".to_string(),
        Dot => "Dot (.)".to_string(),
        DotDot => "DotDot (..)".to_string(),
        Colon => "Colon (:)".to_string(),
        LeftParen => "LeftParen (()".to_string(),
        RightParen => "RightParen ())".to_string(),
        LeftBracket => "LeftBracket ([)".to_string(),
        RightBracket => "RightBracket (])".to_string(),
        LeftBrace => "LeftBrace ({)".to_string(),
        RightBrace => "RightBrace (})".to_string(),
        Identifier(s) => format!("Identifier ({s})"),
        Number(n) => format!("Number ({n})"),
        String(s) => format!("String (\"{s}\")"),
        Plus => "Plus (+)".to_string(),
        Minus => "Minus (-)".to_string(),
        Asterisk => "Asterisk (*)".to_string(),
        Slash => "Slash (/)".to_string(),
        Percent => "Percent (%)".to_string(),
        Invalid => "Invalid".to_string(),
        EOF => "EOF".to_string(),
    }
}
