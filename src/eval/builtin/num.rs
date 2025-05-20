use crate::eval::value::Value;

pub fn str(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Number(n)] => Ok(Value::String(n.to_string())),
        _ => Err("str: expected a number".to_string()),
    }
}
