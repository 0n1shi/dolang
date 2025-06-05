use crate::eval::eval::Evaluator;
use crate::eval::value::Value;

pub fn map(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::Func { params, body, env }, Value::List(items)] => {
            let mut results = Vec::new();
            for item in items {
                let mut new_env = env.clone();
                new_env.set(params[0].clone(), item.clone());
                let result = Evaluator::new("".to_string()).eval_expr(&body, &mut new_env)?;
                results.push(result);
            }
            Ok(Value::List(results))
        }
        [Value::BuiltinFunc {
            name: _,
            func,
            args,
        }, Value::List(items)] => {
            let mut results = Vec::new();
            for item in items {
                let mut args = args.curried.clone();
                args.push(item.clone());
                let result = func(args)?;
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
                let result = Evaluator::new("".to_string()).eval_expr(&body, &mut new_env)?;
                if result == Value::Boolean(true) {
                    results.push(item.clone());
                }
            }
            Ok(Value::List(results))
        }
        [Value::BuiltinFunc {
            name: _,
            func,
            args,
        }, Value::List(items)] => {
            let mut results = Vec::new();
            for item in items {
                let mut args = args.curried.clone();
                args.push(item.clone());
                let result = func(args)?;
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

pub fn first(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(l)] => l.get(0).cloned().ok_or("first: list is empty".to_string()),
        _ => Err("first: expected a list".to_string()),
    }
}

pub fn second(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(l)] => l.get(1).cloned().ok_or("second: list is empty".to_string()),
        _ => Err("second: expected a list".to_string()),
    }
}

pub fn third(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(l)] => l.get(2).cloned().ok_or("third: list is empty".to_string()),
        _ => Err("third: expected a list".to_string()),
    }
}

pub fn tail(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(l)] => {
            if l.len() > 1 {
                Ok(Value::List(l[1..].to_vec()))
            } else {
                Ok(Value::List(vec![]))
            }
        }
        _ => Err("rest: expected a list".to_string()),
    }
}

pub fn last(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(l)] => l.last().cloned().ok_or("last: list is empty".to_string()),
        _ => Err("last: expected a list".to_string()),
    }
}

pub fn sum(args: Vec<Value>) -> Result<Value, String> {
    match args.as_slice() {
        [Value::List(l)] => {
            let sum: f64 = l
                .iter()
                .filter_map(|v| match v {
                    Value::Number(n) => Some(*n),
                    _ => None,
                })
                .sum();
            Ok(Value::Number(sum))
        }
        _ => Err("sum: expected a list".to_string()),
    }
}
