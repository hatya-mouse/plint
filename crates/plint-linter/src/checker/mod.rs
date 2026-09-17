mod std_checker;
mod variants;

pub(crate) use variants::checker_from;

use crate::{Document, LinterError, Match};
use enum_dispatch::enum_dispatch;
use std_checker::{
    byte_count::ByteCountChecker, char_count::CharCountChecker, line_count::LineCountChecker,
    regex::RegexChecker,
};
use variants::CheckerEnum;

pub enum CheckResult {
    Matches(Vec<Match>),
    Value(crate::Value),
}

pub trait CheckerInit: Sized {
    fn new(args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError>;
}

#[enum_dispatch]
pub trait Checker {
    fn check(&self, doc: &Document) -> CheckResult;
}
