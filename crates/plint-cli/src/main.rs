mod args;
mod consts;
mod ruleset;
mod utils;

use crate::{
    args::Commands,
    ruleset::{RulesetLoadError, load_index_file, load_ruleset},
};
use clap::Parser;
use plint_linter::{Document, Ruleset};

fn main() {
    let cli = args::Cli::parse();

    match &cli.command {
        Some(Commands::Lint { files, rulesets }) => {
            let mut parsed_rulesets = Vec::new();
            if let Some(rulesets) = rulesets {
                for ruleset_name in rulesets {
                    parsed_rulesets.extend(load_ruleset(ruleset_name));
                }
            } else {
                match load_index_file() {
                    Ok(index) => {
                        for ruleset_name in index.rulesets.keys() {
                            parsed_rulesets.extend(load_ruleset(ruleset_name));
                        }
                    }
                    Err(err) => {
                        parsed_rulesets.push(Err(err));
                    }
                }
            }

            for file in files {
                let doc = match plint_linter::Document::from_file(file) {
                    Ok(doc) => doc,
                    Err(err) => {
                        println!("Lint Error: {}", err);
                        continue;
                    }
                };

                for result in &parsed_rulesets {
                    process_ruleset(&doc, result);
                }
            }
        }
        _ => (),
    };
}

fn process_ruleset(doc: &Document, result: &Result<Ruleset, RulesetLoadError>) {
    match result {
        Ok(ruleset) => {
            let lint_results = ruleset.lint(doc);
            println!("{:#?}", lint_results);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
