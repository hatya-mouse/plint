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

impl Default for Ruleset {
    fn default() -> Self {
        Self {
            name: String::default(),
            description: String::default(),
            version: 1,
            rules: Vec::new(),
        }
    }
}

impl Ruleset {
    pub fn new_named(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            version: 1,
            rules: Vec::new(),
        }
    }

    pub fn from_yaml(yaml_str: &str) -> yaml_serde::Result<Self> {
        yaml_serde::from_str(yaml_str)
    }
}
