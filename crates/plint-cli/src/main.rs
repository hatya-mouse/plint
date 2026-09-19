mod args;
mod cmd;
mod consts;
mod storage;
mod utils;

use crate::{
    args::{Commands, GroupCommands, RulesetCommands},
    storage::PlintIoError,
};
use clap::Parser;

fn main() {
    let cli = args::Cli::parse();

    match &cli.command {
        Some(Commands::Lint { files, rulesets }) => cmd::lint::lint(files, rulesets.as_ref()),
        Some(Commands::Ruleset { command }) => match command {
            Some(RulesetCommands::Check { rulesets }) => {}
            Some(RulesetCommands::Create {
                name,
                authors,
                description,
                version,
            }) => {
                cmd::ruleset::create(name.clone(), authors.clone(), description.clone(), *version)
            }
            Some(RulesetCommands::List) => {}
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
