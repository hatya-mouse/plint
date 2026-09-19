use crate::storage::{Group, IndexFile};

pub(crate) fn create(name: String) {
    let index_file = match IndexFile::load() {
        Ok(index_file) => index_file,
        Err(err) => {
            eprintln!("Failed to load index file: {:#?}", err);
            return;
        }
    };

    if index_file.is_name_registered(&name) {
        eprintln!("Ruleset name already exists: {}", name);
        return;
    }

    // Create and save the group
    let group = Group::new(name.clone());
    match group.save() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to create a group: {:#?}", err);
            return;
        }
    }

    println!("Group created successfully: {}", name);
}
