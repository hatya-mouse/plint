use plint_lang::{Interpreter, Value};

pub(super) fn add_funcs(interpreter: &mut Interpreter) {
    interpreter.add_func("contains", Box::new(contains));
    interpreter.add_func("starts_with", Box::new(starts_with));
    interpreter.add_func("ends_with", Box::new(ends_with));
    interpreter.add_func("regex", Box::new(regex));
    interpreter.add_func("text", Box::new(text));
}

fn contains(args: &[Value]) -> Result<Value, String> {
    let (string, pattern): (&str, &str) = match args {
        [Value::String(text), Value::String(pattern)] => (text, pattern),
        [Value::String(text), Value::Match(m), Value::String(pattern)] => match text.get(*m) {
            Some(substring) => (substring, pattern),
            None => return Ok(Value::Null),
        },
        _ => return Err("Invalid arguments for contains function".to_string()),
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
        _ => return Err("Invalid arguments for starts_with function".to_string()),
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
        _ => return Err("Invalid arguments for ends_with function".to_string()),
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
        _ => return Err("Invalid arguments for regex function".to_string()),
    };

    // Get the matches with regex
    let regex = regex::Regex::new(pattern).map_err(|err| err.to_string())?;
    Ok(Value::List(
        regex
            .find_iter(string)
            .map(|m| Value::Match(m.range().into()))
            .collect(),
    ))
}

fn text(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::String(text)] => Ok(Value::String(text.to_string())),
        [Value::Integer(integer)] => Ok(Value::String(integer.to_string())),
        [Value::Bool(boolean)] => Ok(Value::String(boolean.to_string())),
        [Value::String(doc), Value::Match(m)] => Ok(doc
            .get(*m)
            .map(|str| Value::String(str.to_string()))
            .unwrap_or(Value::Null)),
        _ => Err("Invalid arguments for text function".to_string()),
    }
}
