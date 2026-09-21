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
        Some(Commands::Lint { files, rulesets }) => cmd::lint::lint(&files, &rulesets),
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
