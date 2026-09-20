use crate::{
    Document, LinterError,
    checker::{CheckResult, CheckResultType, Checker, CheckerInit},
};

pub struct ContainsChecker {
    text: String,
}

impl CheckerInit for ContainsChecker {
    fn new(args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError> {
        let Some(args) = args else {
            return Err(LinterError::MissingArgs);
        };

        let text = args
            .get("text")
            .ok_or_else(|| LinterError::MissingArg {
                arg: "text".to_string(),
            })?
            .as_str()
            .ok_or_else(|| LinterError::InvalidArg {
                arg: "text".to_string(),
                reason: None,
            })?
            .into();

        Ok(Self { text })
    }
}

impl Checker for ContainsChecker {
    fn check(&self, doc: &Document) -> CheckResult {
        CheckResult::Value(crate::Value::Boolean(doc.content.contains(&self.text)))
    }

    fn check_type(&self) -> CheckResultType {
        CheckResultType::Value
    }
}
