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

use std::path::{Path, PathBuf};

pub(super) fn data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join(crate::consts::DATA_DIR_NAME))
}

pub(super) fn rulesets_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join(crate::consts::DATA_DIR_NAME).join("rulesets"))
}

pub(super) fn groups_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join(crate::consts::DATA_DIR_NAME).join("groups"))
}

/// Returns a hyperlink string for the given path, which can be used in terminal output.
pub(super) fn hyperlink(path: &Path) -> String {
    let url = url::Url::from_file_path(path).unwrap().to_string();
    format!("\x1b]8;;{url}\x07{}\x1b]8;;\x07", path.display())
}
