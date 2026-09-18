use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct IndexFile {
    pub rulesets: HashMap<String, PathBuf>,
    pub groups: HashMap<String, PathBuf>,
}
