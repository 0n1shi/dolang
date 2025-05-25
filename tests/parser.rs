use dolang::ast::{Case, CompOp, Expr, FactorOp, LogicOp, Pattern, Stmt, TermOp, UnaryOp, AST};
use dolang::parser::Parser;
use dolang::token::{Position, Range, Token, TokenType};

#[test]
fn test_parser() {
    let test_cases = vec![(
        "1",
        vec![Token {
            token_type: TokenType::Number(1.0),
            range: Range {
                start: Position { line: 1, column: 1 },
                end: Position { line: 1, column: 2 },
            },
        }],
        Ok(AST {
            stmts: vec![Stmt::Expr(Expr::Number(1.0))],
        }),
    )];

    for (_, tokens, expected) in test_cases {
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result, expected);
    }
}
