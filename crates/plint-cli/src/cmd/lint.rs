use crate::{
    PlintIoError,
    storage::{load_group, load_index_file, load_ruleset},
};
use plint_linter::{Document, Ruleset};
use std::path::PathBuf;

pub(crate) fn lint(files: &[PathBuf], rulesets: Option<&Vec<String>>) {
    let mut parsed_rulesets = Vec::new();

    if let Some(rulesets) = rulesets {
        for name in rulesets {
            if let Ok(group) = load_group(name) {
                for ruleset_name in &group.rulesets {
                    parsed_rulesets.push(load_ruleset(ruleset_name));
                }
            } else {
                // Load as a single ruleset if it's not a group
                parsed_rulesets.push(load_ruleset(name));
            }
        }
    } else {
        match load_index_file() {
            Ok(index) => {
                for ruleset_name in index.rulesets.keys() {
                    parsed_rulesets.push(load_ruleset(ruleset_name));
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

fn process_ruleset(doc: &Document, result: &Result<Ruleset, PlintIoError>) {
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
