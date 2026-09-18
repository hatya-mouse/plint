mod args;
mod consts;
mod lint;
mod utils;

use crate::args::Commands;
use clap::Parser;
use lint::lint_doc;

fn main() {
    let cli = args::Cli::parse();

    match &cli.command {
        Some(Commands::Lint { files, rulesets }) => {
            let mut parsed_rulesets = Vec::new();
            for ruleset_name in rulesets {
                parsed_rulesets.extend(load_ruleset());
            }

            for file in files {
                let doc = match plint_linter::Document::from_file(file) {
                    Ok(doc) => doc,
                    Err(e) => continue,
                };
                lint_doc(&doc, &parsed_rulesets);
            }
        }
        _ => (),
    };
}
