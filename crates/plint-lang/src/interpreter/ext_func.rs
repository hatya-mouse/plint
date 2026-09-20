use crate::Value;

pub type ExtFunc = Box<dyn Fn(&[Value]) -> Result<Value, String>>;
