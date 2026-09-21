use crate::{storage::get_rulesets_and_groups, tui::print_result};
use plint_linter::{LintEntry, LintResult};
use std::{cmp::Ordering, path::PathBuf};

pub(crate) fn lint(files: &[PathBuf], rulesets: &[String], verbose: bool) {
    let parsed_rulesets = get_rulesets_and_groups(rulesets);

    for file in files {
        let doc = match plint_linter::Document::from_file(file) {
            Ok(doc) => doc,
            Err(err) => {
                println!("Lint Error: {}", err);
                continue;
            }
        };

        let mut entries = Vec::new();
        for result in &parsed_rulesets {
            match result {
                Ok(ruleset) => {
                    entries.extend(ruleset.lint(&doc));
                }
                Err(err) => {
                    println!("Lint error: {:?}", err);
                }
            }
        }

        entries.sort_by(sort_entries);
        print_result(&doc, &entries);
    }

    if !verbose {
        println!("Use --verbose to show more detailed result");
    }
}

fn sort_entries(a: &LintEntry, b: &LintEntry) -> Ordering {
    match (&a.result, &b.result) {
        (LintResult::LinterError(_), LintResult::Diagnostic { .. }) => Ordering::Less,
        (LintResult::Diagnostic { .. }, LintResult::LinterError(_)) => Ordering::Greater,
        (
            LintResult::Diagnostic {
                match_data: a_match,
                ..
            },
            LintResult::Diagnostic {
                match_data: b_match,
                ..
            },
        ) => match (a_match, b_match) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(a_match), Some(b_match)) => match a_match.start.cmp(&b_match.start) {
                Ordering::Equal => a_match.end.cmp(&b_match.end),
                m => m,
            },
        },
        (LintResult::LinterError(_), LintResult::LinterError(_)) => Ordering::Equal,
    }
}
