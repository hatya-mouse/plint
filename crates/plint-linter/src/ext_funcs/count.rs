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
use unicode_segmentation::UnicodeSegmentation;

pub(super) fn add_funcs(interpreter: &mut Interpreter) {
    interpreter.add_func("len", Box::new(len));
    interpreter.add_func("char_count", Box::new(char_count));
    interpreter.add_func("byte_count", Box::new(byte_count));
    interpreter.add_func("word_count", Box::new(word_count));
    interpreter.add_func("line_count", Box::new(line_count));
}

fn len(args: &[Value]) -> Result<Value, String> {
    if let [Value::List(list)] = args {
        Ok(Value::Integer(list.len().try_into().unwrap_or_default()))
    } else {
        Err("Invalid arguments for len function".to_string())
    }
}

fn char_count(args: &[Value]) -> Result<Value, String> {
    let string = match args {
        [Value::String(text)] => text,
        [Value::String(text), Value::Match(m)] => match text.get(m.clone()) {
            Some(substring) => substring,
            None => return Ok(Value::Null),
        },
        _ => return Err("Invalid arguments for char_count function".to_string()),
    };

    Ok(Value::Integer(
        string
            .graphemes(true)
            .count()
            .try_into()
            .unwrap_or_default(),
    ))
}

fn byte_count(args: &[Value]) -> Result<Value, String> {
    let string = match args {
        [Value::String(text)] => text,
        [Value::String(text), Value::Match(m)] => match text.get(m.clone()) {
            Some(substring) => substring,
            None => return Ok(Value::Null),
        },
        _ => return Err("Invalid arguments for byte_count function".to_string()),
    };

    Ok(Value::Integer(string.len().try_into().unwrap_or_default()))
}

fn word_count(args: &[Value]) -> Result<Value, String> {
    let string = match args {
        [Value::String(text)] => text,
        [Value::String(text), Value::Match(m)] => match text.get(m.clone()) {
            Some(substring) => substring,
            None => return Ok(Value::Null),
        },
        _ => return Err("Invalid arguments for word_count function".to_string()),
    };

    Ok(Value::Integer(
        string
            .unicode_words()
            .count()
            .try_into()
            .unwrap_or_default(),
    ))
}

fn line_count(args: &[Value]) -> Result<Value, String> {
    let string = match args {
        [Value::String(text)] => text,
        [Value::String(text), Value::Match(m)] => match text.get(m.clone()) {
            Some(substring) => substring,
            None => return Ok(Value::Null),
        },
        _ => return Err("Invalid arguments for line_count function".to_string()),
    };

    Ok(Value::Integer(
        string.lines().count().try_into().unwrap_or_default(),
    ))
}
