use dolang::lexer::Lexer;
use dolang::token::Token;

#[test]
fn test_lexer() {
    let test_cases = vec![
        (
            "let if then else for fn in match | _ |> -> return = == != < <= > >= and or not true false, . .. : ( ) [ ] { }",
            vec![
                Token::Let,
                Token::If,
                Token::Then,
                Token::Else,
                Token::For,
                Token::Fn,
                Token::In,
                Token::Match,
                Token::Pipe,
                Token::Underscore,
                Token::ForwardPipe,
                Token::Arrow,
                Token::Return,
                Token::Assign,
                Token::Equal,
                Token::NotEqual,
                Token::LessThan,
                Token::LessThanOrEqual,
                Token::GreaterThan,
                Token::GreaterThanOrEqual,
                Token::And,
                Token::Or,
                Token::Not,
                Token::True,
                Token::False,
                Token::Comma,
                Token::Dot,
                Token::DotDot,
                Token::Colon,
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBracket,
                Token::RightBracket,
                Token::LeftBrace,
                Token::RightBrace
            ]
        ),
        (
            "let x = 5 + 3.14 - 2 * 10 / 4 % 2",
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(5.0),
                Token::Plus,
                Token::Number(3.14),
                Token::Minus,
                Token::Number(2.0),
                Token::Asterisk,
                Token::Number(10.0),
                Token::Slash,
                Token::Number(4.0),
                Token::Percent,
                Token::Number(2.0)
            ]
        ),
        (
            "let str = \"Hello, World!\"",
            vec![
                Token::Let,
                Token::Identifier("str".to_string()),
                Token::Assign,
                Token::String("Hello, World!".to_string())
            ]
        ),
        (
            "let arr = [1, 2, 3]",
            vec![
                Token::Let,
                Token::Identifier("arr".to_string()),
                Token::Assign,
                Token::LeftBracket,
                Token::Number(1.0),
                Token::Comma,
                Token::Number(2.0),
                Token::Comma,
                Token::Number(3.0),
                Token::RightBracket
            ]
        ),
    ];

    for (source, expected_tokens) in test_cases {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();

        loop {
            let token = lexer.next_token();
            if token == Token::EOF {
                break;
            }
            tokens.push(token);
        }

        assert_eq!(tokens, expected_tokens);
    }
}
