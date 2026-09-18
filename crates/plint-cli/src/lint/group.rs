use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Group {
    pub name: String,
    pub rulesets: Vec<String>,
}
