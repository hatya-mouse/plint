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
}
