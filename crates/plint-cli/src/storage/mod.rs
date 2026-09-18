mod group;
mod index;

use crate::{
    storage::{group::Group, index::IndexFile},
    utils::{data_dir, rulesets_dir},
};
use plint_linter::Ruleset;

#[derive(Debug)]
pub(super) enum PlintIoError {
    PathNotAvailable,
    DuplicateName(String),
    NotFound(String),
    YamlParseError(yaml_serde::Error),
    IoError(std::io::Error),
}

/// Loads the ruleset with the given name from the data directory.
pub(super) fn load_ruleset(ruleset_name: &str) -> Result<Ruleset, PlintIoError> {
    // Get the ruleset path from the index file
    let ruleset_path = load_index_file()?.ruleset_path(ruleset_name)?;

    // Read the ruleset file
    let ruleset_string = std::fs::read_to_string(&ruleset_path).map_err(PlintIoError::IoError)?;
    yaml_serde::from_str::<Ruleset>(&ruleset_string).map_err(PlintIoError::YamlParseError)
}

/// Loads the group with the given name from the data directory.
pub(super) fn load_group(group_name: &str) -> Result<Group, PlintIoError> {
    // Get the group path from the index file
    let group_path = load_index_file()?.group_path(group_name)?;

    // Read the group file
    let group_string = std::fs::read_to_string(&group_path).map_err(PlintIoError::IoError)?;
    yaml_serde::from_str::<Group>(&group_string).map_err(PlintIoError::YamlParseError)
}

// --- RULESET FILE OPERATION ---

/// Creates a new ruleset file with the given name and adds it to the index file.
pub(super) fn create_ruleset(ruleset: &Ruleset) -> Result<(), PlintIoError> {
    let mut index_file = load_index_file()?;

    if index_file.rulesets.contains_key(&ruleset.name)
        || index_file.groups.contains_key(&ruleset.name)
    {
        return Err(PlintIoError::DuplicateName(ruleset.name.to_string()));
    }

    let Some(ruleset_path) =
        rulesets_dir().map(|path| path.join(&ruleset.name).with_added_extension("yaml"))
    else {
        return Err(PlintIoError::PathNotAvailable);
    };

    // Register the path to the newly created ruleset file in the index file
    index_file
        .rulesets
        .insert(ruleset.name.to_string(), ruleset_path.clone());
    write_index_file(&index_file)?;

    let result = match yaml_serde::to_string(ruleset) {
        Ok(ruleset_string) => match std::fs::write(ruleset_path, ruleset_string) {
            Ok(_) => Ok(()),
            Err(err) => Err(PlintIoError::IoError(err)),
        },
        Err(err) => Err(PlintIoError::YamlParseError(err)),
    };

    // In case of an error, remove the entry from the index file
    if result.is_err() {
        index_file.rulesets.remove(&ruleset.name);
        write_index_file(&index_file)?;
    }

    result
}

/// Saves the ruleset with the given name.
pub(super) fn save_ruleset(ruleset: &Ruleset) -> Result<(), PlintIoError> {
    // Get the path to the ruleset file from the index file
    let ruleset_path = load_index_file()?.ruleset_path(&ruleset.name)?;

    // Write the ruleset to the file
    match yaml_serde::to_string(ruleset) {
        Ok(ruleset_string) => match std::fs::write(ruleset_path, ruleset_string) {
            Ok(_) => Ok(()),
            Err(err) => Err(PlintIoError::IoError(err)),
        },
        Err(err) => Err(PlintIoError::YamlParseError(err)),
    }
}

/// Removes the ruleset with the given name.
pub(super) fn remove_ruleset(ruleset_name: &str) -> Result<(), PlintIoError> {
    let mut index_file = load_index_file()?;

    // Remove the ruleset entry from the index file
    let ruleset_path = index_file
        .rulesets
        .remove(ruleset_name)
        .ok_or_else(|| PlintIoError::NotFound(ruleset_name.to_string()))?;

    // Remove the ruleset file from the filesystem
    match std::fs::remove_file(&ruleset_path) {
        Ok(_) => (),
        Err(err) => {
            index_file
                .rulesets
                .insert(ruleset_name.to_string(), ruleset_path);
            return Err(PlintIoError::IoError(err));
        }
    }

    write_index_file(&index_file)
}

// --- INDEX FILE OPERATION ---

/// Loads the index file.
pub(super) fn load_index_file() -> Result<IndexFile, PlintIoError> {
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
pub(super) fn write_index_file(index_file: &IndexFile) -> Result<(), PlintIoError> {
    match yaml_serde::to_string(index_file) {
        Ok(index_string) => {
            let Some(data_dir) = data_dir() else {
                return Err(PlintIoError::PathNotAvailable);
            };
            let index_path = data_dir.join("index").with_added_extension("yaml");

            match std::fs::write(index_path, index_string) {
                Ok(_) => Ok(()),
                Err(err) => Err(PlintIoError::IoError(err)),
            }
        }
        Err(err) => Err(PlintIoError::YamlParseError(err)),
    }
}
