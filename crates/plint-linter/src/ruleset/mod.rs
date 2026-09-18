mod lint;
mod rule;

pub use lint::{LintEntry, LintResult};
pub use rule::Rule;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Ruleset {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
    pub rules: Vec<Rule>,
}

impl Ruleset {
    pub fn new_named(name: &str) -> Self {
        Self {
            name: name.to_string(),
            author: None,
            description: None,
            version: None,
            rules: Vec::new(),
        }
    }

    pub fn from_yaml(yaml_str: &str) -> yaml_serde::Result<Self> {
        yaml_serde::from_str(yaml_str)
    }
}
