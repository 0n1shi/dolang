use dolang::lexer::Lexer;
use dolang::token::Token;

#[test]
fn test_lexer() {
    let test_cases = vec![
        (
            "let x = 5",
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(5.0),
            ],
        ),
        (
            "let pi = fn _ -> 3.14",
            vec![
                Token::Let,
                Token::Identifier("pi".to_string()),
                Token::Assign,
                Token::Fn,
                Token::Underscore,
                Token::Arrow,
                Token::Number(3.14),
            ],
        ),
        (
            "let add = fn x, y -> x + y",
            vec![
                Token::Let,
                Token::Identifier("add".to_string()),
                Token::Assign,
                Token::Fn,
                Token::Identifier("x".to_string()),
                Token::Comma,
                Token::Identifier("y".to_string()),
                Token::Arrow,
                Token::Identifier("x".to_string()),
                Token::Plus,
                Token::Identifier("y".to_string()),
            ],
        ),
        (
            "let r = if x < 5 then 10 else 20",
            vec![
                Token::Let,
                Token::Identifier("r".to_string()),
                Token::Assign,
                Token::If,
                Token::Identifier("x".to_string()),
                Token::LessThan,
                Token::Number(5.0),
                Token::Then,
                Token::Number(10.0),
                Token::Else,
                Token::Number(20.0),
            ],
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
