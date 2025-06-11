use crate::ast::{CompOp, Expr, FactorOp, LogicOp, Pattern, Stmt, TermOp, UnaryOp, AST};
use crate::eval::env::Env;
use crate::eval::value::{BuiltinFuncArgs, Value};

pub struct Evaluator {
    pub current_dir: String,
}

impl Evaluator {
    pub fn new(current_dir: String) -> Self {
        Evaluator { current_dir }
    }
    pub fn eval(&mut self, ast: AST, env: &mut Env) -> Result<(), String> {
        for stmt in &ast.stmts {
            self.eval_stmt(&stmt, env)?;
        }
        Ok(())
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt, env: &mut Env) -> Result<(), String> {
        match stmt {
            Stmt::Expr(expr) => {
                self.eval_expr(expr, env)?;
                Ok(())
            }
            Stmt::Let { name, val } => {
                let val = self.eval_expr(val, env)?;
                env.set(name.clone(), val);
                Ok(())
            }
            Stmt::Import { module } => {
                let module_path = format!("{}/{}.do", self.current_dir, module);
                let text = std::fs::read_to_string(module_path)
                    .map_err(|e| format!("Failed to read module {}: {}", module, e))?;

                let mut lexer = crate::lexer::Lexer::new(&text);
                let mut tokens = Vec::new();
                loop {
                    let token = lexer.next_token();
                    if token.token_type == crate::token::TokenType::EOF {
                        break; // Stop on EOF
                    }
                    tokens.push(token);
                }

                let mut parser = crate::parser::Parser::new(tokens);
                let ast = parser
                    .parse()
                    .map_err(|e| format!("Failed to parse module {}: {}", module, e))?;

                let mut imported_evaluator = Evaluator::new(self.current_dir.clone());
                let mut imported_env = Env::new(Some(Box::new(env.clone())));
                imported_evaluator.eval(ast, &mut imported_env)?;

                // init record
                let mut imported_values = std::collections::HashMap::new();
                for (name, value) in imported_env.get_all() {
                    match value {
                        Value::BuiltinFunc {
                            name: _,
                            func: _,
                            args: _,
                        } => continue, // Skip built-in functions
                        _ => {
                            imported_values.insert(name.clone(), value.clone());
                        }
                    }
                }
                env.set(module.clone(), Value::Record(imported_values.clone()));
                Ok(())
            }
            _ => {
                // Handle other statement types (e.g., function definitions, etc.)
                Err("Unsupported statement type".into())
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr, env: &mut Env) -> Result<Value, String> {
        match expr {
            Expr::Func { params, body } => Ok(Value::Func {
                params: params.clone(),
                body: body.clone(),
                env: env.clone(),
            }),
            Expr::If { cond, then, else_ } => {
                let cond_val = self.eval_expr(cond, env)?;
                match cond_val {
                    Value::Boolean(true) => self.eval_expr(then, env),
                    Value::Boolean(false) => self.eval_expr(else_, env),
                    _ => Err("Condition must be a boolean".into()),
                }
            }
            Expr::Match { cond, cases } => {
                let cond_val = self.eval_expr(cond, env)?;
                for case in cases {
                    match &case.pattern {
                        Pattern::Number(n) => {
                            if let Value::Number(val) = cond_val {
                                if val == *n {
                                    return self.eval_expr(&case.body, env);
                                }
                            }
                        }
                        Pattern::String(s) => {
                            if let Value::String(ref val) = cond_val {
                                if *val == *s {
                                    return self.eval_expr(&case.body, env);
                                }
                            }
                        }
                        Pattern::Boolean(b) => {
                            if let Value::Boolean(val) = cond_val {
                                if val == *b {
                                    return self.eval_expr(&case.body, env);
                                }
                            }
                        }
                        Pattern::Wildcard => {
                            return self.eval_expr(&case.body, env);
                        }
                    }
                }
                Err("No matching case found".into())
            }
            Expr::List(items) => {
                let mut values = Vec::new();
                for item in items {
                    values.push(self.eval_expr(item, env)?);
                }
                Ok(Value::List(values))
            }
            Expr::Record(fields) => {
                let mut record = std::collections::HashMap::new();
                for (key, value) in fields {
                    let val = self.eval_expr(value, env)?;
                    record.insert(key.clone(), val);
                }
                Ok(Value::Record(record))
            }
            Expr::Pipe { left, right } => {
                let left_val = self.eval_expr(left, env)?;
                let right_val = self.eval_expr(right, env)?;
                match right_val {
                    Value::Func { params, body, env } => {
                        let mut new_env = Env::new(Some(Box::new(env.clone())));
                        new_env.set(params[0].clone(), left_val);
                        self.eval_expr(&body, &mut new_env)
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
                let left_val = self.eval_expr(left, env)?;
                let right_val = self.eval_expr(right, env)?;
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
                let left_val = self.eval_expr(left, env)?;
                let right_val = self.eval_expr(right, env)?;
                match op {
                    CompOp::Is => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l == r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l == r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
                        _ => Err("Equality comparison requires same type".into()),
                    },
                    CompOp::IsNot => match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l != r)),
                        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l != r)),
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
                        _ => Err("Inequality comparison requires same type".into()),
                    },
                    CompOp::In => match (left_val, right_val) {
                        (Value::String(s), Value::List(l)) => Ok(Value::Boolean(
                            l.iter().any(|v| v == &Value::String(s.clone())),
                        )),
                        (Value::Number(n), Value::List(l)) => {
                            Ok(Value::Boolean(l.iter().any(|v| v == &Value::Number(n))))
                        }
                        (Value::Boolean(b), Value::List(l)) => {
                            Ok(Value::Boolean(l.iter().any(|v| v == &Value::Boolean(b))))
                        }
                        (Value::String(s), Value::Record(r)) => {
                            Ok(Value::Boolean(r.contains_key(&s)))
                        }
                        (Value::String(s), Value::String(r)) => Ok(Value::Boolean(r.contains(&s))),
                        _ => Err("IN operator requires a list on the right".into()),
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
                        _ => {
                            Err("Greater than or equal comparison requires number operands".into())
                        }
                    },
                }
            }
            Expr::Range { start, end } => {
                let start_val = self.eval_expr(start, env)?;
                let end_val = self.eval_expr(end, env)?;
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
                let left_val = self.eval_expr(left, env)?;
                let right_val = self.eval_expr(right, env)?;
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
                let left_val = self.eval_expr(left, env)?;
                let right_val = self.eval_expr(right, env)?;
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
                let right_val = self.eval_expr(right, env)?;
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
                        Value::Record(r) => Value::Record(r.clone()),
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
                    })
                } else {
                    Err(format!("Undefined variable: {}", expr))
                }
            }
            Expr::Index { list, index } => {
                let list_val = self.eval_expr(list, env)?;
                let index_val = self.eval_expr(index, env)?;
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
            Expr::Access { record, field } => {
                let record_val = self.eval_expr(record, env)?;
                match record_val {
                    Value::Record(r) => {
                        if let Some(value) = r.get(field) {
                            Ok(value.clone())
                        } else {
                            Err(format!("Field not found: {}", field))
                        }
                    }
                    _ => Err("Access requires a record".into()),
                }
            }
            Expr::Slice { list, start, end } => {
                let list_val = self.eval_expr(list, env)?;
                match list_val {
                    Value::List(l) => {
                        let start_val =
                            start.as_ref().map(|s| self.eval_expr(s, env)).transpose()?;
                        let end_val = end.as_ref().map(|e| self.eval_expr(e, env)).transpose()?;
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
                    Value::String(s) => {
                        let start_val =
                            start.as_ref().map(|s| self.eval_expr(s, env)).transpose()?;
                        let end_val = end.as_ref().map(|e| self.eval_expr(e, env)).transpose()?;
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
                            .unwrap_or(s.len());
                        if start_idx <= end_idx && end_idx <= s.len() {
                            Ok(Value::String(s[start_idx..end_idx].to_string()))
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
                let func_val = self.eval_expr(call_name, env)?;
                match func_val {
                    Value::Func {
                        params,
                        body,
                        env: func_env,
                    } => {
                        // normal function call
                        if call_args.len() == params.len() {
                            let mut new_env = Env::new(Some(Box::new(func_env)));
                            for (arg, arg_name) in call_args.iter().zip(params) {
                                let arg_val = self.eval_expr(arg, env)?;
                                new_env.set(arg_name, arg_val);
                            }
                            self.eval_expr(&body, &mut new_env)
                        }
                        // currying
                        else if params.len() > call_args.len() {
                            let mut new_env = Env::new(Some(Box::new(func_env)));
                            for (arg, arg_name) in call_args.iter().zip(params.iter()) {
                                let arg_val = self.eval_expr(arg, env)?;
                                new_env.set(arg_name.clone(), arg_val);
                            }
                            let remaining_params = params[call_args.len()..].to_vec();
                            let remaining_body = body.clone();
                            Ok(Value::Func {
                                params: remaining_params,
                                body: remaining_body,
                                env: new_env,
                            })
                        } else {
                            Err(format!(
                                "Function {:?} requires {} arguments, but got {}",
                                call_name,
                                params.len(),
                                call_args.len()
                            ))
                        }
                    }
                    Value::BuiltinFunc { name, func, args } => {
                        // normal function call
                        if call_args.len() == args.length {
                            let mut arg_vals = Vec::new();
                            for arg in args.curried.iter() {
                                arg_vals.push(arg.clone());
                            }
                            for arg in call_args {
                                arg_vals.push(self.eval_expr(arg, env)?);
                            }
                            func(arg_vals)
                        }
                        // currying
                        else if args.length > call_args.len() {
                            let mut new_args = args.curried.clone();
                            for arg in call_args {
                                let arg_val = self.eval_expr(arg, env)?;
                                new_args.push(arg_val);
                            }
                            Ok(Value::BuiltinFunc {
                                name,
                                func: func.clone(),
                                args: BuiltinFuncArgs {
                                    length: args.length - call_args.len(),
                                    curried: new_args,
                                },
                            })
                        } else {
                            Err(format!(
                                "Function {:?} requires {} arguments, but got {}",
                                call_name,
                                args.length,
                                call_args.len()
                            ))
                        }
                    }
                    _ => Err("Function call requires a function".into()),
                }
            }
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
        }
    }
}
