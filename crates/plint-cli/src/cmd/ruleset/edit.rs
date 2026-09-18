use crate::{
    storage::{load_index_file, load_ruleset, save_ruleset},
    utils::{name_validator, version_validator},
};
use inquire::validator::MaxLengthValidator;
use plint_linter::Ruleset;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub(crate) fn edit(mut ruleset_name: Option<String>) {
    // Load the index file
    let mut index_file = match load_index_file() {
        Ok(index_file) => index_file,
        Err(err) => {
            eprintln!("Failed to load the index file: {:#?}", err);
            return;
        }
    };

    // Ask the user for the ruleset name if not provided
    if ruleset_name.is_none() {
        if index_file.rulesets.is_empty() {
            println!("No rulesets installed.");
            return;
        }

        println!("Please select the ruleset to edit:");

        let ruleset_options = index_file.rulesets.keys().collect();
        ruleset_name = inquire::Select::new("Name", ruleset_options)
            .prompt()
            .ok()
            .cloned();
    }

    // Ensure that the ruleset is not None
    let Some(ruleset_name) = ruleset_name else {
        eprintln!("Failed to get the ruleset name");
        return;
    };

    // Get the ruleset from the index file
    let rulesets = load_ruleset(&ruleset_name);
    let mut ruleset = if rulesets.len() == 1 {
        match rulesets.into_iter().next().unwrap() {
            Ok(ruleset) => ruleset,
            Err(err) => {
                eprintln!("Failed to load the ruleset: {:#?}", err);
                return;
            }
        }
    } else {
        eprintln!("'{}' is a ruleset group", ruleset_name);
        return;
    };

    edit_loop(&ruleset_name, &mut ruleset);
}

fn edit_loop(ruleset_name: &str, ruleset: &mut Ruleset) {
    loop {
        let Ok(action) =
            inquire::Select::new("Select an action", EditAction::iter().collect()).prompt()
        else {
            eprintln!("Failed to get the action");
            return;
        };

        match action {
            EditAction::EditName => {
                match inquire::Text::new("Name")
                    .with_validator(name_validator)
                    .prompt()
                {
                    Ok(name) => ruleset.name = name,
                    Err(err) => eprintln!("Failed to get the ruleset name: {}", err),
                }
            }
            EditAction::EditAuthors => {
                match inquire::Text::new("Authors")
                    .with_validator(
                        MaxLengthValidator::new(256)
                            .with_message("Authors must be 256 characters or less"),
                    )
                    .prompt()
                {
                    Ok(authors) => {
                        ruleset.authors = if authors.is_empty() {
                            None
                        } else {
                            Some(authors)
                        }
                    }
                    Err(err) => eprintln!("Failed to get the ruleset authors: {}", err),
                }
            }
            EditAction::EditDescription => {
                match inquire::Text::new("Description")
                    .with_validator(
                        MaxLengthValidator::new(4096)
                            .with_message("Description must be 4096 characters or less"),
                    )
                    .prompt()
                {
                    Ok(description) => {
                        ruleset.description = if description.is_empty() {
                            None
                        } else {
                            Some(description)
                        }
                    }
                    Err(err) => eprintln!("Failed to get the ruleset name: {}", err),
                }
            }
            EditAction::EditVersion => {
                match inquire::Text::new("Version")
                    .with_validator(version_validator)
                    .prompt()
                {
                    Ok(version) => {
                        ruleset.version = if version.trim().is_empty() {
                            None
                        } else {
                            version.trim().parse::<u64>().ok()
                        };
                    }
                    Err(err) => eprintln!("Failed to get the ruleset name: {}", err),
                }
            }
            EditAction::EditRules => rules_edit_loop(),
            EditAction::SaveAndExit => {
                ruleset.modified();
                if let Err(err) = save_ruleset(&ruleset_name, &ruleset) {
                    eprintln!("Failed to save the ruleset: {:#?}", err);
                } else {
                    println!("Ruleset saved successfully.");
                }
                break;
            }
            EditAction::ExitWithoutSaving => {
                println!("Exiting without saving.");
                break;
            }
        }
    }
}

fn rules_edit_loop() {
    loop {
        let Ok(rule_action) =
            inquire::Select::new("Select an action", EditRuleAction::iter().collect()).prompt()
        else {
            eprintln!("Failed to get the action");
            return;
        };

        match rule_action {
            EditRuleAction::Back => break,
            EditRuleAction::AddRule => {}
        }
    }
}

#[derive(EnumIter, strum_macros::Display)]
enum EditAction {
    #[strum(serialize = "Edit Name")]
    EditName,
    #[strum(serialize = "Edit Authors")]
    EditAuthors,
    #[strum(serialize = "Edit Description")]
    EditDescription,
    #[strum(serialize = "Edit Version")]
    EditVersion,
    #[strum(serialize = "Edit Rules")]
    EditRules,
    #[strum(serialize = "Save and Exit")]
    SaveAndExit,
    #[strum(serialize = "Exit Without Saving")]
    ExitWithoutSaving,
}

#[derive(EnumIter, strum_macros::Display)]
enum EditRuleAction {
    #[strum(serialize = "Back")]
    Back,
    #[strum(serialize = "Add Rule")]
    AddRule,
    #[strum(serialize = "Remove Rule")]
    RemoveRule,
    #[strum(serialize = "Select Rule to Edit")]
    SelectRule,
}
