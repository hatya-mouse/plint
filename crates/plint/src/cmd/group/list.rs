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

use crate::{
    storage::{Group, IndexFile},
    utils::hyperlink,
};

pub(crate) fn list(rulesets: bool, path: bool, path_nolink: bool) {
    let index_file = match IndexFile::load() {
        Ok(index_file) => index_file,
        Err(e) => {
            eprintln!("Failed to load index file: {:#?}", e);
            return;
        }
    };

    for (group_name, group_path) in index_file.groups {
        if path_nolink {
            println!("{}: {}", group_name, group_path.display());
        } else if path {
            println!("{}: {}", group_name, hyperlink(&group_path));
        } else {
            println!("{}", group_name);
        }

        if rulesets {
            match Group::load(&group_name) {
                Ok(group) => {
                    for ruleset in group.rulesets {
                        println!("  - {}", ruleset);
                    }
                }
                Err(err) => {
                    eprintln!("  Failed to load group '{}': {:#?}", group_name, err);
                }
            }
        }
    }
}
