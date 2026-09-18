mod group;
mod index;

use crate::{
    ruleset::{group::Group, index::IndexFile},
    utils::data_dir,
};
use plint_linter::Ruleset;

#[derive(Debug)]
pub(super) enum RulesetLoadError {
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
pub(super) fn load_ruleset(ruleset_name: &str) -> Vec<Result<Ruleset, RulesetLoadError>> {
    match load_index_file() {
        Ok(index) => load_ruleset_with_index(ruleset_name, &index),
        Err(err) => vec![Err(err)],
    }
}

/// Loads the index file.
pub(super) fn load_index_file() -> Result<IndexFile, RulesetLoadError> {
    let Some(data_dir) = data_dir() else {
        return Err(RulesetLoadError::PathNotAvailable);
    };

    let index_path = data_dir.join("index").with_added_extension("yaml");
    if !index_path.exists() {
        return Ok(IndexFile::default());
    }

    match std::fs::read_to_string(index_path) {
        Ok(index_string) => match yaml_serde::from_str::<IndexFile>(&index_string) {
            Ok(index) => Ok(index),
            Err(err) => Err(RulesetLoadError::IndexParseError(err)),
        },
        Err(err) => Err(RulesetLoadError::IndexReadError(err)),
    }
}

fn load_ruleset_with_index(
    ruleset_name: &str,
    index: &IndexFile,
) -> Vec<Result<Ruleset, RulesetLoadError>> {
    if let Some(ruleset_path) = index.rulesets.get(ruleset_name) {
        // Load the ruleset if the ruleset name is found in the index
        match std::fs::read_to_string(ruleset_path) {
            Ok(ruleset_string) => vec![
                yaml_serde::from_str::<Ruleset>(&ruleset_string)
                    .map_err(RulesetLoadError::RulesetParseError),
            ],
            Err(err) => vec![Err(RulesetLoadError::RulesetReadError(err))],
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
                Err(err) => vec![Err(RulesetLoadError::GroupParseError(err))],
            },
            Err(err) => vec![Err(RulesetLoadError::GroupReadError(err))],
        }
    } else {
        vec![Err(RulesetLoadError::RulesetNotFound(
            ruleset_name.to_string(),
        ))]
    }
}
