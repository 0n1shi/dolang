use super::eval::eval_expr;
use super::value::Value;

pub fn map(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Func { args, body, env }, Value::List(items)] => {
            let mut results = Vec::new();
            for item in items {
                let mut new_env = env.clone();
                new_env.set(args[0].clone(), item.clone());
                let result = eval_expr(&body, &mut new_env)?;
                results.push(result);
            }
            Ok(Value::List(results))
        }
        _ => Err("map: expected a function and a list".to_string()),
    }
}

pub fn print(args: Vec<Value>) -> Result<Value, String> {
    for arg in args {
        match arg {
            Value::Number(n) => println!("{}", n),
            Value::String(s) => println!("{}", s),
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
            Value::Func { args, body, .. } => {
                let args_str = args.join(", ");
                println!("Function: {} -> {:?}", args_str, body);
            }
            Value::BuiltinFunc { name, .. } => {
                println!("Builtin function: {}", name);
            }
        }
    }
    Ok(Value::Number(0.0)) // Return a dummy
}
