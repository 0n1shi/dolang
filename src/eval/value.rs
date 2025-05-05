use crate::ast::Expr;
use crate::eval::env::Env;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Func {
        args: Vec<String>,
        body: Box<Expr>,
        env: Env,
    },
    BuiltinFunc {
        name: String,
        func: fn(&[Value]) -> Result<Value, String>,
    },
}
