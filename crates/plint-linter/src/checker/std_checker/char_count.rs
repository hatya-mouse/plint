use crate::{
    Document, LinterError,
    checker::{CheckResult, Checker, CheckerInit},
};

pub struct CharCountChecker {}

impl CheckerInit for CharCountChecker {
    fn new(_args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError> {
        Ok(Self {})
    }
}

impl Checker for CharCountChecker {
    fn check(&self, doc: &Document) -> CheckResult {
        CheckResult::Value(crate::Value::Integer(
            doc.content.chars().count().try_into().unwrap_or_default(),
        ))
    }
}
