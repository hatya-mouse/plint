use crate::{PlintIoError, storage::IndexFile, utils::groups_dir};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Group {
    pub name: String,
    pub rulesets: Vec<String>,
}

impl Group {
    /// Creates a new group with the given name.
    pub(crate) fn new(name: String) -> Self {
        Group {
            name,
            rulesets: Vec::new(),
        }
    }

    /// Loads the group with the given name from the data directory.
    pub(crate) fn load(group_name: &str) -> Result<Group, PlintIoError> {
        // Get the group path from the index file
        let group_path = IndexFile::load()?.group_path(group_name)?;

        // Read the group file
        let group_string = std::fs::read_to_string(&group_path).map_err(PlintIoError::IoError)?;
        yaml_serde::from_str::<Group>(&group_string).map_err(PlintIoError::YamlParseError)
    }

    /// Saves the group to the storage, adding it to the index file if it doesn't already exist.
    /// This overwrites the existing group file if it already exists.
    pub(crate) fn save(&self) -> Result<(), PlintIoError> {
        let mut index_file = IndexFile::load()?;

        let Some(group_path) =
            groups_dir().map(|path| path.join(&self.name).with_added_extension("yaml"))
        else {
            return Err(PlintIoError::PathNotAvailable);
        };
        group_path.parent().map(std::fs::create_dir_all);

        // Register the path to the newly created ruleset file in the index file
        let original_entry = index_file
            .groups
            .insert(self.name.clone(), group_path.clone());
        IndexFile::save(&index_file)?;

        let result = match yaml_serde::to_string(self) {
            Ok(group_string) => match std::fs::write(group_path, group_string) {
                Ok(_) => Ok(()),
                Err(err) => Err(PlintIoError::IoError(err)),
            },
            Err(err) => Err(PlintIoError::YamlParseError(err)),
        };

        // In case of an error, revert the entry from the index file
        if result.is_err() {
            if let Some(original_entry) = original_entry {
                index_file.groups.insert(self.name.clone(), original_entry);
            } else {
                index_file.groups.remove(&self.name);
            }
            IndexFile::save(&index_file)?;
        }

        result
    }

    /// Removes the group with the name.
    fn remove(name: &str) -> Result<(), PlintIoError> {
        let mut index_file = IndexFile::load()?;

        // Remove the ruleset entry from the index file
        let group_path = index_file
            .groups
            .remove(name)
            .ok_or_else(|| PlintIoError::NotFound(name.to_string()))?;

        // Remove the ruleset file from the filesystem
        match std::fs::remove_file(&group_path) {
            Ok(_) => (),
            Err(err) => {
                index_file.groups.insert(name.to_string(), group_path);
                return Err(PlintIoError::IoError(err));
            }
        }

        IndexFile::save(&index_file)
    }
}
