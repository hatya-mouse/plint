//
//  Copyright 2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

mod lint;
mod rule;

pub use lint::{LintEntry, LintResult};
pub use rule::{Rule, Severity};

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
