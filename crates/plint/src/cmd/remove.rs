use crate::storage::{Group, IndexFile, RulesetIo};
use plint_linter::Ruleset;
use std::fmt::Display;

enum RemoveTarget<'a> {
    Ruleset(&'a str),
    Group(&'a str),
}

impl Display for RemoveTarget<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoveTarget::Ruleset(ruleset) => write!(f, "ruleset '{}'", ruleset),
            RemoveTarget::Group(group) => write!(f, "group '{}'", group),
        }
    }
}

pub(crate) fn remove(rulesets: &[String], force: bool) {
    if rulesets.is_empty() {
        println!("No rulesets or groups specified.");
        return;
    }

    let index_file = match IndexFile::load() {
        Ok(index_file) => index_file,
        Err(err) => {
            println!("Error loading index file: {:#?}", err);
            return;
        }
    };

    // Collect targets and show not found errors
    let mut targets = Vec::new();
    for name in rulesets {
        if index_file.rulesets.contains_key(name) {
            targets.push(RemoveTarget::Ruleset(name));
        } else if index_file.groups.contains_key(name) {
            targets.push(RemoveTarget::Group(name));
        } else {
            println!("Ruleset or group not found: {}", name);
        }
    }

    if targets.is_empty() {
        println!("No valid rulesets or groups to remove.");
        return;
    }

    // Remove targets
    for target in targets {
        if !force {
            // If force is not specified, ask for confirmation
            let confirmation = inquire::Confirm::new(&format!(
                "Are you sure you want to remove the {}? (y/n)",
                target
            ))
            .prompt();

            match confirmation {
                Ok(true) => (),
                Ok(false) => {
                    println!("Skipping the {}", target);
                    continue;
                }
                Err(err) => {
                    println!("Error during confirmation: {:#?}", err);
                    return;
                }
            }
        }

        // Remove the target
        let remove_result = match target {
            RemoveTarget::Ruleset(ruleset) => Ruleset::remove(ruleset),
            RemoveTarget::Group(group) => Group::remove(group),
        };

        match remove_result {
            Ok(_) => {
                println!("Removed the {}", target);
            }
            Err(err) => {
                println!("Error removing {}: {:#?}", target, err);
            }
        }
    }
}
