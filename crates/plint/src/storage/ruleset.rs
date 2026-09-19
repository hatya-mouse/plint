use crate::{PlintIoError, storage::IndexFile, utils::rulesets_dir};
use plint_linter::Ruleset;

pub(crate) trait RulesetIo {
    /// Loads the ruleset with the given name from the data directory.
    fn load(name: &str) -> Result<Ruleset, PlintIoError>;

    /// Saves the ruleset to the storage, adding it to the index file if it doesn't already exist.
    /// This overwrites the existing ruleset file if it already exists.
    fn save(&self) -> Result<(), PlintIoError>;

    /// Removes the ruleset with the name.
    fn remove(name: &str) -> Result<(), PlintIoError>;
}

impl RulesetIo for Ruleset {
    fn load(name: &str) -> Result<Ruleset, PlintIoError> {
        // Get the ruleset path from the index file
        let ruleset_path = IndexFile::load()?.ruleset_path(name)?;

        // Read the ruleset file
        let ruleset_string =
            std::fs::read_to_string(&ruleset_path).map_err(PlintIoError::IoError)?;
        yaml_serde::from_str::<Ruleset>(&ruleset_string).map_err(PlintIoError::YamlParseError)
    }

    fn save(&self) -> Result<(), PlintIoError> {
        let mut index_file = IndexFile::load()?;

        let Some(ruleset_path) =
            rulesets_dir().map(|path| path.join(&self.name).with_added_extension("yaml"))
        else {
            return Err(PlintIoError::PathNotAvailable);
        };
        ruleset_path.parent().map(std::fs::create_dir_all);

        // Register the path to the newly created ruleset file in the index file
        let original_entry = index_file
            .rulesets
            .insert(self.name.clone(), ruleset_path.clone());
        IndexFile::save(&index_file)?;

        let result = match yaml_serde::to_string(self) {
            Ok(ruleset_string) => match std::fs::write(ruleset_path, ruleset_string) {
                Ok(_) => Ok(()),
                Err(err) => Err(PlintIoError::IoError(err)),
            },
            Err(err) => Err(PlintIoError::YamlParseError(err)),
        };

        // In case of an error, revert the entry from the index file
        if result.is_err() {
            if let Some(original_entry) = original_entry {
                index_file
                    .rulesets
                    .insert(self.name.clone(), original_entry);
            } else {
                index_file.rulesets.remove(&self.name);
            }
            IndexFile::save(&index_file)?;
        }

        result
    }

    fn remove(name: &str) -> Result<(), PlintIoError> {
        let mut index_file = IndexFile::load()?;

        // Remove the ruleset entry from the index file
        let ruleset_path = index_file
            .rulesets
            .remove(name)
            .ok_or_else(|| PlintIoError::NotFound(name.to_string()))?;

        // Remove the ruleset file from the filesystem
        match std::fs::remove_file(&ruleset_path) {
            Ok(_) => (),
            Err(err) => {
                index_file.rulesets.insert(name.to_string(), ruleset_path);
                return Err(PlintIoError::IoError(err));
            }
        }

        IndexFile::save(&index_file)
    }
}
