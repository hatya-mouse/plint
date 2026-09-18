mod lint;
mod rule;

pub use lint::{LintEntry, LintResult};
pub use rule::Rule;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruleset {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
    #[serde(default)]
    pub creation_time: DateTime<Utc>,
    #[serde(default)]
    pub modified_time: DateTime<Utc>,
    pub rules: Vec<Rule>,
}

impl Default for Ruleset {
    fn default() -> Self {
        Ruleset {
            name: String::new(),
            authors: None,
            description: None,
            version: None,
            creation_time: Utc::now(),
            modified_time: Utc::now(),
            rules: Vec::new(),
        }
    }
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
            creation_time: Utc::now(),
            modified_time: Utc::now(),
            rules: Vec::new(),
        }
    }

    pub fn from_yaml(yaml_str: &str) -> yaml_serde::Result<Self> {
        yaml_serde::from_str(yaml_str)
    }

    /// Updates the modified time of the ruleset to the current time.
    pub fn modified(&mut self) {
        self.modified_time = Utc::now();
    }
}
