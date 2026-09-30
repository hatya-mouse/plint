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

mod group;
mod index;
mod ruleset;

pub(super) use group::Group;
pub(super) use index::IndexFile;
pub(super) use ruleset::RulesetIo;

use plint_linter::Ruleset;

#[derive(Debug)]
pub(super) enum PlintIoError {
    PathNotAvailable,
    NotFound(String),
    YamlParseError(yaml_serde::Error),
    IoError(std::io::Error),
}

// --- MULTIPLE RULESETS ---

pub(super) fn get_rulesets_and_groups(rulesets: &[String]) -> Vec<Result<Ruleset, PlintIoError>> {
    let mut parsed_rulesets = Vec::new();

    if rulesets.is_empty() {
        match IndexFile::load() {
            Ok(index) => {
                for ruleset_name in index.rulesets.keys() {
                    parsed_rulesets.push(Ruleset::load(ruleset_name));
                }
            }
            Err(err) => {
                parsed_rulesets.push(Err(err));
            }
        }
    } else {
        for name in rulesets {
            if let Ok(group) = Group::load(name) {
                for ruleset_name in &group.rulesets {
                    parsed_rulesets.push(Ruleset::load(ruleset_name));
                }
            } else {
                // Load as a single ruleset if it's not a group
                parsed_rulesets.push(Ruleset::load(name));
            }
        }
    }

    parsed_rulesets
}
