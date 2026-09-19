use crate::{PlintIoError, utils::data_dir};
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
            .ok_or_else(|| PlintIoError::NotFound(group_name.to_string()))
    }

    /// Returns whether the name is already registered as either a ruleset or a group.
    pub(crate) fn is_name_registered(&self, name: &str) -> bool {
        self.rulesets.contains_key(name) || self.groups.contains_key(name)
    }

    // --- INDEX FILE STORAGE OPERATION ---

    /// Loads the index file.
    pub(crate) fn load() -> Result<IndexFile, PlintIoError> {
        let Some(data_dir) = data_dir() else {
            return Err(PlintIoError::PathNotAvailable);
        };

        let index_path = data_dir.join("index").with_added_extension("yaml");
        if !index_path.exists() {
            return Ok(IndexFile::default());
        }

        match std::fs::read_to_string(index_path) {
            Ok(index_string) => match yaml_serde::from_str::<IndexFile>(&index_string) {
                Ok(index) => Ok(index),
                Err(err) => Err(PlintIoError::YamlParseError(err)),
            },
            Err(err) => Err(PlintIoError::IoError(err)),
        }
    }

    /// Writes the given index file to the data directory.
    pub(crate) fn save(&self) -> Result<(), PlintIoError> {
        match yaml_serde::to_string(self) {
            Ok(index_string) => {
                let Some(data_dir) = data_dir() else {
                    return Err(PlintIoError::PathNotAvailable);
                };
                let index_path = data_dir.join("index").with_added_extension("yaml");
                data_dir.parent().map(std::fs::create_dir_all);

                match std::fs::write(index_path, index_string) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(PlintIoError::IoError(err)),
                }
            }
            Err(err) => Err(PlintIoError::YamlParseError(err)),
        }
    }
}
