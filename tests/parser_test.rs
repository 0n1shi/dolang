use dolang::ast::{CompOp, Expr, FactorOp, LogicOp, Stmt, TermOp, UnaryOp, AST};
use dolang::parser::Parser;
use dolang::token::Token;

#[test]
fn test_parser() {
    let test_cases = vec![
        (
            // 1
            vec![Token::Number(1.0)],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Number(1.0))],
            }),
        ),
        (
            // let x = 1
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
        (
            // 1 == 2,
            vec![Token::Number(1.0), Token::Equal, Token::Number(2.0)],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Comp {
                    left: Box::new(Expr::Number(1.0)),
                    op: CompOp::Equal,
                    right: Box::new(Expr::Number(2.0)),
                })],
            }),
        ),
        (
            // 1 + 2
            vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Term {
                    left: Box::new(Expr::Number(1.0)),
                    op: TermOp::Plus,
                    right: Box::new(Expr::Number(2.0)),
                })],
            }),
        ),
        (
            // let x = 1 + 2
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(1.0),
                Token::Plus,
                Token::Number(2.0),
            ],
            Ok(AST {
                stmts: vec![Stmt::Let {
                    name: "x".to_string(),
                    val: Expr::Term {
                        left: Box::new(Expr::Number(1.0)),
                        op: TermOp::Plus,
                        right: Box::new(Expr::Number(2.0)),
                    },
                }],
            }),
        ),
        (
            // 2 * 3
            vec![Token::Number(2.0), Token::Asterisk, Token::Number(3.0)],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Factor {
                    left: Box::new(Expr::Number(2.0)),
                    op: FactorOp::Multiply,
                    right: Box::new(Expr::Number(3.0)),
                })],
            }),
        ),
        (
            // -1
            vec![Token::Minus, Token::Number(1.0)],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Unary {
                    op: UnaryOp::Minus,
                    right: Box::new(Expr::Number(1.0)),
                })],
            }),
        ),
        (
            // true
            vec![Token::True],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Boolean(true))],
            }),
        ),
        (
            // 1 + 1 && 2 + 2
            vec![
                Token::Number(1.0),
                Token::Plus,
                Token::Number(1.0),
                Token::And,
                Token::Number(2.0),
                Token::Plus,
                Token::Number(2.0),
            ],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Logic {
                    left: Box::new(Expr::Term {
                        left: Box::new(Expr::Number(1.0)),
                        op: TermOp::Plus,
                        right: Box::new(Expr::Number(1.0)),
                    }),
                    op: LogicOp::And,
                    right: Box::new(Expr::Term {
                        left: Box::new(Expr::Number(2.0)),
                        op: TermOp::Plus,
                        right: Box::new(Expr::Number(2.0)),
                    }),
                })],
            }),
        ),
    ];

    for (tokens, expected) in test_cases {
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result, expected);
    }
}
