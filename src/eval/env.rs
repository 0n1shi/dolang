use crate::eval::builtin::BUILTIN_FUNCTIONS;
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
        for (name, func) in BUILTIN_FUNCTIONS.iter() {
            variables.insert(
                name.to_string(),
                Value::BuiltinFunc {
                    name: name.to_string(),
                    func: *func,
                },
            );
        }

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
