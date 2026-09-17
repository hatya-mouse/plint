mod std_checker;

pub use std_checker::RegexChecker;

use crate::Document;
use ::std::ops::Range;

pub trait Checker: Sized {
    fn new(args: yaml_serde::Value) -> Result<Self, Box<dyn std::error::Error>>;

    fn check(&self, doc: &Document) -> Result<CheckResult, Box<dyn std::error::Error>>;
}

pub enum CheckResult {
    Matches(Vec<Match>),
    Value(Value),
}

pub struct Match {
    pub range: Range<usize>,
}

pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl Value {
    pub(crate) fn into_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
        }
    }

    pub(crate) fn into_i64(&self) -> Option<i64> {
        match self {
            Value::String(s) => s.parse::<i64>().ok(),
            Value::Integer(i) => Some(*i),
            Value::Float(f) => Some(*f as i64),
            Value::Boolean(_) => None,
        }
    }

    pub(crate) fn into_f64(&self) -> Option<f64> {
        match self {
            Value::String(s) => s.parse::<f64>().ok(),
            Value::Integer(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            Value::Boolean(_) => None,
        }
    }

    pub(crate) fn into_bool(&self) -> Option<bool> {
        match self {
            Value::String(s) => s.parse::<bool>().ok(),
            Value::Integer(_) => None,
            Value::Float(_) => None,
            Value::Boolean(b) => Some(*b),
        }
    }
}
