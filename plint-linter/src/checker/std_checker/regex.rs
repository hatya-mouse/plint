use crate::{
    Document, LinterError,
    checker::{CheckResult, Checker, Match},
};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RegexArgs {
    pub pattern: String,
}

pub struct RegexChecker {
    regex: Regex,
}

impl Checker for RegexChecker {
    fn new(args: &Option<yaml_serde::Mapping>) -> Result<Self, LinterError> {
        let Some(args) = args else {
            return Err(LinterError::MissingArgs);
        };

        let parsed_args = RegexArgs {
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

    fn check(&self, doc: &Document) -> CheckResult {
        if let Some(caps) = self.regex.captures(&doc.content) {
            let matches = (1..caps.len())
                .into_iter()
                .filter_map(|i| {
                    caps.get(i).map(|caps| Match {
                        range: caps.range(),
                    })
                })
                .collect();
            CheckResult::Matches(matches)
        } else {
            CheckResult::Matches(Vec::new())
        }
    }
}
