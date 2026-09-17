use crate::Condition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
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
