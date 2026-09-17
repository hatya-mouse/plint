mod de;
mod ser;

#[derive(Debug, Clone)]
pub struct Match {
    pub range: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl Value {
    pub(crate) fn force_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
        }
    }

    pub(crate) fn try_i64(&self) -> Option<i64> {
        match self {
            Value::String(s) => s.parse::<i64>().ok(),
            Value::Integer(i) => Some(*i),
            Value::Float(f) => Some(*f as i64),
            Value::Boolean(_) => None,
        }
    }

    pub(crate) fn try_f64(&self) -> Option<f64> {
        match self {
            Value::String(s) => s.parse::<f64>().ok(),
            Value::Integer(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            Value::Boolean(_) => None,
        }
    }

    pub(crate) fn try_bool(&self) -> Option<bool> {
        match self {
            Value::String(s) => s.parse::<bool>().ok(),
            Value::Integer(_) => None,
            Value::Float(_) => None,
            Value::Boolean(b) => Some(*b),
        }
    }
}
