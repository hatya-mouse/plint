use crate::{
    Document, LinterError,
    checker::{CheckResult, Checker, CheckerInit},
};

pub struct ByteCountChecker;

impl CheckerInit for ByteCountChecker {
    fn new(_args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError> {
        Ok(Self)
    }
}

impl Checker for ByteCountChecker {
    fn check(&self, doc: &Document) -> CheckResult {
        CheckResult::Value(crate::Value::Integer(
            doc.content.len().try_into().unwrap_or_default(),
        ))
    }
}
