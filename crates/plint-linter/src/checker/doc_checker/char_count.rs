use crate::{
    Document, LinterError,
    checker::{CheckResult, CheckResultType, Checker, CheckerInit},
};
use unicode_segmentation::UnicodeSegmentation;

pub struct CharCountChecker;

impl CheckerInit for CharCountChecker {
    fn new(_args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError> {
        Ok(Self)
    }
}

impl Checker for CharCountChecker {
    fn check(&self, doc: &Document) -> CheckResult {
        CheckResult::Value(crate::Value::Integer(
            doc.content
                .graphemes(true)
                .count()
                .try_into()
                .unwrap_or_default(),
        ))
    }

    fn check_type(&self) -> CheckResultType {
        CheckResultType::Value
    }
}
