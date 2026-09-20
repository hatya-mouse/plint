use crate::{
    Document, LinterError,
    checker::{CheckResult, CheckResultType, Checker, CheckerInit},
};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RegexCountArgs {
    pub pattern: String,
}

pub struct RegexCountChecker {
    regex: Regex,
}

impl CheckerInit for RegexCountChecker {
    fn new(args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError> {
        let Some(args) = args else {
            return Err(LinterError::MissingArgs);
        };

        let parsed_args = RegexCountArgs {
            pattern: args
                .get("pattern")
                .ok_or_else(|| LinterError::MissingArg {
                    arg: "pattern".to_string(),
                })?
                .as_str()
                .ok_or_else(|| LinterError::InvalidArg {
                    arg: "pattern".to_string(),
                    reason: None,
                })?
                .into(),
        };
        let regex = Regex::new(&parsed_args.pattern).map_err(|e| LinterError::InvalidArg {
            arg: "pattern".to_string(),
            reason: Some(format!("Invalid regex pattern: {}", e)),
        })?;

        Ok(Self { regex })
    }
}

impl Checker for RegexCountChecker {
    fn check(&self, doc: &Document) -> CheckResult {
        let matches: Vec<_> = self.regex.find_iter(&doc.content).collect();
        CheckResult::Value(crate::Value::Integer(
            matches.len().try_into().unwrap_or_default(),
        ))
    }

    fn check_type(&self) -> CheckResultType {
        CheckResultType::Value
    }
}
