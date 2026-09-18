use crate::{
    cli::{name_validator, version_validator},
    storage::{load_index_file, write_index_file},
    utils::data_dir,
};
use inquire::validator::MaxLengthValidator;
use plint_linter::Ruleset;

pub(crate) fn create(
    mut name: Option<String>,
    mut authors: Option<String>,
    mut description: Option<String>,
    mut version: Option<u64>,
) {
    // Load the index file
    let mut index_file = match load_index_file() {
        Ok(index_file) => index_file,
        Err(err) => {
            eprintln!("Failed to load the index file: {:#?}", err);
            return;
        }
    };

    // Ask the user for the information if not provided
    if name.is_none() {
        println!("Please enter the information for the new ruleset:");

        name = inquire::Text::new("Name")
            .with_validator(name_validator)
            .prompt()
            .ok();
        authors = inquire::Text::new("Authors")
            .with_validator(
                MaxLengthValidator::new(256).with_message("Authors must be 256 characters or less"),
            )
            .with_initial_value(&authors.unwrap_or_default())
            .prompt()
            .ok();
        description = inquire::Text::new("Description")
            .with_validator(
                MaxLengthValidator::new(4096)
                    .with_message("Description must be 4096 characters or less"),
            )
            .with_initial_value(&description.unwrap_or_default())
            .prompt()
            .ok();
        version = inquire::Text::new("Version")
            .with_validator(version_validator)
            .prompt()
            .ok()
            .filter(|input| !input.trim().is_empty())
            .and_then(|input| input.trim().parse::<u64>().ok());
    }

    // Ensure that the name is not None
    let name = match name {
        Some(name) => name,
        None => {
            eprintln!("Failed to get the ruleset name");
            return;
        }
    };

    // Return if the ruleset already exists in the index file
    if index_file.rulesets.contains_key(&name) {
        eprintln!("Ruleset already exists: {}", name);
        return;
    }

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

    // Write the ruleset to the destination path
    match yaml_serde::to_string(&ruleset) {
        Ok(ruleset_string) => match std::fs::write(&dest_path, ruleset_string) {
            Ok(_) => {
                println!("Created new ruleset: {}", name);
            }
            Err(err) => {
                eprintln!("Failed to create a ruleset: {}", err);
                return;
            }
        },
        Err(err) => {
            eprintln!("Failed to create a ruleset: {}", err);
            return;
        }
    }

    // Add the ruleset to the index file
    index_file
        .rulesets
        .insert(name.to_string(), dest_path.clone());
    match write_index_file(&index_file) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to create a ruleset: {:#?}", err);
            // Clean up the created ruleset file if writing to the index file fails
            std::fs::remove_file(&dest_path).ok();
        }
    }
}
