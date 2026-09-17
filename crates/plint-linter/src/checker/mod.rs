mod std_checker;

use crate::{Document, LinterError, Match, Value};
use std_checker::regex::RegexChecker;

pub trait Checker: Sized {
    fn new(args: &Option<yaml_serde::Mapping>) -> Result<Self, LinterError>;

    fn check(&self, doc: &Document) -> CheckResult;
}

pub enum CheckResult {
    Matches(Vec<Match>),
    Value(Value),
}

pub(crate) fn checker_from(
    check: &str,
    args: &Option<yaml_serde::Mapping>,
) -> Result<Box<impl Checker>, LinterError> {
    match check {
        "std.regex" => Ok(Box::new(RegexChecker::new(args)?)),
        _ => Err(LinterError::UnknownChecker(check.to_string())),
    }
}
