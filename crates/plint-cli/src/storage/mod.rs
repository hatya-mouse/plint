mod group;
mod index;

use crate::{
    storage::{group::Group, index::IndexFile},
    utils::data_dir,
};
use plint_linter::Ruleset;

#[derive(Debug)]
pub(super) enum PlintIoError {
    PathNotAvailable,
    RulesetNotFound(String),
    RulesetParseError(yaml_serde::Error),
    RulesetReadError(std::io::Error),
    GroupParseError(yaml_serde::Error),
    GroupReadError(std::io::Error),
    IndexParseError(yaml_serde::Error),
    IndexReadError(std::io::Error),
}

/// Loads the ruleset with the given name from the data directory.
pub(super) fn load_ruleset(ruleset_name: &str) -> Vec<Result<Ruleset, PlintIoError>> {
    match load_index_file() {
        Ok(index) => load_ruleset_with_index(ruleset_name, &index),
        Err(err) => vec![Err(err)],
    }
}

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
            Err(err) => Err(PlintIoError::IndexParseError(err)),
        },
        Err(err) => Err(PlintIoError::IndexReadError(err)),
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
                Err(err) => Err(PlintIoError::IndexReadError(err)),
            }
        }
        Err(err) => Err(PlintIoError::IndexParseError(err)),
    }
}

fn load_ruleset_with_index(
    ruleset_name: &str,
    index: &IndexFile,
) -> Vec<Result<Ruleset, PlintIoError>> {
    if let Some(ruleset_path) = index.rulesets.get(ruleset_name) {
        // Load the ruleset if the ruleset name is found in the index
        match std::fs::read_to_string(ruleset_path) {
            Ok(ruleset_string) => vec![
                yaml_serde::from_str::<Ruleset>(&ruleset_string)
                    .map_err(PlintIoError::RulesetParseError),
            ],
            Err(err) => vec![Err(PlintIoError::RulesetReadError(err))],
        }
    } else if let Some(group_path) = index.groups.get(ruleset_name) {
        // Load the every single rulesets in the group recursively
        match std::fs::read_to_string(group_path) {
            Ok(group_string) => match yaml_serde::from_str::<Group>(&group_string) {
                Ok(group) => group
                    .rulesets
                    .iter()
                    .flat_map(|ruleset_name| load_ruleset_with_index(ruleset_name, index))
                    .collect(),
                Err(err) => vec![Err(PlintIoError::GroupParseError(err))],
            },
            Err(err) => vec![Err(PlintIoError::GroupReadError(err))],
        }
    } else {
        vec![Err(PlintIoError::RulesetNotFound(ruleset_name.to_string()))]
    }
}
