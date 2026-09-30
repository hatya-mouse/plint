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

use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub message: String,
    pub severity: Severity,
    pub code: String,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    #[serde(rename = "info")]
    #[default]
    Info,
    #[serde(rename = "advisory")]
    Advisory,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

impl Severity {
    pub fn all() -> Vec<Severity> {
        vec![
            Severity::Error,
            Severity::Warning,
            Severity::Advisory,
            Severity::Info,
        ]
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Advisory => write!(f, "advisory"),
            Self::Info => write!(f, "info"),
        }
    }
}
