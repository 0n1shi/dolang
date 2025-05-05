use crate::eval::builtin;
use crate::eval::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    variables: std::collections::HashMap<String, Value>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new(parent: Option<Box<Env>>) -> Self {
        let mut variables = std::collections::HashMap::new();

        // register built-in functions
        variables.insert(
            "map".to_string(),
            Value::BuiltinFunc {
                name: "map".to_string(),
                func: builtin::map,
            },
        );
        variables.insert(
            "print".to_string(),
            Value::BuiltinFunc {
                name: "print".to_string(),
                func: builtin::print,
            },
        );
        Env { variables, parent }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value)
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
}
