use dolang::ast::{Case, CompOp, Expr, FactorOp, LogicOp, Pattern, Stmt, TermOp, UnaryOp, AST};
use dolang::parser::Parser;
use dolang::token::Token;

#[test]
fn test_parser() {
    let test_cases = vec![
        (
            "1",
            vec![Token::Number(1.0)],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Number(1.0))],
            }),
        ),
        (
            "let x = 1",
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
            "1 == 2",
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
            "1 + 2",
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
            "let x = 1 + 2",
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
            "2 * 3",
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
            "-1",
            vec![Token::Minus, Token::Number(1.0)],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Unary {
                    op: UnaryOp::Minus,
                    right: Box::new(Expr::Number(1.0)),
                })],
            }),
        ),
        (
            "true",
            vec![Token::True],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Boolean(true))],
            }),
        ),
        (
            "1 + 1 and 2 + 2",
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
        (
            "[1, 2, 3]",
            vec![
                Token::LeftBracket,
                Token::Number(1.0),
                Token::Comma,
                Token::Number(2.0),
                Token::Comma,
                Token::Number(3.0),
                Token::RightBracket,
            ],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::List(vec![
                    Expr::Number(1.0),
                    Expr::Number(2.0),
                    Expr::Number(3.0),
                ]))],
            }),
        ),
        (
            "if 1 then 2 else 3",
            vec![
                Token::If,
                Token::Number(1.0),
                Token::Then,
                Token::Number(2.0),
                Token::Else,
                Token::Number(3.0),
            ],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::If {
                    cond: Box::new(Expr::Number(1.0)),
                    then: Box::new(Expr::Number(2.0)),
                    else_: Box::new(Expr::Number(3.0)),
                })],
            }),
        ),
        (
            "
            let x = [1, 2, 3]
            x[1]
            ",
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::LeftBracket,
                Token::Number(1.0),
                Token::Comma,
                Token::Number(2.0),
                Token::Comma,
                Token::Number(3.0),
                Token::RightBracket,
                Token::Identifier("x".to_string()),
                Token::LeftBracket,
                Token::Number(1.0),
                Token::RightBracket,
            ],
            Ok(AST {
                stmts: vec![
                    Stmt::Let {
                        name: "x".to_string(),
                        val: Expr::List(vec![
                            Expr::Number(1.0),
                            Expr::Number(2.0),
                            Expr::Number(3.0),
                        ]),
                    },
                    Stmt::Expr(Expr::ListAccess {
                        list: Box::new(Expr::Identifier("x".to_string())),
                        index: 1.0,
                    }),
                ],
            }),
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
            Ok(AST {
                stmts: vec![Stmt::Let {
                    name: "pi".to_string(),
                    val: Expr::Func {
                        params: vec![],
                        body: Box::new(Expr::Number(3.14)),
                    },
                }],
            }),
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
            "
            let x = 1
            let r = match x
                | 1 -> 2
                | 2 -> 3
            ",
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(1.0),
                Token::Let,
                Token::Identifier("r".to_string()),
                Token::Assign,
                Token::Match,
                Token::Identifier("x".to_string()),
                Token::Pipe,
                Token::Number(1.0),
                Token::Arrow,
                Token::Number(2.0),
                Token::Pipe,
                Token::Number(2.0),
                Token::Arrow,
                Token::Number(3.0),
            ],
            Ok(AST {
                stmts: vec![
                    Stmt::Let {
                        name: "x".to_string(),
                        val: Expr::Number(1.0),
                    },
                    Stmt::Let {
                        name: "r".to_string(),
                        val: Expr::Match {
                            cond: Box::new(Expr::Identifier("x".to_string())),
                            cases: vec![
                                Case {
                                    pattern: Pattern::Number(1.0),
                                    body: Expr::Number(2.0),
                                },
                                Case {
                                    pattern: Pattern::Number(2.0),
                                    body: Expr::Number(3.0),
                                },
                            ],
                        },
                    },
                ],
            }),
        ),
        (
            "print(\"Hello, World!\")",
            vec![
                Token::Identifier("print".to_string()),
                Token::LeftParen,
                Token::String("Hello, World!".to_string()),
                Token::RightParen,
            ],
            Ok(AST {
                stmts: vec![Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Identifier("print".to_string())),
                    args: vec![Expr::String("Hello, World!".to_string())],
                })],
            }),
        ),
    ];

    for (_, tokens, expected) in test_cases {
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert_eq!(result, expected);
    }
}
