use crate::eval::value::Value;

pub fn read_file(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::String(path)] => {
            let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
            Ok(Value::String(content))
        }
        _ => Err("read_file: expected a string".to_string()),
    }
}
