use crate::{
    Document, LinterError,
    checker::{CheckResult, Checker, CheckerInit},
};
use unicode_segmentation::UnicodeSegmentation;

pub struct SentenceCountChecker {}

impl CheckerInit for SentenceCountChecker {
    fn new(_args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError> {
        Ok(Self {})
    }
}

impl Checker for SentenceCountChecker {
    fn check(&self, doc: &Document) -> CheckResult {
        CheckResult::Value(crate::Value::Integer(
            doc.content
                .unicode_sentences()
                .count()
                .try_into()
                .unwrap_or_default(),
        ))
    }
}
