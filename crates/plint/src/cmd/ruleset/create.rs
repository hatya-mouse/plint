use crate::storage::{IndexFile, RulesetIo};
use plint_linter::Ruleset;

pub(crate) fn create(
    name: String,
    authors: Option<String>,
    description: Option<String>,
    version: Option<u64>,
) {
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

    // Create and save the ruleset
    let ruleset = Ruleset::new_empty(name.clone(), authors, description, version);
    match ruleset.save() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to create a ruleset: {:#?}", err);
            return;
        }
    }

    println!("Ruleset created successfully: {}", name);
}
