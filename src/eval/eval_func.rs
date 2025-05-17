use crate::ast::Expr;
use crate::eval::env::Env;
use crate::eval::eval::eval_expr;
use crate::eval::value::BuiltinFuncArgs;
use crate::eval::value::Value;

pub fn eval_func(
    name: &Box<Expr>,
    args: &Vec<Expr>,
    params: Vec<String>,
    body: Box<Expr>,
    func_env: Env,
    env: &mut Env,
) -> Result<Value, String> {
    // normal call
    if args.len() == params.len() {
        let mut new_env = func_env.clone();
        for (arg, arg_name) in args.iter().zip(params) {
            let arg_val = eval_expr(arg, env)?;
            new_env.set(arg_name, arg_val);
        }
        eval_expr(&body, &mut new_env)

    // currying
    } else if params.len() > args.len() {
        let mut new_env = Env::new(Some(Box::new(func_env.clone())));
        for (arg, arg_name) in args.iter().zip(params.iter()) {
            let arg_val = eval_expr(arg, env)?;
            new_env.set(arg_name.clone(), arg_val);
        }
        let remaining_params = params[args.len()..].to_vec();
        let remaining_body = body.clone();
        Ok(Value::Func {
            params: remaining_params,
            body: remaining_body,
            env: new_env,
        })
    } else {
        Err(format!(
            "Function {:?} expected {} arguments, but got {}",
            name,
            params.len(),
            args.len()
        ))
    }
}

pub fn eval_builtin_func(
    call_name: &Box<Expr>,
    call_args: &Vec<Expr>,
    name: String,
    func: fn(Vec<Value>) -> Result<Value, String>,
    args: BuiltinFuncArgs,
    env: &mut Env,
) -> Result<Value, String> {
    // normal function call
    if call_args.len() == args.length {
        let mut arg_vals = Vec::new();
        for arg in args.curried.iter() {
            arg_vals.push(arg.clone());
        }
        for arg in call_args {
            arg_vals.push(eval_expr(arg, env)?);
        }
        func(arg_vals)
    }
    // currying
    else if args.length > call_args.len() {
        let mut new_args = args.curried.clone();
        for arg in call_args {
            let arg_val = eval_expr(arg, env)?;
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
