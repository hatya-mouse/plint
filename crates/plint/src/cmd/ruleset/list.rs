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

use crate::{storage::IndexFile, utils::hyperlink};

pub(crate) fn list(path: bool, path_nolink: bool) {
    let index_file = match IndexFile::load() {
        Ok(index_file) => index_file,
        Err(e) => {
            eprintln!("Failed to load index file: {:#?}", e);
            return;
        }
    };

    for (ruleset_name, ruleset_path) in index_file.rulesets {
        if path_nolink {
            println!("{}: {}", ruleset_name, ruleset_path.display());
        } else if path {
            println!("{}: {}", ruleset_name, hyperlink(&ruleset_path));
        } else {
            println!("{}", ruleset_name);
        }
    }
}
