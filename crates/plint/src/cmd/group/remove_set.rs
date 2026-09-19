use crate::storage::Group;

pub(crate) fn remove_set(group: &str, rulesets: &[String]) {
    if rulesets.is_empty() {
        eprintln!("No rulesets specified to be removed.");
        return;
    }

    let mut group = match Group::load(group) {
        Ok(group) => group,
        Err(err) => {
            eprintln!("Failed to load the group: {:#?}", err);
            return;
        }
    };

    // Remove the specified rulesets from the group
    for ruleset in rulesets {
        if let Some(index) = group.rulesets.iter().position(|r| r == ruleset) {
            group.rulesets.remove(index);
            println!("Removed ruleset: {}", ruleset);
        } else {
            eprintln!("Ruleset not found: {}", ruleset);
        }
    }

    // Save the updated group back to storage
    if let Err(err) = group.save() {
        eprintln!("Failed to save changes: {:#?}", err);
    }
}
