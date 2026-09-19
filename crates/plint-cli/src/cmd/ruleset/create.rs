use crate::{storage::create_ruleset, utils::data_dir};
use plint_linter::Ruleset;

pub(crate) fn create(
    name: String,
    authors: Option<String>,
    description: Option<String>,
    version: Option<u64>,
) {
    let ruleset = Ruleset::new_empty(name.clone(), authors, description, version);
    let dest_path = match data_dir().map(|path| {
        path.join("rulesets")
            .join(&name)
            .with_added_extension("yaml")
    }) {
        Some(dest_path) => dest_path,
        None => {
            eprintln!("Failed to get data directory");
            return;
        }
    };

    // Create directories is they don't exist
    dest_path.parent().map(std::fs::create_dir_all);

    // Write the ruleset
    match create_ruleset(&ruleset) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to create a ruleset: {:#?}", err);
            return;
        }
    }

    println!("Ruleset created successfully: {}", name);
}
