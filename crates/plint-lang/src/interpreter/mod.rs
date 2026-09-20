use crate::{Value, ast::Expr, parser::exprs};
use std::collections::HashMap;

pub type ExtFunc = fn(Vec<Value>) -> Value;

pub struct Interpreter {
    /// Externally defined functions.
    ext_funcs: HashMap<String, ExtFunc>,
    /// Externally defined constants.
    ext_consts: HashMap<String, Value>,
}

impl Interpreter {
    /// Parses and evaluates the given code, returning the result of the last expression.
    pub fn run(&self, code: &str) -> Result<Value, String> {
        // First parse the code into expressions
        let (rest, parsed) = exprs(code).map_err(|err| err.to_string())?;
        // Return an error if there is any remaining unparsed code,
        // which means that code contains unknown syntax that cannot be interpreted
        if !rest.is_empty() {
            return Err(format!("Unknown syntax: {}", rest));
        }

        // Evaluate the expressions
        self.eval_exprs(&parsed)
    }

    /// Evaluates a list of expressions and returns the value of the last expression.
    fn eval_exprs(&self, exprs: &[Expr]) -> Result<Value, String> {
        let mut last_value = Value::Null;
        for expr in exprs {
            last_value = self.eval_expr(expr)?;
        }

        Ok(last_value)
    }

    /// Evaluates a single expression and returns the result as a Value.
    fn eval_expr(&self, expr: &Expr) -> Result<Value, String> {
        todo!()
    }
}
