use crate::{
    storage::{load_index_file, write_index_file},
    utils::data_dir,
};
use plint_linter::Ruleset;

pub(crate) fn create(name: &str) {
    let ruleset = Ruleset::default();
    let dest_path = match data_dir().map(|path| {
        path.join("rulesets")
            .join(name)
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
    match load_index_file() {
        Ok(mut index_file) => {
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
        Err(err) => {
            eprintln!("Failed to create a ruleset: {:#?}", err);
            // Clean up the created ruleset file if loading the index file fails
            std::fs::remove_file(dest_path).ok();
        }
    };
}
