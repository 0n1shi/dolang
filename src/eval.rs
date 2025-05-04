use crate::ast::{CompOp, Expr, FactorOp, LogicOp, Stmt, TermOp, UnaryOp, AST};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Tuple(Vec<Value>),
}

pub struct Env {
    variables: std::collections::HashMap<String, Value>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new(parent: Option<Box<Env>>) -> Self {
        Env {
            variables: std::collections::HashMap::new(),
            parent,
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value)
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
}

pub struct Evaluator {
    ast: AST,
}

impl Evaluator {
    pub fn new(ast: AST) -> Self {
        Evaluator { ast }
    }

    pub fn eval(&self, env: &mut Env) -> Result<(), String> {
        for stmt in &self.ast.stmts {
            self.eval_stmt(&stmt, env)?;
        }
        Ok(())
    }

    fn eval_stmt(&self, stmt: &Stmt, env: &mut Env) -> Result<(), String> {
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
            Stmt::Print(expr) => {
                let value = self.eval_expr(expr, env)?;
                match value {
                    Value::Number(n) => println!("{}", n),
                    Value::String(s) => println!("\"{}\"", s),
                    Value::Boolean(b) => println!("{}", b),
                    Value::List(l) => {
                        let list_str: Vec<String> = l
                            .iter()
                            .map(|v| match v {
                                Value::Number(n) => n.to_string(),
                                Value::String(s) => s.clone(),
                                Value::Boolean(b) => b.to_string(),
                                _ => "Unsupported type".to_string(),
                            })
                            .collect();
                        println!("[{}]", list_str.join(", "));
                    }
                    Value::Tuple(t) => {
                        let tuple_str: Vec<String> = t
                            .iter()
                            .map(|v| match v {
                                Value::Number(n) => n.to_string(),
                                Value::String(s) => s.clone(),
                                Value::Boolean(b) => b.to_string(),
                                _ => "Unsupported type".to_string(),
                            })
                            .collect();
                        println!("({})", tuple_str.join(", "));
                    }
                }
                Ok(())
            }
        }
    }

    fn eval_expr(&self, expr: &Expr, env: &mut Env) -> Result<Value, String> {
        match expr {
            Expr::If { cond, then, else_ } => {
                let cond_val = self.eval_expr(cond, env)?;
                match cond_val {
                    Value::Boolean(true) => self.eval_expr(then, env),
                    Value::Boolean(false) => self.eval_expr(else_, env),
                    _ => Err("Condition must be a boolean".into()),
                }
            }
            Expr::List(items) => {
                let mut values = Vec::new();
                for item in items {
                    values.push(self.eval_expr(item, env)?);
                }
                Ok(Value::List(values))
            }
            Expr::Tuple(items) => {
                let mut values = Vec::new();
                for item in items {
                    values.push(self.eval_expr(item, env)?);
                }
                Ok(Value::List(values))
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
                        _ => {
                            Err("Greater than or equal comparison requires number operands".into())
                        }
                    },
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
                        Value::Tuple(t) => Value::Tuple(t.to_vec()),
                        Value::List(l) => Value::List(l.to_vec()),
                    })
                } else {
                    Err(format!("Undefined variable: {}", expr))
                }
            }
            Expr::ListAccess { list, index } => {
                let list_val = self.eval_expr(list, env)?;
                match list_val {
                    Value::List(l) => {
                        if *index as usize >= l.len() {
                            Err(format!("Index out of bounds: {}", index))
                        } else {
                            Ok(l[*index as usize].clone())
                        }
                    }
                    _ => Err("List access requires a list".into()),
                }
            }
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
        }
    }
}
