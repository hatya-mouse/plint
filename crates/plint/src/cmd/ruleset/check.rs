use crate::storage::get_rulesets_and_groups;
use plint_linter::LintResult;

pub(crate) fn check(rulesets: &[String]) {
    let parsed_rulesets = get_rulesets_and_groups(rulesets);

    for ruleset in parsed_rulesets {
        match ruleset {
            Ok(ruleset) => {
                println!("Checking ruleset: {}", ruleset.name);

                let check_result = ruleset.check_rules();
                for result in check_result {
                    if let LintResult::LinterError(err) = result.result {
                        println!("  Error on rule {}: {:#?}", result.rule_name, err);
                    }
                }
            }
            Err(e) => {
                eprintln!("  Error opening ruleset: {:#?}", e);
            }
        }
    }
}
