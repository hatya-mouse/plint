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

mod calc;
mod count;
mod logic;
mod md;
mod string;

use plint_lang::Interpreter;

pub(super) fn add_ext_funcs(interpreter: &mut Interpreter) {
    calc::add_funcs(interpreter);
    count::add_funcs(interpreter);
    logic::add_funcs(interpreter);
    string::add_funcs(interpreter);
    md::add_funcs(interpreter);
}
