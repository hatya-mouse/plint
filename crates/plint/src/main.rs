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

mod cmd;
mod consts;
mod storage;
mod tui;
mod utils;

use crate::{
    cmd::{Commands, GroupCommands, RulesetCommands},
    storage::PlintIoError,
};
use clap::Parser;

fn main() {
    let cli = cmd::Cli::parse();

    match cli.command {
        Some(Commands::Lint {
            files,
            rulesets,
            verbose,
        }) => cmd::lint::lint(&files, &rulesets, verbose),
        Some(Commands::Ruleset { command }) => match command {
            RulesetCommands::Create {
                name,
                authors,
                description,
                version,
            } => cmd::ruleset::create(name, authors, description, version),
            RulesetCommands::Edit { ruleset } => cmd::ruleset::edit(&ruleset),
            RulesetCommands::List { path, path_nolink } => cmd::ruleset::list(path, path_nolink),
        },
        Some(Commands::Group { command }) => match command {
            GroupCommands::Create { name } => cmd::group::create(name),
            GroupCommands::Edit { group } => cmd::group::edit(&group),
            GroupCommands::AddSet { group, rulesets } => cmd::group::add_set(&group, rulesets),
            GroupCommands::RemoveSet { group, rulesets } => {
                cmd::group::remove_set(&group, &rulesets)
            }
            GroupCommands::List {
                rulesets,
                path,
                path_nolink,
            } => cmd::group::list(rulesets, path, path_nolink),
        },
        Some(Commands::Remove { rulesets, force }) => cmd::remove::remove(&rulesets, force),
        None => (),
    };
}
