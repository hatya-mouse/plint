//
//  Copyright 2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::storage::Group;

pub(crate) fn add_set(group: &str, rulesets: Vec<String>) {
    if rulesets.is_empty() {
        eprintln!("No rulesets specified to be added.");
        return;
    }

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
