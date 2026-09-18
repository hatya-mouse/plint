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
