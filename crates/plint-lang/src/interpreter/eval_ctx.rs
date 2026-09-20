use crate::Value;
use std::collections::HashMap;

/// The `EvalCtx` struct holds the context when evaluating expressions.
#[derive(Default)]
pub(super) struct EvalCtx {
    /// Currently defined local variables in the scope.
    variables: HashMap<String, Value>,
}

impl EvalCtx {
    /// Sets a variable to the given value, returning the previous value if it existed.
    pub(super) fn set_var(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    /// Returns the value of a variable if it exists, otherwise returns None.
    pub(super) fn get_var(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    /// Returns whether the context has a variable with the given name.
    pub(super) fn has_var(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}
