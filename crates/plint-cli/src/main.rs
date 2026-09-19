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

    match &cli.command {
        Some(Commands::Lint { files, rulesets }) => cmd::lint::lint(files, rulesets),
        Some(Commands::Ruleset { command }) => match command {
            Some(RulesetCommands::Check { rulesets }) => cmd::ruleset::check(rulesets),
            Some(RulesetCommands::Create {
                name,
                authors,
                description,
                version,
            }) => {
                cmd::ruleset::create(name.clone(), authors.clone(), description.clone(), *version)
            }
            Some(RulesetCommands::Edit { ruleset }) => cmd::ruleset::edit(ruleset),
            Some(RulesetCommands::List {
                paths,
                paths_nolinks,
            }) => cmd::ruleset::list(*paths, *paths_nolinks),
            None => (),
        },
        Some(Commands::Group { command }) => match command {
            Some(GroupCommands::Create { name }) => {}
            Some(GroupCommands::Edit { group }) => {}
            Some(GroupCommands::Add { group, rulesets }) => {}
            Some(GroupCommands::RemoveSet { group, rulesets }) => {}
            Some(GroupCommands::List) => {}
            None => (),
        },
        Some(Commands::Remove { rulesets, force }) => {}
        None => (),
    };
}
