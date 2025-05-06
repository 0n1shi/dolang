use super::eval::eval_expr;
use super::value::Value;

pub const BUILTIN_FUNCTIONS: &[(&str, fn(Vec<Value>) -> Result<Value, String>)] = &[
    ("map", map),
    ("filter", filter),
    ("print", print),
    ("println", println),
    ("append", append),
];

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

pub fn filter(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Func { args, body, env }, Value::List(items)] => {
            let mut results = Vec::new();
            for item in items {
                let mut new_env = env.clone();
                new_env.set(args[0].clone(), item.clone());
                let result = eval_expr(&body, &mut new_env)?;
                if result == Value::Boolean(true) {
                    results.push(item.clone());
                }
            }
            Ok(Value::List(results))
        }
        _ => Err("filter: expected a function and a list".to_string()),
    }
}

pub fn print(args: Vec<Value>) -> Result<Value, String> {
    for arg in args {
        match arg {
            Value::Number(n) => print!("{}", n),
            Value::String(s) => print!("{}", s),
            Value::Boolean(b) => print!("{}", b),
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
                print!("[{}]", list_str.join(", "));
            }
            Value::Func { args, body, .. } => {
                let args_str = args.join(", ");
                print!("Function: {} -> {:?}", args_str, body);
            }
            Value::BuiltinFunc { name, .. } => {
                print!("Builtin function: {}", name);
            }
        }
    }
    Ok(Value::Number(0.0)) // Return a dummy
}

pub fn println(args: Vec<Value>) -> Result<Value, String> {
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

pub fn append(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(list1), Value::List(list2)] => {
            let mut new_list = list1.clone();
            new_list.extend(list2.clone());
            Ok(Value::List(new_list))
        }
        [Value::List(list1), item] => {
            let mut new_list = list1.clone();
            new_list.push(item.clone());
            Ok(Value::List(new_list))
        }
        _ => Err("append: expected two lists".to_string()),
    }
}
