use inquire::{CustomUserError, validator::Validation};
use std::path::PathBuf;

pub(super) fn data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join(crate::consts::DATA_DIR_NAME))
}

pub(super) fn rulesets_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join(crate::consts::DATA_DIR_NAME).join("rulesets"))
}

pub(super) fn groups_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join(crate::consts::DATA_DIR_NAME).join("groups"))
}

pub(super) fn name_validator(input: &str) -> Result<Validation, CustomUserError> {
    if input.chars().count() > 256 {
        Ok(Validation::Invalid(
            "Name must be 256 characters or less".into(),
        ))
    } else if input.is_empty() {
        Ok(Validation::Invalid("Name must not be empty".into()))
    } else if input
        .chars()
        .any(|char| !char.is_ascii_alphanumeric() && char != '-')
    {
        Ok(Validation::Invalid(
            "Name must only contain alphabets, numbers and hyphens".into(),
        ))
    } else {
        Ok(Validation::Valid)
    }
}
