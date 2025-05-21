use dolang::ast::Expr;
use dolang::ast::{CompOp, FactorOp, LogicOp, TermOp, UnaryOp};
use dolang::eval::env::Env;
use dolang::eval::eval::eval_expr;
use dolang::eval::value::Value;

#[test]
fn test_eval_expr() {
    let test_cases = vec![
        (
            // { name: "John", age: 30 }
            Expr::Record(
                vec![
                    ("name".to_string(), Expr::String("John".to_string())),
                    ("age".to_string(), Expr::Number(30.0)),
                ]
                .into_iter()
                .collect(),
            ),
            Ok(Value::Record(
                vec![
                    ("name".to_string(), Value::String("John".to_string())),
                    ("age".to_string(), Value::Number(30.0)),
                ]
                .into_iter()
                .collect(),
            )),
        ),
        (
            // 5 + 5 == 10 and true -> true
            Expr::Logic {
                left: Box::new(Expr::Comp {
                    left: Box::new(Expr::Term {
                        left: Box::new(Expr::Number(5.0)),
                        op: TermOp::Plus,
                        right: Box::new(Expr::Number(5.0)),
                    }),
                    op: CompOp::Equal,
                    right: Box::new(Expr::Number(10.0)),
                }),
                op: LogicOp::And,
                right: Box::new(Expr::Boolean(true)),
            },
            Ok(Value::Boolean(true)),
        ),
        (
            // 5 == 3 -> false
            Expr::Comp {
                left: Box::new(Expr::Number(5.0)),
                op: CompOp::Equal,
                right: Box::new(Expr::Number(3.0)),
            },
            Ok(Value::Boolean(false)),
        ),
        (
            // 5 + 3 -> 8
            Expr::Term {
                left: Box::new(Expr::Number(5.0)),
                op: TermOp::Plus,
                right: Box::new(Expr::Number(3.0)),
            },
            Ok(Value::Number(8.0)),
        ),
        (
            // 5 * 3 -> 15
            Expr::Factor {
                left: Box::new(Expr::Number(5.0)),
                op: FactorOp::Multiply,
                right: Box::new(Expr::Number(3.0)),
            },
            Ok(Value::Number(15.0)),
        ),
        (
            Expr::Unary {
                op: UnaryOp::Minus,
                right: Box::new(Expr::Number(5.0)),
            },
            Ok(Value::Number(-5.0)),
        ), // -5 -> -5
        (Expr::Number(5.0), Ok(Value::Number(5.0))), // 5 -> 5
    ];

    for (input, expected) in test_cases {
        let mut env = Env::new(None);
        let result = eval_expr(&input, &mut env);
        assert_eq!(result, expected, "Failed for input: {:?}", input);
    }
}
