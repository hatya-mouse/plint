use crate::{PlintIoError, storage::get_rulesets_and_groups, tui::print_result};
use plint_linter::{Document, Ruleset};
use std::path::PathBuf;

pub(crate) fn lint(files: &[PathBuf], rulesets: &[String]) {
    let parsed_rulesets = get_rulesets_and_groups(rulesets);

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

fn process_ruleset(doc: &Document, result: &Result<Ruleset, PlintIoError>) {
    match result {
        Ok(ruleset) => {
            let entries = ruleset.lint(doc);
            print_result(doc, &entries);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
