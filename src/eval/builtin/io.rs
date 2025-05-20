use crate::eval::value::Value;

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
