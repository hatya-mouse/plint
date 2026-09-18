use crate::{
    storage::{create_ruleset, load_index_file},
    tui::{name_validator, version_validator},
    utils::data_dir,
};
use inquire::validator::{MaxLengthValidator, Validation};
use plint_linter::Ruleset;

pub(crate) fn create(
    mut name: Option<String>,
    mut authors: Option<String>,
    mut description: Option<String>,
    mut version: Option<u64>,
) {
    // Load the index file
    let index_file = match load_index_file() {
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
            .with_validators(&[
                Box::new(name_validator),
                Box::new(move |input: &str| {
                    if index_file.rulesets.contains_key(input) {
                        Ok(Validation::Invalid(
                            "A ruleset with this name already exists".into(),
                        ))
                    } else {
                        Ok(Validation::Valid)
                    }
                }),
            ])
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
