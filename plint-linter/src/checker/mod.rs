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
