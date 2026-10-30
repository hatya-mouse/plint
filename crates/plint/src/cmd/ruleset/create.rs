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

use crate::storage::{IndexFile, RulesetIo};
use plint_linter::Ruleset;

pub(crate) fn create(
    name: String,
    authors: Option<String>,
    description: Option<String>,
    version: Option<u64>,
) {
    let index_file = match IndexFile::load() {
        Ok(index_file) => index_file,
        Err(err) => {
            eprintln!("Failed to load index file: {:#?}", err);
            return;
        }
    };

    if index_file.is_name_registered(&name) {
        eprintln!("Ruleset name already exists: {}", name);
        return;
    }

    // Create and save the ruleset
    let ruleset = Ruleset::new_empty(name.clone(), authors, description, version);
    match ruleset.save() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to create a ruleset: {:#?}", err);
            return;
        }
    }

    println!("Ruleset created successfully: {}", name);
}
