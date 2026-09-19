use crate::{
    Document, LinterError,
    checker::{CheckResult, CheckResultType, Checker, CheckerInit},
};

pub struct LineCountChecker;

impl CheckerInit for LineCountChecker {
    fn new(_args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError> {
        Ok(Self)
    }
}

impl Checker for LineCountChecker {
    fn check(&self, doc: &Document) -> CheckResult {
        CheckResult::Value(crate::Value::Integer(
            doc.content.lines().count().try_into().unwrap_or_default(),
        ))
    }

    fn check_type(&self) -> CheckResultType {
        CheckResultType::Value
    }
}
