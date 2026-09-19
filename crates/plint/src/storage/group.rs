use crate::{PlintIoError, storage::IndexFile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Group {
    pub name: String,
    pub rulesets: Vec<String>,
}

impl Group {
    /// Loads the group with the given name from the data directory.
    pub(crate) fn load(group_name: &str) -> Result<Group, PlintIoError> {
        // Get the group path from the index file
        let group_path = IndexFile::load()?.group_path(group_name)?;

        // Read the group file
        let group_string = std::fs::read_to_string(&group_path).map_err(PlintIoError::IoError)?;
        yaml_serde::from_str::<Group>(&group_string).map_err(PlintIoError::YamlParseError)
    }
}
