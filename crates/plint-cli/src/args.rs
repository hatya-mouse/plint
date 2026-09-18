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
        rulesets: Vec<String>,
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
}

#[derive(Subcommand)]
pub(super) enum RulesetCommands {
    /// Check the validity of the ruleset(s)
    Check {
        /// Name(s) of the ruleset(s) and group(s) to verify
        rulesets: Vec<String>,
    },
    /// Edit an existing ruleset
    Edit {
        /// Name of the ruleset to edit
        ruleset: String,
    },
    /// Create a new ruleset
    Create {
        /// Name of the ruleset to edit
        ruleset: String,
    },
    /// List all installed or locally available rulesets
    List,
}

#[derive(Subcommand)]
pub(super) enum GroupCommands {
    /// Edit an existing ruleset group
    Edit { group: String },
    /// Create a new ruleset group interactively
    Create { group: String },
    /// Add ruleset(s) to the group
    Add {
        /// Name of the group to add the ruleset to
        group: String,
        /// Name(s) of the ruleset(s) to add to the group
        ruleset: Vec<String>,
    },
    /// Remove ruleset(s) from the group
    Remove {
        /// Name of the group to add the ruleset to
        group: String,
        /// Name(s) of the ruleset(s) to add to the group
        ruleset: Vec<String>,
    },
    /// List all installed or locally available groups
    List,
}
