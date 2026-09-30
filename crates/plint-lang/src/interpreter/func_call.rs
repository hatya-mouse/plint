//
//  Copyright 2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

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
        func(&evaluated_args)
    }
}
