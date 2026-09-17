use crate::Value;
use serde::Serialize;

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::String(v) => serializer.serialize_str(v),
            Value::Integer(v) => serializer.serialize_i64(*v),
            Value::Float(v) => serializer.serialize_f64(*v),
            Value::Boolean(v) => serializer.serialize_bool(*v),
        }
    }
}
