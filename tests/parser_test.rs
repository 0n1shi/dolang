use dolang::ast::{Expr, Stmt, AST};
use dolang::parser::Parser;
use dolang::token::Token;

#[test]
fn test_parser() {
    let test_cases = vec![
        (
            vec![Token::Number(1.0)],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Number(1.0))],
            }),
        ),
        (
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(1.0),
            ],
            Ok(AST {
                stmts: vec![Stmt::Let {
                    name: "x".to_string(),
                    val: Expr::Number(1.0),
                }],
            }),
        ),
    ];

    for (tokens, expected) in test_cases {
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result, expected);
    }
}
