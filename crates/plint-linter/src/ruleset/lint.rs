use crate::{
    Document, LinterError,
    ext_funcs::add_ext_funcs,
    ruleset::{Rule, Ruleset, Severity},
};
use plint_lang::{Interpreter, Value};
use std::range::Range;

impl Ruleset {
    pub fn lint(&self, doc: &Document) -> Vec<LintEntry> {
        let mut results = Vec::new();

        for rule in &self.rules {
            // Create an interpreter
            let mut interpreter = Interpreter::default();

            // Add external functions
            add_ext_funcs(&mut interpreter);

            // Add the document content as file name as constants
            interpreter.add_const("file_name", Value::String(doc.file_name.clone()));
            interpreter.add_const("doc", Value::String(doc.content.clone()));

            // Run the code and get the result
            let result = interpreter.run(&rule.code);

            let result_value = match result {
                Ok(value) => value,
                Err(code_error) => {
                    results.push(LintEntry {
                        rule_name: rule.name.clone(),
                        result: LintResult::LinterError(LinterError::CodeError(code_error)),
                    });
                    continue;
                }
            };

            // Process the result and add the lint entries
            let processed_results = process_code_result(rule, result_value);
            results.extend(processed_results);
        }

        results
    }
}

fn process_code_result(rule: &Rule, value: Value) -> Vec<LintEntry> {
    let mut results = Vec::new();

    match value {
        Value::Bool(boolean) => {
            if boolean {
                results.push(LintEntry {
                    rule_name: rule.name.clone(),
                    result: LintResult::Diagnostic {
                        message: rule.message.clone(),
                        severity: rule.severity.clone(),
                        match_data: None,
                    },
                });
            }
        }
        Value::List(matches) => {
            for item in matches {
                if let Value::Match(m) = item {
                    results.push(LintEntry {
                        rule_name: rule.name.clone(),
                        result: LintResult::Diagnostic {
                            message: rule.message.clone(),
                            severity: rule.severity.clone(),
                            match_data: Some(m),
                        },
                    });
                }
            }
        }
        Value::Match(m) => {
            results.push(LintEntry {
                rule_name: rule.name.clone(),
                result: LintResult::Diagnostic {
                    message: rule.message.clone(),
                    severity: rule.severity.clone(),
                    match_data: Some(m),
                },
            });
        }
        _ => (),
    }

    results
}

#[derive(Debug, Clone)]
pub struct LintEntry {
    pub rule_name: String,
    pub result: LintResult,
}

#[derive(Debug, Clone)]
pub enum LintResult {
    LinterError(LinterError),
    Diagnostic {
        message: String,
        severity: Severity,
        match_data: Option<Range<usize>>,
    },
}
