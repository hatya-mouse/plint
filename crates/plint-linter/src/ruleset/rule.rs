use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub message: String,
    pub severity: Severity,
    pub code: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Advisory => write!(f, "advisory"),
            Self::Info => write!(f, "info"),
        }
    }
}
