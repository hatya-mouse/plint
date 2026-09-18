use crate::PlintIoError;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct IndexFile {
    pub rulesets: HashMap<String, PathBuf>,
    pub groups: HashMap<String, PathBuf>,
}

impl IndexFile {
    /// Returns the path to the ruleset file with the given name.
    pub(crate) fn ruleset_path(&self, ruleset_name: &str) -> Result<PathBuf, PlintIoError> {
        // Get the path to the ruleset file from the index file
        self.rulesets
            .get(ruleset_name)
            .cloned()
            .ok_or_else(|| PlintIoError::NotFound(ruleset_name.to_string()))
    }

    /// Returns the path to the group file with the given name.
    pub(crate) fn group_path(&self, group_name: &str) -> Result<PathBuf, PlintIoError> {
        // Get the path to the group file from the index file
        self.groups
            .get(group_name)
            .cloned()
            .ok_or_else(|| PlintIoError::NotFound(ruleset_name.to_string()))
    }
}
