mod condition;

pub use condition::Condition;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruleset {
    pub name: String,
    pub version: u32,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub check: String,
    pub severity: Severity,
    pub message: String,
    #[serde(default)]
    pub args: yaml_serde::Value,
    pub condition: Option<Condition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
}
