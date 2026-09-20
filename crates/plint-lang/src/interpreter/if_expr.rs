use crate::{
    Interpreter, Value,
    ast::{Expr, IfArm},
    interpreter::EvalCtx,
};

impl Interpreter {
    pub(super) fn eval_if_expr(
        &self,
        ctx: &mut EvalCtx,
        main: &IfArm,
        else_ifs: &[IfArm],
        else_body: Option<&Vec<Expr>>,
    ) -> Result<Value, String> {
        let if_arms = std::iter::once(main).chain(else_ifs.iter());

        // Evaluate each if arm in order
        for if_arm in if_arms {
            let cond_bool = self.eval_if_cond(ctx, &if_arm.condition)?;
            if cond_bool {
                return self.eval_exprs(ctx, &if_arm.body);
            }
        }

        // If no if arm was true, evaluate the else body
        if let Some(else_body) = else_body {
            self.eval_exprs(ctx, else_body)
        } else {
            Ok(Value::Null)
        }
    }

    fn eval_if_cond(&self, ctx: &mut EvalCtx, cond: &Expr) -> Result<bool, String> {
        // Evaluate the condition
        let cond_value = self.eval_expr(ctx, cond)?;

        match cond_value {
            Value::Bool(cond_bool) => Ok(cond_bool),
            _ => Err(format!(
                "Condition must evaluate to a boolean, got: {}",
                cond_value
            )),
        }
    }
}
