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
    /// Lint the files with the specified rulesets and groups.
    /// Leave the rulesets and groups empty to use the all available rulesets
    Lint {
        /// Paths to the files to lint
        files: Vec<PathBuf>,
        /// Optional names of the rulesets and groups to use for linting.
        /// Leave this empty to use all available rulesets
        #[arg(short, long)]
        rulesets: Vec<String>,
    },
    /// Manage rulesets
    Ruleset {
        #[command(subcommand)]
        command: RulesetCommands,
    },
    /// Manage groups of rulesets
    Group {
        #[command(subcommand)]
        command: GroupCommands,
    },
    /// Remove rulesets or groups from the local storage.
    /// Removing groups does not remove its rulesets
    Remove {
        /// Names of the rulesets and groups to remove.
        /// Leave this empty to select rulesets and groups interactively
        rulesets: Vec<String>,
        /// Whether to remove the rulesets and groups forcibly without confirmation
        #[arg(short)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub(super) enum RulesetCommands {
    /// Check the validity of the rulesets
    Check {
        /// Names of the rulesets and groups to verify.
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
    List {
        /// Show the path of the rulesets
        #[arg(short, long)]
        path: bool,
        /// Show the path of the rulesets without hyperlinks
        #[arg(long)]
        path_nolink: bool,
    },
}

#[derive(Subcommand)]
pub(super) enum GroupCommands {
    /// Create a new ruleset group
    Create {
        /// Name of the group to create
        name: String,
    },
    /// Edit an existing ruleset group in the default editor
    Edit {
        /// Name of the group to edit
        group: String,
    },
    /// Add rulesets to the group
    Add {
        /// Name of the group to add the ruleset to
        group: String,
        /// Names of the rulesets to add to the group
        #[arg(short, long)]
        rulesets: Vec<String>,
    },
    /// Remove rulesets from the group
    RemoveSet {
        /// Name of the group to add the ruleset to
        group: String,
        /// Names of the rulesets to add to the group
        #[arg(short, long)]
        rulesets: Vec<String>,
    },
    /// List all installed or locally available groups
    List {
        /// Show the rulesets in the group
        #[arg(short, long)]
        rulesets: bool,
        /// Show the path of the groups (and rulesets, if --rulesets is specified)
        #[arg(short, long)]
        path: bool,
        /// Show the path of the groups (and rulesets, if --rulesets is specified) without hyperlinks
        #[arg(long)]
        path_nolink: bool,
    },
}
