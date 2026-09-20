use plint_lang::{Interpreter, Value};

pub(super) fn add_funcs(interpreter: &mut Interpreter) {
    interpreter.add_func("eq", Box::new(eq));
    interpreter.add_func("neq", Box::new(neq));
    interpreter.add_func("gt", Box::new(gt));
    interpreter.add_func("gte", Box::new(gte));
    interpreter.add_func("lt", Box::new(lt));
    interpreter.add_func("lte", Box::new(lte));
    interpreter.add_func("and", Box::new(and));
    interpreter.add_func("or", Box::new(or));
    interpreter.add_func("not", Box::new(not));
}

fn eq(args: &[Value]) -> Result<Value, String> {
    if let [a, b] = args {
        Ok(Value::Bool(a == b))
    } else {
        Err("Argument is invalid for eq function".to_string())
    }
}

fn neq(args: &[Value]) -> Result<Value, String> {
    if let [a, b] = args {
        Ok(Value::Bool(a != b))
    } else {
        Err("Argument is invalid for neq function".to_string())
    }
}

fn gt(args: &[Value]) -> Result<Value, String> {
    if let [Value::Integer(a), Value::Integer(b)] = args {
        Ok(Value::Bool(a > b))
    } else if let [Value::Bool(a), Value::Bool(b)] = args {
        Ok(Value::Bool(a > b))
    } else if let [Value::String(a), Value::String(b)] = args {
        Ok(Value::Bool(a.len() > b.len()))
    } else {
        Err("Argument is invalid for gt function".to_string())
    }
}

fn gte(args: &[Value]) -> Result<Value, String> {
    if let [Value::Integer(a), Value::Integer(b)] = args {
        Ok(Value::Bool(a >= b))
    } else if let [Value::Bool(a), Value::Bool(b)] = args {
        Ok(Value::Bool(a >= b))
    } else if let [Value::String(a), Value::String(b)] = args {
        Ok(Value::Bool(a.len() >= b.len()))
    } else {
        Err("Argument is invalid for gte function".to_string())
    }
}

fn lt(args: &[Value]) -> Result<Value, String> {
    if let [Value::Integer(a), Value::Integer(b)] = args {
        Ok(Value::Bool(a < b))
    } else if let [Value::Bool(a), Value::Bool(b)] = args {
        Ok(Value::Bool(a < b))
    } else if let [Value::String(a), Value::String(b)] = args {
        Ok(Value::Bool(a.len() < b.len()))
    } else {
        Err("Argument is invalid for lt function".to_string())
    }
}

fn lte(args: &[Value]) -> Result<Value, String> {
    if let [Value::Integer(a), Value::Integer(b)] = args {
        Ok(Value::Bool(a <= b))
    } else if let [Value::Bool(a), Value::Bool(b)] = args {
        Ok(Value::Bool(a <= b))
    } else if let [Value::String(a), Value::String(b)] = args {
        Ok(Value::Bool(a.len() <= b.len()))
    } else {
        Err("Argument is invalid for leq function".to_string())
    }
}

fn and(args: &[Value]) -> Result<Value, String> {
    if !args.is_empty() && args.iter().all(|value| matches!(value, Value::Bool(_))) {
        Ok(Value::Bool(
            args.iter().all(|value| matches!(value, Value::Bool(true))),
        ))
    } else {
        Err("and function can only take boolean values".to_string())
    }
}

fn or(args: &[Value]) -> Result<Value, String> {
    if !args.is_empty() && args.iter().all(|value| matches!(value, Value::Bool(_))) {
        Ok(Value::Bool(
            args.iter().any(|value| matches!(value, Value::Bool(true))),
        ))
    } else {
        Err("or function can only take boolean values".to_string())
    }
}

fn not(args: &[Value]) -> Result<Value, String> {
    if let [Value::Bool(boolean)] = args {
        Ok(Value::Bool(!boolean))
    } else {
        Err("not function can only take a single boolean value".to_string())
    }
}
