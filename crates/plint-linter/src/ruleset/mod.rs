mod lint;
mod rule;

pub use lint::{LintEntry, LintResult};
pub use rule::Rule;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Ruleset {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
    pub rules: Vec<Rule>,
}

impl Ruleset {
    /// Creates a new empty ruleset with given name, authors, description, and version.
    pub fn new_empty(
        name: String,
        authors: Option<String>,
        description: Option<String>,
        version: Option<u64>,
    ) -> Self {
        Self {
            name,
            authors,
            description,
            version,
            rules: Vec::new(),
        }
    }

    pub fn from_yaml(yaml_str: &str) -> yaml_serde::Result<Self> {
        yaml_serde::from_str(yaml_str)
    }
}
