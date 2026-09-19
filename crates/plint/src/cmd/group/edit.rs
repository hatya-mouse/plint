use crate::{PlintIoError, storage::IndexFile};

pub(crate) fn edit(group: &str) {
    match open_group(group) {
        Ok(_) => println!("Opened the group '{}' in the default editor.", group),
        Err(err) => eprintln!("Error opening group: {:#?}", err),
    }
}

fn open_group(group: &str) -> Result<(), PlintIoError> {
    let group_path = IndexFile::load()?.group_path(group)?;
    open::that(group_path.as_os_str()).map_err(PlintIoError::IoError)
}
