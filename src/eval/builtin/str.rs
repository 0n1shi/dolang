use crate::eval::value::Value;

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
