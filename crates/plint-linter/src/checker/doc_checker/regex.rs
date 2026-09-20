use crate::{
    Document, LinterError,
    checker::{CheckResult, CheckResultType, Checker, CheckerInit, Match},
};
use regex::Regex;

pub struct RegexChecker {
    regex: Regex,
}

impl CheckerInit for RegexChecker {
    fn new(args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError> {
        let Some(args) = args else {
            return Err(LinterError::MissingArgs);
        };

        let pattern: String = args
            .get("pattern")
            .ok_or_else(|| LinterError::MissingArg {
                arg: "pattern".to_string(),
            })?
            .as_str()
            .ok_or_else(|| LinterError::InvalidArg {
                arg: "pattern".to_string(),
                reason: None,
            })?
            .into();
        let regex = Regex::new(&pattern).map_err(|e| LinterError::InvalidArg {
            arg: "pattern".to_string(),
            reason: Some(format!("Invalid regex pattern: {}", e)),
        })?;

        Ok(Self { regex })
    }
}

impl Checker for RegexChecker {
    fn check(&self, doc: &Document) -> CheckResult {
        let matches = self
            .regex
            .find_iter(&doc.content)
            .map(|m| Match { range: m.range() })
            .collect();
        CheckResult::Matches(matches)
    }

    fn check_type(&self) -> CheckResultType {
        CheckResultType::Matches
    }
}
