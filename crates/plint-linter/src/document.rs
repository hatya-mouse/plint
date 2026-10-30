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

use std::path::Path;

pub struct Document {
    pub file_name: String,
    pub content: String,
}

impl Document {
    pub fn new(file_name: String, content: String) -> Self {
        Document { file_name, content }
    }

    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|str| str.to_string())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidFilename,
                    "Could not retrieve file name",
                )
            })?;
        Ok(Self::new(file_name, content))
    }
}
