use dolang::lexer::Lexer;
use dolang::token::{Position, Range, Token, TokenType};

#[test]
fn test_lexer() {
    let test_cases = vec![
        (
            "let msg = \"Hello, World!\"
println(msg)",
            vec![
                Token {
                    token_type: TokenType::Let,
                    range: Range {
                        start: Position { line: 1, column: 1 },
                        end: Position { line: 1, column: 3 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("msg".to_string()),
                    range: Range {
                        start: Position { line: 1, column: 5 },
                        end: Position { line: 1, column: 7 },
                    },
                },
                Token {
                    token_type: TokenType::Assign,
                    range: Range {
                        start: Position { line: 1, column: 9 },
                        end: Position { line: 1, column: 9 },
                    },
                },
                Token {
                    token_type: TokenType::String("Hello, World!".to_string()),
                    range: Range {
                        start: Position { line: 1, column: 11 },
                        end: Position { line: 1, column: 25 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("println".to_string()),
                    range: Range {
                        start: Position { line: 2, column: 1 },
                        end: Position { line: 2, column: 7 },
                    },
                },
                Token {
                    token_type: TokenType::LeftParen,
                    range: Range {
                        start: Position { line: 2, column: 8 },
                        end: Position { line: 2, column: 8 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("msg".to_string()),
                    range: Range {
                        start: Position { line: 2, column: 9 },
                        end: Position { line: 2, column: 11 },
                    },
                },
                Token {
                    token_type: TokenType::RightParen,
                    range: Range {
                        start: Position { line: 2, column: 12 },
                        end: Position { line: 2, column: 12 },
                    },
                },
            ]
        ), (
            "let nums = [1, 2, 3, 4, 5]
let odds = nums |> filter(fn n -> n % 2 != 0)",
            vec![
                Token {
                    token_type: TokenType::Let,
                    range: Range {
                        start: Position { line: 1, column: 1 },
                        end: Position { line: 1, column: 3 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("nums".to_string()),
                    range: Range {
                        start: Position { line: 1, column: 5 },
                        end: Position { line: 1, column: 8 },
                    },
                },
                Token {
                    token_type: TokenType::Assign,
                    range: Range {
                        start: Position { line: 1, column: 10 },
                        end: Position { line: 1, column: 10 },
                    },
                },
                Token {
                    token_type: TokenType::LeftBracket,
                    range: Range {
                        start: Position { line: 1, column: 12 },
                        end: Position { line: 1, column: 12 },
                    },
                },
                Token {
                    token_type: TokenType::Number(1.0),
                    range: Range {
                        start: Position { line: 1, column: 13 },
                        end: Position { line: 1, column: 13 },
                    },
                },
                Token {
                    token_type: TokenType::Comma,
                    range: Range {
                        start: Position { line: 1, column: 14 },
                        end: Position { line: 1, column: 14 },
                    },
                },
                Token {
                    token_type: TokenType::Number(2.0),
                    range: Range {
                        start: Position { line: 1, column: 16 },
                        end: Position { line: 1, column: 16 },
                    },
                },
                Token {
                    token_type: TokenType::Comma,
                    range: Range {
                        start: Position { line: 1, column: 17 },
                        end: Position { line: 1, column: 17 },
                    },
                },
                Token {
                    token_type: TokenType::Number(3.0),
                    range: Range {
                        start: Position { line: 1, column: 19 },
                        end: Position { line: 1, column: 19 },
                    },
                },
                Token {
                    token_type: TokenType::Comma,
                    range: Range {
                        start: Position { line: 1, column: 20 },
                        end: Position { line: 1, column: 20 },
                    },
                },
                Token {
                    token_type: TokenType::Number(4.0),
                    range: Range {
                        start: Position { line: 1, column: 22 },
                        end: Position { line: 1, column: 22 },
                    },
                },
                Token {
                    token_type: TokenType::Comma,
                    range: Range {
                        start: Position { line: 1, column: 23 },
                        end: Position { line: 1, column: 23 },
                    },
                },
                Token {
                    token_type: TokenType::Number(5.0),
                    range: Range {
                        start: Position { line: 1, column: 25 },
                        end: Position { line: 1, column: 25 },
                    },
                },
                Token {
                    token_type: TokenType::RightBracket,
                    range: Range {
                        start: Position { line: 1, column: 26 },
                        end: Position { line: 1, column: 26 },
                    },
                },
                Token {
                    token_type: TokenType::Let,
                    range: Range {
                        start: Position { line: 2, column: 1 },
                        end: Position { line: 2, column: 3 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("odds".to_string()),
                    range: Range {
                        start: Position { line: 2, column: 5 },
                        end: Position { line: 2, column: 8 },
                    },
                },
                Token {
                    token_type: TokenType::Assign,
                    range: Range {
                        start: Position { line: 2, column: 10 },
                        end: Position { line: 2, column: 10 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("nums".to_string()),
                    range: Range {
                        start: Position { line: 2, column: 12 },
                        end: Position { line: 2, column: 15 },
                    },
                },
                Token {
                    token_type: TokenType::ForwardPipe,
                    range: Range {
                        start: Position { line: 2, column: 17 },
                        end: Position { line: 2, column: 18 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("filter".to_string()),
                    range: Range {
                        start: Position { line: 2, column: 20 },
                        end: Position { line: 2, column: 25 },
                    },
                },
                Token {
                    token_type: TokenType::LeftParen,
                    range: Range {
                        start: Position { line: 2, column: 26 },
                        end: Position { line: 2, column: 26 },
                    },
                },
                Token {
                    token_type: TokenType::Fn,
                    range: Range {
                        start: Position { line: 2, column: 27 },
                        end: Position { line: 2, column: 28 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("n".to_string()),
                    range: Range {
                        start: Position { line: 2, column: 30 },
                        end: Position { line: 2, column: 30 },
                    },
                },
                Token {
                    token_type: TokenType::Arrow,
                    range: Range {
                        start: Position { line: 2, column: 32 },
                        end: Position { line: 2, column: 33 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("n".to_string()),
                    range: Range {
                        start: Position { line: 2, column: 35 },
                        end: Position { line: 2, column: 35 },
                    },
                },
                Token {
                    token_type: TokenType::Percent,
                    range: Range {
                        start: Position { line: 2, column: 37 },
                        end: Position { line: 2, column: 37 },
                    },
                },
                Token {
                    token_type: TokenType::Number(2.0),
                    range: Range {
                        start: Position { line: 2, column: 39 },
                        end: Position { line: 2, column: 39 },
                    },
                },
                Token {
                    token_type: TokenType::NotEqual,
                    range: Range {
                        start: Position { line: 2, column: 41 },
                        end: Position { line: 2, column: 42 },
                    },
                },
                Token {
                    token_type: TokenType::Number(0.0),
                    range: Range {
                        start: Position { line: 2, column: 44 },
                        end: Position { line: 2, column: 44 },
                    },
                },
                Token {
                    token_type: TokenType::RightParen,
                    range: Range {
                        start: Position { line: 2, column: 45 },
                        end: Position { line: 2, column: 45 },
                    },
                },
            ]
        )
    ];

    for (source, expected_tokens) in test_cases {
        let mut lexer = Lexer::new(source);

        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            if token.token_type == TokenType::EOF {
                break;
            }
            tokens.push(token);
        }

        assert_eq!(
            tokens.len(),
            expected_tokens.len(),
            "Token count mismatch for source: {}",
            source
        );

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token, &expected_tokens[i]);
        }
    }
}
