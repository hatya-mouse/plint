use crate::{
    Document, LinterError, Match,
    checker::{self, CheckResult, Checker},
    ruleset::{Rule, Ruleset},
};

impl Ruleset {
    pub fn lint(&self, doc: &Document) -> Vec<LintResult> {
        let mut results = Vec::new();

        for rule in &self.rules {
            let checker = checker::checker_from(&rule.checker, rule.args.as_ref());

            match checker {
                Ok(checker) => {
                    let check_result = checker.check(doc);
                    let processed_results = process_check_result(rule, check_result);
                    results.extend(processed_results);
                }
                Err(linter_error) => {
                    results.push(LintResult::LinterError(linter_error));
                }
            }
        }

        results
    }
}

fn process_check_result(rule: &Rule, check_result: CheckResult) -> Vec<LintResult> {
    let mut results;

    match check_result {
        CheckResult::Matches(matches) => {
            results = Vec::with_capacity(matches.len());
            for m in matches {
                results.push(LintResult::Entry {
                    message: rule.message.clone(),
                    match_data: Some(m),
                });
            }
        }
        CheckResult::Value(value) => {
            results = Vec::new();

            if let Some(condition) = rule.condition.as_ref() {
                let eval_result = condition.evaluate(&value);

                if eval_result {
                    results.push(LintResult::Entry {
                        message: rule.message.clone(),
                        match_data: None,
                    });
                }
            } else {
                results.push(LintResult::MissingCondition {
                    value: value.clone(),
                })
            }
        }
    }

    results
}

#[derive(Debug)]
pub enum LintResult {
    LinterError(LinterError),
    MissingCondition {
        value: crate::Value,
    },
    Entry {
        message: String,
        match_data: Option<Match>,
    },
}
