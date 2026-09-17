mod lint;
mod rule;

pub use lint::{LintEntry, LintResult};
pub use rule::Rule;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruleset {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: u64,
    pub rules: Vec<Rule>,
}

impl Ruleset {
    pub fn from_yaml(yaml_str: &str) -> yaml_serde::Result<Self> {
        yaml_serde::from_str(yaml_str)
    }
}
