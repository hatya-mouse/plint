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
