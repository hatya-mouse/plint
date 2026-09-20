use crate::{Interpreter, Value, ast::Expr, interpreter::EvalCtx};

impl Interpreter {
    pub(super) fn eval_for_loop(
        &self,
        ctx: &mut EvalCtx,
        loop_var: &str,
        iterable: &Expr,
        body: &[Expr],
    ) -> Result<Value, String> {
        // First evaluate the iterable expression
        let iterable_value = self.eval_expr(ctx, iterable)?;

        // Construct a collection from the evaluated value
        // If the value is not a list, treat it as a single-element list
        let collection = match iterable_value {
            Value::List(list) => list,
            Value::Null => vec![],
            _ => vec![iterable_value],
        };

        // Loop over the collection and collect the results
        let mut results = Vec::new();
        for item in collection {
            // Set the loop variable in the context
            ctx.set_variable(loop_var.to_string(), item);

            // Evaluate the body of the loop
            let value = self.eval_exprs(ctx, body)?;

            // Add the result the the results vector when the result is not Null
            if value.is_some() {
                results.push(value);
            }
        }

        Ok(Value::List(results))
    }
}
