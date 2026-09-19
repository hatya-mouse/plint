use crate::storage::Group;

pub(crate) fn add(group: &str, rulesets: Vec<String>) {
    let mut group = match Group::load(group) {
        Ok(group) => group,
        Err(err) => {
            eprintln!("Failed to load the group: {:#?}", err);
            return;
        }
    };

    // Add the specified rulesets from the group
    for ruleset in rulesets {
        if group.rulesets.contains(&ruleset) {
            println!("Ruleset already exists in the group: {}", ruleset);
        } else {
            println!("Added ruleset: {}", ruleset);
            group.rulesets.push(ruleset);
        }
    }

    // Save the updated group back to storage
    if let Err(err) = group.save() {
        eprintln!("Failed to save changes: {:#?}", err);
    }
}
