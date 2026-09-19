pub(super) mod group;
pub(super) mod lint;
pub(super) mod ruleset;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(super) struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(super) debug: u8,

    #[command(subcommand)]
    pub(super) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(super) enum Commands {
    /// Lint the file(s) with the specified ruleset(s) and group(s).
    /// Leave the rulesets and groups empty to use the all available rulesets
    Lint {
        /// Path(s) to the file(s) to lint
        files: Vec<PathBuf>,
        /// Optional name(s) of the ruleset(s) and group(s) to use for linting.
        /// Leave this empty to use all available rulesets
        #[arg(short, long)]
        rulesets: Option<Vec<String>>,
    },
    /// Manage rulesets
    Ruleset {
        #[command(subcommand)]
        command: Option<RulesetCommands>,
    },
    /// Manage groups of rulesets
    Group {
        #[command(subcommand)]
        command: Option<GroupCommands>,
    },
    /// Remove ruleset(s) or group(s) from the local storage.
    /// Removing group(s) does not remove its rulesets
    Remove {
        /// Name(s) of the ruleset(s) and group(s) to remove.
        /// Leave this empty to select rulesets and groups interactively
        rulesets: Vec<String>,
        /// Whether to remove the ruleset(s) and group(s) forcibly without confirmation
        #[arg(short)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub(super) enum RulesetCommands {
    /// Check the validity of the ruleset(s)
    Check {
        /// Name(s) of the ruleset(s) and group(s) to verify.
        /// Leave this empty to verify all available rulesets
        rulesets: Vec<String>,
    },
    /// Create a new ruleset
    Create {
        /// Name of the ruleset to create
        name: String,
        /// Authors of the ruleset to create
        #[arg(short, long)]
        authors: Option<String>,
        /// Description of the ruleset to create
        #[arg(short, long)]
        description: Option<String>,
        /// Version of the ruleset to create
        #[arg(short, long)]
        version: Option<u64>,
    },
    /// Edit an existing ruleset in the default editor
    Edit {
        /// Name of the ruleset to edit
        ruleset: String,
    },
    /// List all installed or locally available rulesets
    List,
}

#[derive(Subcommand)]
pub(super) enum GroupCommands {
    /// Create a new ruleset group
    Create {
        /// Name of the group to create.
        /// Leave this empty to create a new group interactively
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Edit an existing ruleset group in the default editor
    Edit {
        /// Name of the group to edit
        group: String,
    },
    /// Add ruleset(s) to the group
    Add {
        /// Name of the group to add the ruleset to
        group: String,
        /// Name(s) of the ruleset(s) to add to the group
        rulesets: Vec<String>,
    },
    /// Remove ruleset(s) from the group
    RemoveSet {
        /// Name of the group to add the ruleset to
        group: String,
        /// Name(s) of the ruleset(s) to add to the group
        rulesets: Vec<String>,
    },
    /// List all installed or locally available groups
    List,
}
