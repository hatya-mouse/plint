mod std_checker;

pub use std_checker::RegexChecker;

use crate::Document;
use ::std::ops::Range;
use serde::{Deserialize, Serialize};

pub trait Checker<'a>: Sized + Serialize + Deserialize<'a> {
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
