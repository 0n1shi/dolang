use crate::ast::{CompOp, Expr, FactorOp, LogicOp, Pattern, Stmt, TermOp, UnaryOp, AST};
use crate::eval::env::Env;
use crate::eval::value::{BuiltinFuncArgs, Value};

use super::eval_func::{eval_builtin_func, eval_func};

pub fn eval(ast: AST, env: &mut Env) -> Result<(), String> {
    for stmt in &ast.stmts {
        eval_stmt(&stmt, env)?;
    }
    Ok(())
}

pub fn eval_stmt(stmt: &Stmt, env: &mut Env) -> Result<(), String> {
    match stmt {
        Stmt::Expr(expr) => {
            eval_expr(expr, env)?;
            Ok(())
        }
        Stmt::Let { name, val } => {
            let val = eval_expr(val, env)?;
            env.set(name.clone(), val);
            Ok(())
        }
        _ => {
            // Handle other statement types (e.g., function definitions, etc.)
            Err("Unsupported statement type".into())
        }
    }
}

pub fn eval_expr(expr: &Expr, env: &mut Env) -> Result<Value, String> {
    match expr {
        Expr::Func { params, body } => Ok(Value::Func {
            params: params.clone(),
            body: body.clone(),
            env: env.clone(),
        }),
        Expr::If { cond, then, else_ } => {
            let cond_val = eval_expr(cond, env)?;
            match cond_val {
                Value::Boolean(true) => eval_expr(then, env),
                Value::Boolean(false) => eval_expr(else_, env),
                _ => Err("Condition must be a boolean".into()),
            }
        }
        Expr::Match { cond, cases } => {
            let cond_val = eval_expr(cond, env)?;
            for case in cases {
                match &case.pattern {
                    Pattern::Number(n) => {
                        if let Value::Number(val) = cond_val {
                            if val == *n {
                                return eval_expr(&case.body, env);
                            }
                        }
                    }
                    Pattern::String(s) => {
                        if let Value::String(ref val) = cond_val {
                            if *val == *s {
                                return eval_expr(&case.body, env);
                            }
                        }
                    }
                    Pattern::Boolean(b) => {
                        if let Value::Boolean(val) = cond_val {
                            if val == *b {
                                return eval_expr(&case.body, env);
                            }
                        }
                    }
                    Pattern::Wildcard => {
                        return eval_expr(&case.body, env);
                    }
                }
            }
            Err("No matching case found".into())
        }
        Expr::List(items) => {
            let mut values = Vec::new();
            for item in items {
                values.push(eval_expr(item, env)?);
            }
            Ok(Value::List(values))
        }
        Expr::Pipe { left, right } => {
            let left_val = eval_expr(left, env)?;
            let right_val = eval_expr(right, env)?;
            match right_val {
                Value::Func { params, body, env } => {
                    let mut new_env = Env::new(Some(Box::new(env.clone())));
                    new_env.set(params[0].clone(), left_val);
                    eval_expr(&body, &mut new_env)
                }
                Value::BuiltinFunc {
                    name: _,
                    func,
                    args,
                } => {
                    let mut arg_vals = Vec::new();
                    for arg in args.curried.iter() {
                        arg_vals.push(arg.clone());
                    }
                    arg_vals.push(left_val);
                    func(arg_vals)
                }
                _ => Err("Pipe requires a function on the right".into()),
            }
        }
        Expr::Logic { left, op, right } => {
            let left_val = eval_expr(left, env)?;
            let right_val = eval_expr(right, env)?;
            match op {
                LogicOp::And => match (left_val, right_val) {
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l && r)),
                    _ => Err("Logical AND requires boolean operands".into()),
                },
                LogicOp::Or => match (left_val, right_val) {
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l || r)),
                    _ => Err("Logical OR requires boolean operands".into()),
                },
            }
        }
        Expr::Comp { left, op, right } => {
            let left_val = eval_expr(left, env)?;
            let right_val = eval_expr(right, env)?;
            match op {
                CompOp::Equal => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l == r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l == r)),
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
                    _ => Err("Equality comparison requires same type".into()),
                },
                CompOp::NotEqual => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l != r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l != r)),
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
                    _ => Err("Inequality comparison requires same type".into()),
                },
                CompOp::LessThan => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                    _ => Err("Less than comparison requires number operands".into()),
                },
                CompOp::LessThanOrEqual => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                    _ => Err("Less than or equal comparison requires number operands".into()),
                },
                CompOp::GreaterThan => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                    _ => Err("Greater than comparison requires number operands".into()),
                },
                CompOp::GreaterThanOrEqual => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                    _ => Err("Greater than or equal comparison requires number operands".into()),
                },
            }
        }
        Expr::Range { start, end } => {
            let start_val = eval_expr(start, env)?;
            let end_val = eval_expr(end, env)?;
            match (start_val, end_val) {
                (Value::Number(s), Value::Number(e)) => {
                    if s <= e {
                        let mut range = Vec::new();
                        for i in s as usize..=e as usize {
                            range.push(Value::Number(i as f64));
                        }
                        Ok(Value::List(range))
                    } else {
                        Err("Start of range must be less than or equal to end".into())
                    }
                }
                _ => Err("Range requires number operands".into()),
            }
        }
        Expr::Term { left, op, right } => {
            let left_val = eval_expr(left, env)?;
            let right_val = eval_expr(right, env)?;
            match op {
                TermOp::Plus => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                    _ => Err("Addition requires number or string operands".into()),
                },
                TermOp::Minus => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                    _ => Err("Subtraction requires number operands".into()),
                },
            }
        }
        Expr::Factor { left, op, right } => {
            let left_val = eval_expr(left, env)?;
            let right_val = eval_expr(right, env)?;
            match op {
                FactorOp::Multiply => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                    _ => Err("Multiplication requires number operands".into()),
                },
                FactorOp::Divide => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        if r == 0.0 {
                            Err("Division by zero".into())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    _ => Err("Division requires number operands".into()),
                },
                FactorOp::Modulus => match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l % r)),
                    _ => Err("Modulus requires number operands".into()),
                },
            }
        }
        Expr::Unary { op, right } => {
            let right_val = eval_expr(right, env)?;
            match op {
                UnaryOp::Minus => match right_val {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    _ => Err("Unary minus requires number operand".into()),
                },
                UnaryOp::Not => match right_val {
                    Value::Boolean(b) => Ok(Value::Boolean(!b)),
                    _ => Err("Logical NOT requires boolean operand".into()),
                },
            }
        }
        Expr::Identifier(expr) => {
            if let Some(value) = env.get(&expr.to_string()) {
                Ok(match value {
                    Value::Number(n) => Value::Number(*n),
                    Value::String(s) => Value::String(s.clone()),
                    Value::Boolean(b) => Value::Boolean(*b),
                    Value::List(l) => Value::List(l.to_vec()),
                    Value::Func { params, body, env } => Value::Func {
                        params: params.clone(),
                        body: body.clone(),
                        env: env.clone(),
                    },
                    Value::BuiltinFunc { name, func, args } => Value::BuiltinFunc {
                        name: name.clone(),
                        func: func.clone(),
                        args: args.clone(),
                    },
                    Value::ComposedFunc { left, right } => Value::ComposedFunc {
                        left: left.clone(),
                        right: right.clone(),
                    },
                })
            } else {
                Err(format!("Undefined variable: {}", expr))
            }
        }
        Expr::Index { list, index } => {
            let list_val = eval_expr(list, env)?;
            let index_val = eval_expr(index, env)?;
            match (list_val, index_val) {
                // Indexing list
                (Value::List(l), Value::Number(i)) => {
                    let idx = i as usize;
                    if idx < l.len() {
                        Ok(l[idx].clone())
                    } else {
                        Err(format!("Index out of bounds: {}", idx))
                    }
                }
                (Value::List(l), Value::List(i)) => {
                    let mut values = Vec::new();
                    let idx = i
                        .iter()
                        .filter_map(|v| match v {
                            Value::Number(n) => Some(*n as usize),
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                    for idx in idx {
                        if idx < l.len() {
                            values.push(l[idx].clone());
                        } else {
                            return Err(format!("Index out of bounds: {}", idx));
                        }
                    }
                    Ok(Value::List(values))
                }
                // Indexing string
                (Value::String(s), Value::Number(i)) => {
                    let idx = i as usize;
                    if idx < s.len() {
                        Ok(Value::String(s[idx..idx + 1].to_string()))
                    } else {
                        Err(format!("Index out of bounds: {}", idx))
                    }
                }
                (Value::String(s), Value::List(i)) => {
                    let start = i[0].clone();
                    let end = i[i.len() - 1].clone();
                    match (start, end) {
                        (Value::Number(start), Value::Number(end)) => {
                            let start = start as usize;
                            let end = end as usize;
                            if start > end || end > s.len() {
                                return Err(format!("Index out of bounds: {}..{}", start, end));
                            }
                            Ok(Value::String(s[start..end].to_string()))
                        }
                        _ => return Err("Indexing requires a number".into()),
                    }
                }
                _ => Err("Indexing requires a list and a number".into()),
            }
        }
        Expr::Slice { list, start, end } => {
            let list_val = eval_expr(list, env)?;
            match list_val {
                Value::List(l) => {
                    let start_val = start.as_ref().map(|s| eval_expr(s, env)).transpose()?;
                    let end_val = end.as_ref().map(|e| eval_expr(e, env)).transpose()?;
                    let start_idx = start_val
                        .and_then(|v| match v {
                            Value::Number(n) => Some(n as usize),
                            _ => None,
                        })
                        .unwrap_or(0);
                    let end_idx = end_val
                        .and_then(|v| match v {
                            Value::Number(n) => Some(n as usize),
                            _ => None,
                        })
                        .unwrap_or(l.len());
                    if start_idx <= end_idx && end_idx <= l.len() {
                        Ok(Value::List(l[start_idx..end_idx].to_vec()))
                    } else {
                        Err(format!(
                            "Slice indices out of bounds: {}..{}",
                            start_idx, end_idx
                        ))
                    }
                }
                _ => Err("Slicing requires a list".into()),
            }
        }
        Expr::Call {
            name: call_name,
            args: call_args,
        } => {
            let func_val = eval_expr(call_name, env)?;
            match func_val {
                Value::Func {
                    params,
                    body,
                    env: func_env,
                } => eval_func(call_name, call_args, params, body, func_env, env),
                Value::BuiltinFunc { name, func, args } => {
                    eval_builtin_func(call_name, call_args, name, func, args, env)
                }
                Value::ComposedFuncs(funcs) => {
                    let mut returned = Option::<Value>::None;
                    for func in funcs {
                        let mut new_call_args = call_args.clone();
                        if let Some(returned) = returned {
                            new_call_args.push(returned);
                        }
                        match func {
                            Value::Func {
                                params,
                                body,
                                env: func_env,
                            } => {
                                eval_func(call_name, &new_call_args, params, body, func_env, env)?;
                            }
                            _ => return Err("Composed function requires a function".into()),
                        }
                    }
                    Ok(Value::ComposedFuncs(values))
                }
                _ => {
                    return Err(format!(
                        "Function call requires a function, but got {:?}",
                        func_val
                    ))
                }
            }
        }
        Expr::Compose(items) => {
            let mut values = Vec::new();
            for item in items {
                values.push(eval_expr(item, env)?);
            }
            Ok(Value::ComposedFunc(values))
        }
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Boolean(b) => Ok(Value::Boolean(*b)),
    }
}
