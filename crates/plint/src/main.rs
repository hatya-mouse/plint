mod cmd;
mod consts;
mod storage;
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
            Some(RulesetCommands::Check { rulesets }) => cmd::ruleset::check(&rulesets),
            Some(RulesetCommands::Create {
                name,
                authors,
                description,
                version,
            }) => cmd::ruleset::create(name, authors, description, version),
            Some(RulesetCommands::Edit { ruleset }) => cmd::ruleset::edit(&ruleset),
            Some(RulesetCommands::List { path, path_nolink }) => {
                cmd::ruleset::list(path, path_nolink)
            }
            None => (),
        },
        Some(Commands::Group { command }) => match command {
            Some(GroupCommands::Create { name }) => cmd::group::create(name),
            Some(GroupCommands::Edit { group }) => cmd::group::edit(&group),
            Some(GroupCommands::Add { group, rulesets }) => cmd::group::add(&group, rulesets),
            Some(GroupCommands::RemoveSet { group, rulesets }) => {
                cmd::group::remove_set(&group, &rulesets)
            }
            Some(GroupCommands::List {
                rulesets,
                path,
                path_nolink,
            }) => cmd::group::list(rulesets, path, path_nolink),
            None => (),
        },
        Some(Commands::Remove { rulesets, force }) => {}
        None => (),
    };
}
