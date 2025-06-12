use dolang::ast::{Case, CompOp, Expr, FactorOp, LogicOp, Pattern, Stmt, TermOp, UnaryOp, AST};
use dolang::lexer::Lexer;
use dolang::parser::Parser;
use dolang::token::TokenType;

#[test]
fn test_parser() {
    let test_cases = vec![
        (
            "1",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Number(1.0))],
            }),
        ),
        (
            "not true",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Unary {
                    op: UnaryOp::Not,
                    right: Box::new(Expr::Boolean(true)),
                })],
            }),
        ),
        (
            "1 + 2 * 3",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Term {
                    left: Box::new(Expr::Number(1.0)),
                    op: TermOp::Plus,
                    right: Box::new(Expr::Factor {
                        left: Box::new(Expr::Number(2.0)),
                        op: FactorOp::Multiply,
                        right: Box::new(Expr::Number(3.0)),
                    }),
                })],
            }),
        ),
        (
            "1 + 2",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Term {
                    left: Box::new(Expr::Number(1.0)),
                    op: TermOp::Plus,
                    right: Box::new(Expr::Number(2.0)),
                })],
            }),
        ),
        (
            "if true then 1 else 2",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::If {
                    cond: Box::new(Expr::Boolean(true)),
                    then: Box::new(Expr::Number(1.0)),
                    else_: Box::new(Expr::Number(2.0)),
                })],
            }),
        ),
        (
            "match x 
    | 1 -> \"one\"
    | 2 -> \"two\"",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Match {
                    cond: Box::new(Expr::Identifier("x".to_string())),
                    cases: vec![
                        Case {
                            pattern: Pattern::Number(1.0),
                            body: Expr::String("one".to_string()),
                        },
                        Case {
                            pattern: Pattern::Number(2.0),
                            body: Expr::String("two".to_string()),
                        },
                    ],
                })],
            }),
        ),
        (
            "let x = 10",
            Ok(AST {
                stmts: vec![Stmt::Let {
                    name: "x".to_string(),
                    val: Expr::Number(10.0),
                }],
            }),
        ),
        (
            "let add = fn x, y -> x + y",
            Ok(AST {
                stmts: vec![Stmt::Let {
                    name: "add".to_string(),
                    val: Expr::Func {
                        params: vec!["x".to_string(), "y".to_string()],
                        body: Box::new(Expr::Term {
                            left: Box::new(Expr::Identifier("x".to_string())),
                            op: TermOp::Plus,
                            right: Box::new(Expr::Identifier("y".to_string())),
                        }),
                    },
                }],
            }),
        ),
        (
            "[1, 2, 3] |> filter(fn x -> x > 1)",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Pipe {
                    left: Box::new(Expr::List(vec![
                        Expr::Number(1.0),
                        Expr::Number(2.0),
                        Expr::Number(3.0),
                    ])),
                    right: Box::new(Expr::Call {
                        name: Box::new(Expr::Identifier("filter".to_string())),
                        args: vec![Expr::Func {
                            params: vec!["x".to_string()],
                            body: Box::new(Expr::Comp {
                                left: Box::new(Expr::Identifier("x".to_string())),
                                op: CompOp::GreaterThan,
                                right: Box::new(Expr::Number(1.0)),
                            }),
                        }],
                    }),
                })],
            }),
        ),
        (
            "true and false",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Logic {
                    left: Box::new(Expr::Boolean(true)),
                    op: LogicOp::And,
                    right: Box::new(Expr::Boolean(false)),
                })],
            }),
        ),
        (
            "1 < 2 or 3 > 4",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Logic {
                    left: Box::new(Expr::Comp {
                        left: Box::new(Expr::Number(1.0)),
                        op: CompOp::LessThan,
                        right: Box::new(Expr::Number(2.0)),
                    }),
                    op: LogicOp::Or,
                    right: Box::new(Expr::Comp {
                        left: Box::new(Expr::Number(3.0)),
                        op: CompOp::GreaterThan,
                        right: Box::new(Expr::Number(4.0)),
                    }),
                })],
            }),
        ),
        (
            "mysql.connect(\"localhost\", \"user\", \"password\")",
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Call {
                    name: Box::new(Expr::Access {
                        record: Box::new(Expr::Identifier("mysql".to_string())),
                        field: "connect".to_string(),
                    }),
                    args: vec![
                        Expr::String("localhost".to_string()),
                        Expr::String("user".to_string()),
                        Expr::String("password".to_string()),
                    ],
                })],
            }),
        ),
    ];

    for (input, expected) in test_cases {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            if token.token_type == TokenType::EOF {
                break;
            }
            tokens.push(token);
        }

        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result, expected, "Failed to parse input: {}", input);
    }
}
