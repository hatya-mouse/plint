use std::fmt::Display;

use crate::Condition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub message: String,
    #[serde(default)]
    pub severity: Severity,
    pub checker: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<yaml_serde::Mapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "advisory")]
    Advisory,
    #[serde(rename = "info")]
    #[default]
    Info,
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
            Self::Error => write!(f, "Error"),
            Self::Warning => write!(f, "Warning"),
            Self::Advisory => write!(f, "Advisory"),
            Self::Info => write!(f, "Info"),
        }
    }
}
