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

use plint_lang::{Interpreter, Value};

pub(super) fn add_funcs(interpreter: &mut Interpreter) {
    interpreter.add_func("sum", Box::new(sum));
    interpreter.add_func("sub", Box::new(sub));
    interpreter.add_func("mul", Box::new(mul));
    interpreter.add_func("div", Box::new(div));
}

fn sum(args: &[Value]) -> Result<Value, String> {
    let mut sum = 0i64;
    for arg in args {
        if let Value::Integer(integer) = arg {
            sum += integer;
        } else {
            return Err("Non-integer value is passed to sum function".to_string());
        }
    }

    Ok(Value::Integer(sum))
}

fn sub(args: &[Value]) -> Result<Value, String> {
    if let [Value::Integer(a), Value::Integer(b)] = args {
        Ok(Value::Integer(a - b))
    } else {
        Err("Argument is invalid for sub function".to_string())
    }
}

fn mul(args: &[Value]) -> Result<Value, String> {
    let mut mul = 0i64;
    for arg in args {
        if let Value::Integer(integer) = arg {
            mul *= integer;
        } else {
            return Err("Non-integer value is passed to mul function".to_string());
        }
    }

    Ok(Value::Integer(mul))
}

fn div(args: &[Value]) -> Result<Value, String> {
    if let [Value::Integer(a), Value::Integer(b)] = args {
        if *b == 0 {
            Err("0 is passed as a divider".to_string())
        } else {
            Ok(Value::Integer(a / b))
        }
    } else {
        Err("Argument is invalid for sub function".to_string())
    }
}
