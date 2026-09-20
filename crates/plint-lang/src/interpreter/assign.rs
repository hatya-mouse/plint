use crate::{Interpreter, Value, ast::Expr, interpreter::EvalCtx};

impl Interpreter {
    pub(super) fn eval_assign(
        &self,
        ctx: &mut EvalCtx,
        name: &str,
        expr: &Expr,
    ) -> Result<Value, String> {
        // Evaluate the rhs expression
        let value = self.eval_expr(ctx, expr)?;
        // Assign the evaluated value to the variable
        self.set_var(ctx, name, value)?;

        Ok(Value::Null)
    }
}
