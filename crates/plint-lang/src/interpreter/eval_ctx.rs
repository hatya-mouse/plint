use crate::Value;
use std::collections::HashMap;

/// The `EvalCtx` struct holds the context when evaluating expressions.
pub(super) struct EvalCtx {
    /// Currently defined local variables in the scope.
    variables: HashMap<String, Value>,
}

impl Default for EvalCtx {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl EvalCtx {
    /// Sets a variable to the given value, returning the previous value if it existed.
    pub(super) fn set_variable(&mut self, name: String, value: Value) -> Option<Value> {
        self.variables.insert(name, value)
    }

    /// Returns the value of a variable if it exists, otherwise returns None.
    pub(super) fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}
