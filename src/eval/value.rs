use crate::ast::Expr;
use crate::eval::env::Env;

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinFuncArgs {
    pub length: usize,
    pub curried: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Func {
        params: Vec<String>,
        body: Box<Expr>,
        env: Env,
    },
    BuiltinFunc {
        name: String,
        func: fn(Vec<Value>) -> Result<Value, String>,
        args: BuiltinFuncArgs,
    },
    ComposedFuncs(Vec<Value>),
}
