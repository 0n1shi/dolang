use crate::eval::builtin::BUILTIN_FUNCTIONS;
use crate::eval::value::{BuiltinFuncArgs, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    variables: std::collections::HashMap<String, Value>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new(parent: Option<Box<Env>>) -> Self {
        let mut variables = std::collections::HashMap::new();

        // register built-in functions
        for func in BUILTIN_FUNCTIONS {
            variables.insert(
                func.name.to_string(),
                Value::BuiltinFunc {
                    name: func.name.to_string(),
                    func: func.func,
                    args: BuiltinFuncArgs {
                        length: func.args_len,
                        curried: vec![],
                    },
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
