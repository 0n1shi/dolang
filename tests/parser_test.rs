use dolang::ast::Expr;
use dolang::parser::Parser;
use dolang::token::Token;

#[test]
fn test_parser() {
    let test_cases = vec![
        (vec![Token::Number(1.0)], Some(Expr::Number(1.0))),
        (
            vec![Token::String("hello".to_string())],
            Some(Expr::String("hello".to_string())),
        ),
        (vec![Token::True], Some(Expr::Boolean(true))),
        (vec![Token::False], Some(Expr::Boolean(false))),
        (
            vec![Token::Identifier("y".to_string())],
            Some(Expr::Identifier("y".to_string())),
        ),
    ];

    for (tokens, expected) in test_cases {
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result, expected);
    }
}
