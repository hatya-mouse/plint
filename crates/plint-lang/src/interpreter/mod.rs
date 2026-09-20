mod eval_ctx;
mod for_loop;

use crate::{Value, ast::Expr, interpreter::eval_ctx::EvalCtx, parser::exprs};
use std::collections::HashMap;

pub type ExtFunc = fn(Vec<Value>) -> Result<Value, String>;

/// The `Interpreter` struct parses and evaluates the given code
/// and returns the result of the last expression.
///
/// You can add functions and constants to the interpreter by using the `add_func` and `add_const` methods.
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
        let mut ctx = EvalCtx::default();
        self.eval_exprs(&mut ctx, &parsed)
    }

    /// Adds an externally defined function to the interpreter
    /// and returns the existing function if it was already added to the interpreter.
    pub fn add_func(&mut self, name: &str, func: ExtFunc) -> Option<ExtFunc> {
        self.ext_funcs.insert(name.to_string(), func)
    }

    /// Adds an externally defined constant to the interpreter and returns
    /// the existing value of the constant if it was already added to the interpreter.
    pub fn add_const(&mut self, name: &str, value: Value) -> Option<Value> {
        self.ext_consts.insert(name.to_string(), value)
    }

    // --- EVALUATION ---

    /// Evaluates a list of expressions and returns the value of the last expression.
    fn eval_exprs(&self, ctx: &mut EvalCtx, exprs: &[Expr]) -> Result<Value, String> {
        let mut last_value = Value::Null;
        for expr in exprs {
            last_value = self.eval_expr(ctx, expr)?;
        }

        Ok(last_value)
    }

    /// Evaluates a single expression and returns the result as a Value.
    fn eval_expr(&self, ctx: &mut EvalCtx, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::For {
                loop_var,
                iterable,
                body,
            } => self.eval_for_loop(ctx, loop_var, iterable, body),
            Expr::If {
                main,
                else_ifs,
                else_body,
            } => {}
            Expr::Literal(value) => {}
            Expr::FunctionCall { name, args } => {}
            Expr::Assign { name, value } => {}
            Expr::Variable { name } => {}
        }
    }
}
