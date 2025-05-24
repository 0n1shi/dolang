use crate::eval::value::Value;

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

pub fn not_empty(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(l)] => Ok(Value::Boolean(!l.is_empty())),
        [Value::String(s)] => Ok(Value::Boolean(!s.is_empty())),
        _ => Err("notEmpty: expected a list or string".to_string()),
    }
}
