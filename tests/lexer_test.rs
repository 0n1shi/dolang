use std::fs::File;

fn file_to_tokens(path: &str) -> Vec<Token> {
    let source =
        std::fs::read_to_string(path).expect(format!("Failed to read file: {}", path).as_str());
    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }
    tokens
}

#[test]
fn test_lexer() {
    let test_cases = vec![(
        "examples/let.do",
        vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Number(10),
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Assign,
            Token::Number(20),
            Token::Let,
            Token::Identifier("sum".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
        ],
    )];

    for (test_file, expected_tokens) in test_cases {
        let tokens = file_to_tokens(test_file);
        assert_eq!(tokens, expected_tokens);
    }
}
