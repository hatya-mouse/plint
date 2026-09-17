use crate::{
    Document,
    checker::{CheckResult, Checker, Match},
};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegexChecker {
    pattern: String,
}

impl Checker<'_> for RegexChecker {
    fn check(&self, doc: &Document) -> Result<CheckResult, Box<dyn std::error::Error>> {
        let re = Regex::new(&self.pattern)?;

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
