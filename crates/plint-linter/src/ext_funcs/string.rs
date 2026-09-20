use plint_lang::{Interpreter, Value};

pub(super) fn add_funcs(interpreter: &mut Interpreter) {
    interpreter.add_func("contains", Box::new(contains));
    interpreter.add_func("starts_with", Box::new(starts_with));
    interpreter.add_func("ends_with", Box::new(ends_with));
    interpreter.add_func("regex", Box::new(regex));
}

fn contains(args: &[Value]) -> Result<Value, String> {
    let (string, pattern): (&str, &str) = match args {
        [Value::String(text), Value::String(pattern)] => (text, pattern),
        [Value::String(text), Value::Match(m), Value::String(pattern)] => match text.get(*m) {
            Some(substring) => (substring, pattern),
            None => return Ok(Value::Null),
        },
        _ => return Ok(Value::Null),
    };

    Ok(Value::Bool(string.contains(pattern)))
}

fn starts_with(args: &[Value]) -> Result<Value, String> {
    let (string, pattern): (&str, &str) = match args {
        [Value::String(text), Value::String(pattern)] => (text, pattern),
        [Value::String(text), Value::Match(m), Value::String(pattern)] => match text.get(*m) {
            Some(substring) => (substring, pattern),
            None => return Ok(Value::Null),
        },
        _ => return Ok(Value::Null),
    };

    Ok(Value::Bool(string.starts_with(pattern)))
}

fn ends_with(args: &[Value]) -> Result<Value, String> {
    let (string, pattern): (&str, &str) = match args {
        [Value::String(text), Value::String(pattern)] => (text, pattern),
        [Value::String(text), Value::Match(m), Value::String(pattern)] => match text.get(*m) {
            Some(substring) => (substring, pattern),
            None => return Ok(Value::Null),
        },
        _ => return Ok(Value::Null),
    };

    Ok(Value::Bool(string.ends_with(pattern)))
}

fn regex(args: &[Value]) -> Result<Value, String> {
    let (string, pattern): (&str, &str) = match args {
        [Value::String(text), Value::String(pattern)] => (text, pattern),
        [Value::String(text), Value::Match(m), Value::String(pattern)] => match text.get(*m) {
            Some(substring) => (substring, pattern),
            None => return Ok(Value::Null),
        },
        _ => return Ok(Value::Null),
    };

    // Get the matches with regex
    let regex = regex::Regex::new(pattern).map_err(|err| err.to_string())?;
    match regex.captures(string) {
        Some(matches) => Ok(Value::List(
            matches
                .iter()
                .filter_map(|m| m.map(|m| m.range()))
                .map(|range| Value::Match(range.into()))
                .collect(),
        )),
        None => Ok(Value::List(Vec::new())),
    }
}
