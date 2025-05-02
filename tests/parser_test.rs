use dolang::ast::Expression;
use dolang::parser::Parser;
use dolang::token::Token;

#[test]
fn test_parser() {
    let test_cases = vec![
        (vec![Token::Number(1.0)], Some(Expression::Number(1.0))),
        (
            vec![Token::String("hello".to_string())],
            Some(Expression::String("hello".to_string())),
        ),
        (vec![Token::True], Some(Expression::Boolean(true))),
        (vec![Token::False], Some(Expression::Boolean(false))),
        (
            vec![Token::Identifier("y".to_string())],
            Some(Expression::Identifier("y".to_string())),
        ),
    ];

    for (tokens, expected) in test_cases {
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result, expected);
    }
}
