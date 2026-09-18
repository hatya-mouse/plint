use crate::{
    storage::{load_index_file, load_ruleset, save_ruleset}, tui::{name_validator, version_validator}, utils::rulesets_dir,
};
use inquire::{error::InquireResult, validator::MaxLengthValidator};
use plint_linter::{Rule, Ruleset, checker::ALL_CHECKERS, ruleset::Severity};
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub(crate) fn edit(mut ruleset_name: Option<String>) {
    // Load the index file
    let index_file = match load_index_file() {
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
    let Some(original_ruleset_name) = ruleset_name else {
        eprintln!("Failed to get the ruleset name");
        return;
    };

    // Get the ruleset from the index file
    let rulesets = load_ruleset(&original_ruleset_name);
    let mut ruleset = if rulesets.len() == 1 {
        match rulesets.into_iter().next().unwrap() {
            Ok(ruleset) => ruleset,
            Err(err) => {
                eprintln!("Failed to load the ruleset: {:#?}", err);
                return;
            }
        }
    } else {
        eprintln!("'{}' is a ruleset group", original_ruleset_name);
        return;
    };

    edit_loop(&original_ruleset_name, &mut ruleset);
}

fn edit_loop(original_ruleset_name: &str, ruleset: &mut Ruleset) {
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
                    Err(err) => eprintln!("Failed to get the ruleset description: {}", err),
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
                    Err(err) => eprintln!("Failed to get the ruleset version: {}", err),
                }
            }
            EditAction::EditRules => rules_edit_loop(ruleset),
            EditAction::SaveAndExit => {
                if original_ruleset_name != ruleset.name {
                    let rulesets_dir = rulesets_dir().map(||)
                    std::fs::remove_file();
                }

                ruleset.modified();
                if let Err(err) = save_ruleset(&ruleset.name, ruleset) {
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

fn rules_edit_loop(ruleset: &mut Ruleset) {
    loop {
        let Ok(rule_action) =
            inquire::Select::new("Select an action", EditRuleAction::iter().collect()).prompt()
        else {
            eprintln!("Failed to get the action");
            return;
        };

        match rule_action {
            EditRuleAction::Back => break,
            EditRuleAction::AddRule => {
                let mut new_rule = Rule::default();
                match rule_field(&mut new_rule) {
                    Ok(()) => {
                        let name = new_rule.name.clone();
                        ruleset.rules.push(new_rule);
                        println!("Rule '{}' added.", name);
                    }
                    Err(err) => eprintln!("Failed to add a new rule: {}", err),
                };
            }
            EditRuleAction::RemoveRule => {
                match select_rule(ruleset) {
                    Ok(RuleSelection::Back) => (),
                    Ok(RuleSelection::Rule(index, id)) => {
                        // Remove the rule from the ruleset
                        ruleset.rules.remove(index);
                        println!("Rule '{}' removed.", id);
                    }
                    Err(err) => eprintln!("Failed to select a rule: {}", err),
                }
            }
            EditRuleAction::SelectRule => match select_rule(ruleset) {
                Ok(RuleSelection::Back) => (),
                Ok(RuleSelection::Rule(index, _)) => {
                    if let Some(rule) = ruleset.rules.get_mut(index) {
                        match rule_field(rule) {
                            Ok(()) => {
                                println!("Rule '{}' edited.", rule.name);
                            }
                            Err(err) => eprintln!("Failed to add a new rule: {}", err),
                        };
                    }
                }
                Err(err) => eprintln!("Failed to select a rule: {}", err),
            },
        }
    }
}

fn rule_field(rule: &mut Rule) -> InquireResult<()> {
    let Rule {
        name,
        message,
        severity,
        checker,
        args,
        condition,
    } = rule;

    *name = inquire::Text::new("Name")
        .with_validator(name_validator)
        .with_initial_value(name)
        .prompt()?;
    *checker = inquire::Select::new("Checker", ALL_CHECKERS.to_vec())
        .prompt()?
        .to_string();
    *severity = inquire::Select::new("Severity", Severity::all()).prompt()?;
    *message = inquire::Text::new("Message")
        .with_validator(
            MaxLengthValidator::new(4096)
                .with_message("Description must be 4096 characters or less"),
        )
        .with_initial_value(message)
        .prompt()?;

    Ok(())
}

/// Lets user select a rule from the ruleset.
fn select_rule(ruleset: &Ruleset) -> InquireResult<RuleSelection> {
    let mut choices = vec![RuleSelection::Back];
    choices.extend(
        ruleset
            .rules
            .iter()
            .enumerate()
            .map(|(index, rule)| RuleSelection::Rule(index, rule.name.clone())),
    );
    inquire::Select::new("Select a rule", choices).prompt()
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

#[derive(EnumIter)]
enum RuleSelection {
    Back,
    Rule(usize, String),
}

impl Display for RuleSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleSelection::Back => write!(f, "Back"),
            RuleSelection::Rule(index, id) => write!(f, "{}. {}", index, id),
        }
    }
}
