use crate::{
    Document, LinterError,
    checker::{CheckResult, CheckResultType, Checker, CheckerInit},
};
use unicode_segmentation::UnicodeSegmentation;

pub struct WordCountChecker;

impl CheckerInit for WordCountChecker {
    fn new(_args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError> {
        Ok(Self)
    }
}

impl Checker for WordCountChecker {
    fn check(&self, doc: &Document) -> CheckResult {
        CheckResult::Value(crate::Value::Integer(
            doc.content
                .unicode_words()
                .count()
                .try_into()
                .unwrap_or_default(),
        ))
    }

    fn check_type(&self) -> CheckResultType {
        CheckResultType::Value
    }
}
