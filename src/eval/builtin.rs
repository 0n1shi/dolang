use super::eval::eval_expr;
use super::value::Value;

pub struct BuiltinFunc {
    pub name: &'static str,
    pub func: fn(Vec<Value>) -> Result<Value, String>,
    pub args_len: usize,
}

pub const BUILTIN_FUNCTIONS: &[BuiltinFunc] = &[
    // IO
    BuiltinFunc {
        name: "print",
        func: print,
        args_len: 1,
    },
    BuiltinFunc {
        name: "println",
        func: println,
        args_len: 1,
    },
    // File
    BuiltinFunc {
        name: "read_file",
        func: read_file,
        args_len: 1,
    },
    // List
    BuiltinFunc {
        name: "map",
        func: map,
        args_len: 2,
    },
    BuiltinFunc {
        name: "filter",
        func: filter,
        args_len: 2,
    },
    BuiltinFunc {
        name: "append",
        func: append,
        args_len: 2,
    },
    // String
    BuiltinFunc {
        name: "int",
        func: int,
        args_len: 1,
    },
    BuiltinFunc {
        name: "split",
        func: split,
        args_len: 2,
    },
    // Number
    BuiltinFunc {
        name: "str",
        func: str,
        args_len: 1,
    },
    // etc
    BuiltinFunc {
        name: "len",
        func: len,
        args_len: 1,
    },
    BuiltinFunc {
        name: "empty",
        func: empty,
        args_len: 1,
    },
];

fn format_value(value: &Value) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Boolean(b) => b.to_string(),
        Value::List(l) => {
            let items: Vec<String> = l.iter().map(format_value).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Func {
            params, body: _, ..
        } => {
            let params_str = params.join(", ");
            format!("Function: {}", params_str)
        }
        Value::BuiltinFunc { name, .. } => format!("Builtin function: {}", name),
    }
}

// IO
pub fn print(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("print: expected one argument".to_string());
    }
    let arg = &args[0];
    print!("{}", format_value(arg));
    Ok(Value::Number(0.0)) // Return a dummy
}

pub fn println(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("println: expected one argument".to_string());
    }
    let arg = &args[0];
    println!("{}", format_value(arg));
    Ok(Value::Number(0.0)) // Return a dummy
}

// File
pub fn read_file(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(path)] => {
            let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
            Ok(Value::String(content))
        }
        _ => Err("read_file: expected a string".to_string()),
    }
}

// List
pub fn map(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Func { params, body, env }, Value::List(items)] => {
            let mut results = Vec::new();
            for item in items {
                let mut new_env = env.clone();
                new_env.set(params[0].clone(), item.clone());
                let result = eval_expr(&body, &mut new_env)?;
                results.push(result);
            }
            Ok(Value::List(results))
        }
        _ => Err(format!(
            "map: expected a function and a list, got {:?}",
            args
        )),
    }
}

pub fn filter(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Func { params, body, env }, Value::List(items)] => {
            let mut results = Vec::new();
            for item in items {
                let mut new_env = env.clone();
                new_env.set(params[0].clone(), item.clone());
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

// String
pub fn int(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(s)] => match s.parse::<f64>() {
            Ok(n) => Ok(Value::Number(n)),
            Err(_) => Err("int: invalid number".to_string()),
        },
        _ => Err("int: expected a number".to_string()),
    }
}
pub fn split(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(delim), Value::String(s)] => {
            let parts: Vec<String> = s.split(delim).map(|s| s.to_string()).collect();
            Ok(Value::List(parts.into_iter().map(Value::String).collect()))
        }
        _ => Err("split: expected two strings".to_string()),
    }
}

// Number
pub fn str(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Number(n)] => Ok(Value::String(n.to_string())),
        _ => Err("str: expected a number".to_string()),
    }
}
// etc
pub fn len(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(l)] => Ok(Value::Number(l.len() as f64)),
        [Value::String(s)] => Ok(Value::Number(s.len() as f64)),
        _ => Err("len: expected a list or string".to_string()),
    }
}
pub fn empty(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(l)] => Ok(Value::Boolean(l.is_empty())),
        [Value::String(s)] => Ok(Value::Boolean(s.is_empty())),
        _ => Err("empty: expected a list or string".to_string()),
    }
}
