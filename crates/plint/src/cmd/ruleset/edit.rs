use crate::{PlintIoError, storage::IndexFile};

pub(crate) fn edit(ruleset: &str) {
    match open_ruleset(ruleset) {
        Ok(_) => println!("Opened the ruleset '{}' in the default editor.", ruleset),
        Err(err) => eprintln!("Error opening ruleset: {:#?}", err),
    }
}

fn open_ruleset(ruleset: &str) -> Result<(), PlintIoError> {
    let ruleset_path = IndexFile::load()?.ruleset_path(ruleset)?;
    open::that(ruleset_path.as_os_str()).map_err(PlintIoError::IoError)
}
