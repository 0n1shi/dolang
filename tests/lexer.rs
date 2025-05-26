use dolang::lexer::Lexer;
use dolang::token::{Position, Range, Token, TokenType};

#[test]
fn test_lexer() {
    let test_cases = vec![
        (
            "let if then else for fn in match | _ |> -> return = == != < <= > >= and or not true false , . .. : ( ) [ ] { }",
            vec![
                Token {
                    token_type: TokenType::Let,
                    range: Range {
                        start: Position { line: 1, column: 1 },
                        end: Position { line: 1, column: 3 },
                    },
                },
                Token {
                    token_type: TokenType::If,
                    range: Range {
                        start: Position { line: 1, column: 5 },
                        end: Position { line: 1, column: 6 },
                    },
                },
                Token {
                    token_type: TokenType::Then,
                    range: Range {
                        start: Position { line: 1,  column: 8 },
                        end: Position { line: 1,  column: 11 },
                    },
                },
                Token {
                    token_type: TokenType::Else,
                    range: Range {
                        start: Position { line: 1, column: 13 },
                        end: Position { line: 1, column: 16 },
                    },
                },
                Token {
                    token_type: TokenType::For,
                    range: Range {
                        start: Position { line: 1, column: 18 },
                        end: Position { line: 1, column: 20 },
                    },
                },
                Token {
                    token_type: TokenType::Fn,
                    range: Range {
                        start: Position { line: 1, column: 22 },
                        end: Position { line: 1, column: 23 },
                    },
                },
                Token {
                    token_type: TokenType::In,
                    range: Range {
                        start: Position { line: 1, column: 25 },
                        end: Position { line: 1, column: 26 },
                    },
                },
                Token {
                    token_type: TokenType::Match,
                    range: Range {
                        start: Position { line: 1, column: 28 },
                        end: Position { line: 1, column: 32 },
                    },
                },
                Token {
                    token_type: TokenType::Pipe,
                    range: Range {
                        start: Position { line: 1, column: 34 },
                        end: Position { line: 1, column: 34 },
                    },
                },
                Token {
                    token_type: TokenType::Underscore,
                    range: Range {
                        start: Position { line: 1, column: 36 },
                        end: Position { line: 1, column: 36 },
                    },
                },
                Token {
                    token_type: TokenType::ForwardPipe,
                    range: Range {
                        start: Position { line: 1, column: 38 },
                        end: Position { line: 1, column: 39 },
                    },
                },
                Token {
                    token_type: TokenType::Arrow,
                    range: Range {
                        start: Position { line: 1, column: 41 },
                        end: Position { line: 1, column: 42 },
                    },
                },
                Token {
                    token_type: TokenType::Return,
                    range: Range {
                        start: Position { line: 1, column: 44 },
                        end: Position { line: 1, column: 49 },
                    },
                },
                Token {
                    token_type: TokenType::Assign,
                    range: Range {
                        start: Position { line: 1, column: 51 },
                        end: Position { line: 1, column: 51 },
                    },
                },
                Token {
                    token_type: TokenType::Equal,
                    range: Range {
                        start: Position { line: 1, column: 53 },
                        end: Position { line: 1, column: 54 },
                    },
                },
                Token {
                    token_type: TokenType::NotEqual,
                    range: Range {
                        start: Position { line: 1, column: 56 },
                        end: Position { line: 1, column: 57 },
                    },
                },
                Token {
                    token_type: TokenType::LessThan,
                    range: Range {
                        start: Position { line: 1, column: 59 },
                        end: Position { line: 1, column: 59 },
                    },
                },
                Token {
                    token_type: TokenType::LessThanOrEqual,
                    range: Range {
                        start: Position { line: 1, column: 61 },
                        end: Position { line: 1, column: 62 },
                    },
                },
                Token {
                    token_type: TokenType::GreaterThan,
                    range: Range {
                        start: Position { line: 1, column: 64 },
                        end: Position { line: 1, column: 64 },
                    },
                },
                Token {
                    token_type: TokenType::GreaterThanOrEqual,
                    range: Range {
                        start: Position { line: 1, column: 66 },
                        end: Position { line: 1, column: 67 },
                    },
                },
                Token {
                    token_type: TokenType::And,
                    range: Range {
                        start: Position { line: 1, column: 69 },
                        end: Position { line: 1, column: 71 },
                    },
                },
                Token {
                    token_type: TokenType::Or,
                    range: Range {
                        start: Position { line: 1, column: 73 },
                        end: Position { line: 1, column: 74 },
                    },
                },
                Token {
                    token_type: TokenType::Not,
                    range: Range {
                        start: Position { line: 1, column: 76 },
                        end: Position { line: 1, column: 78 },
                    },
                },
                Token {
                    token_type: TokenType::True,
                    range: Range {
                        start: Position { line: 1, column: 80 },
                        end: Position { line: 1, column: 83 },
                    },
                },
                Token {
                    token_type: TokenType::False,
                    range: Range {
                        start: Position { line: 1, column: 85 },
                        end: Position { line: 1, column: 89 },
                    },
                },
                Token {
                    token_type: TokenType::Comma,
                    range: Range {
                        start: Position { line: 1, column: 91 },
                        end: Position { line: 1, column: 91 },
                    },
                },
                Token {
                    token_type: TokenType::Dot,
                    range: Range {
                        start: Position { line: 1, column: 93 },
                        end: Position { line: 1, column: 93 },
                    },
                },
                Token {
                    token_type: TokenType::DotDot,
                    range: Range {
                        start: Position { line: 1, column: 95 },
                        end: Position { line: 1, column: 96 },
                    },
                },
                Token {
                    token_type: TokenType::Colon,
                    range: Range {
                        start: Position { line: 1, column: 98 },
                        end: Position { line: 1, column: 98 },
                    },
                },
                Token {
                    token_type: TokenType::LeftParen,
                    range: Range {
                        start: Position { line: 1, column: 100 },
                        end: Position { line: 1, column: 100 },
                    },
                },
                Token {
                    token_type: TokenType::RightParen,
                    range: Range {
                        start: Position { line: 1, column: 102 },
                        end: Position { line: 1, column: 102 },
                    },
                },
                Token {
                    token_type: TokenType::LeftBracket,
                    range: Range {
                        start: Position { line: 1, column: 104 },
                        end: Position { line: 1, column: 104 },
                    },
                },
                Token {
                    token_type: TokenType::RightBracket,
                    range: Range {
                        start: Position { line: 1, column: 106 },
                        end: Position { line: 1, column: 106 },
                    },
                },
                Token {
                    token_type: TokenType::LeftBrace,
                    range: Range {
                        start: Position { line: 1, column: 108 },
                        end: Position { line: 1, column: 108 },
                    },
                },
                Token {
                    token_type: TokenType::RightBrace,
                    range: Range {
                        start: Position { line: 1, column: 110 },
                        end: Position { line: 1, column: 110 },
                    },
                },
                Token {
                    token_type: TokenType::EOF,
                    range: Range {
                        start: Position { line: 1, column: 111 },
                        end: Position { line: 1, column: 110 },
                    },
                },
            ]
        ), (
            r#"let msg = "Hello world"
            println(msg)
            "#,
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
                    token_type: TokenType::String("Hello world".to_string()),
                    range: Range {
                        start: Position { line: 1, column: 11 },
                        end: Position { line: 1, column: 23 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("println".to_string()),
                    range: Range {
                        start: Position { line: 2, column: 13 },
                        end: Position { line: 2, column: 19 },
                    },
                },
                Token {
                    token_type: TokenType::LeftParen,
                    range: Range {
                        start: Position { line: 2, column: 21 },
                        end: Position { line: 2, column: 21 },
                    },
                },
                Token {
                    token_type: TokenType::Identifier("msg".to_string()),
                    range: Range {
                        start: Position { line: 2, column: 22 },
                        end: Position { line: 2, column: 24 },
                    },
                },
                Token {
                    token_type: TokenType::RightParen,
                    range: Range {
                        start: Position { line: 2, column: 25 },
                        end: Position { line: 2, column: 25 },
                    },
                },
                Token {
                    token_type: TokenType::EOF,
                    range: Range {
                        start: Position { line: 2, column: 26 },
                        end: Position { line: 2, column: 26 },
                    },
                },
            ]
        )
    ];

    for (source, expected_tokens) in test_cases {
        let mut lexer = Lexer::new(source);

        let mut count = 0;
        for expected_token in &expected_tokens {
            assert_eq!(lexer.next_token(), *expected_token);
            count += 1;
        }
        assert_eq!(count, expected_tokens.len());
    }
}
