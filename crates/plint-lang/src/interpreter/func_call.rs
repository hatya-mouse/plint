use crate::{Interpreter, Value, ast::Expr, interpreter::EvalCtx};

impl Interpreter {
    pub(super) fn eval_func_call(
        &self,
        ctx: &mut EvalCtx,
        name: &str,
        args: &[Expr],
    ) -> Result<Value, String> {
        // Get the function
        let func = self
            .ext_funcs
            .get(name)
            .ok_or_else(|| format!("Function {} not found", name))?;

        // Evaluate the arguments
        let mut evaluated_args = Vec::with_capacity(args.len());
        for arg in args {
            evaluated_args.push(self.eval_expr(ctx, arg)?);
        }

        // Execute the function
        func(evaluated_args)
    }
}
