use crate::{
    Document, LinterError, Match,
    checker::{self, CheckResult, Checker},
    ruleset::{Rule, Ruleset},
};

impl Ruleset {
    pub fn lint(&self, doc: &Document) -> Vec<LintEntry> {
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
                    results.push(LintEntry {
                        rule_id: rule.id.clone(),
                        result: LintResult::LinterError(linter_error),
                    });
                }
            }
        }

        results
    }
}

fn process_check_result(rule: &Rule, check_result: CheckResult) -> Vec<LintEntry> {
    let mut results;

    match check_result {
        CheckResult::Matches(matches) => {
            results = Vec::with_capacity(matches.len());
            for m in matches {
                results.push(LintEntry {
                    rule_id: rule.id.clone(),
                    result: LintResult::Info {
                        message: rule.message.clone(),
                        match_data: Some(m),
                    },
                });
            }
        }
        CheckResult::Value(value) => {
            results = Vec::new();

            if let Some(condition) = rule.condition.as_ref() {
                let eval_result = condition.evaluate(&value);

                if eval_result {
                    results.push(LintEntry {
                        rule_id: rule.id.clone(),
                        result: LintResult::Info {
                            message: rule.message.clone(),
                            match_data: None,
                        },
                    });
                }
            } else {
                results.push(LintEntry {
                    rule_id: rule.id.clone(),
                    result: LintResult::MissingCondition {
                        value: value.clone(),
                    },
                })
            }
        }
    }

    results
}

#[derive(Debug, Clone)]
pub struct LintEntry {
    pub rule_id: String,
    pub result: LintResult,
}

#[derive(Debug, Clone)]
pub enum LintResult {
    LinterError(LinterError),
    MissingCondition {
        value: crate::Value,
    },
    Info {
        message: String,
        match_data: Option<Match>,
    },
}
