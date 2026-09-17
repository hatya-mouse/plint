use crate::{
    Document,
    checker::{CheckResult, Checker, Match},
};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RegexArgs {
    pub pattern: String,
}

pub struct RegexChecker {
    args: RegexArgs,
}

impl Checker for RegexChecker {
    fn new(args: yaml_serde::Value) -> Result<Self, Box<dyn std::error::Error>> {
        let args: RegexArgs = yaml_serde::from_value(args)?;
        Ok(Self { args })
    }

    fn check(&self, doc: &Document) -> Result<CheckResult, Box<dyn std::error::Error>> {
        let re = Regex::new(&self.args.pattern)?;

        if let Some(caps) = re.captures(&doc.content) {
            let matches = (1..caps.len())
                .into_iter()
                .filter_map(|i| {
                    caps.get(i).map(|caps| Match {
                        range: caps.range(),
                    })
                })
                .collect();
            Ok(CheckResult::Matches(matches))
        } else {
            Ok(CheckResult::Matches(Vec::new()))
        }
    }
}
