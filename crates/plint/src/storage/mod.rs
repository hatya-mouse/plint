mod group;
mod index;
mod ruleset;

pub(super) use group::Group;
pub(super) use index::IndexFile;
pub(super) use ruleset::RulesetIo;

use plint_linter::Ruleset;

#[derive(Debug)]
pub(super) enum PlintIoError {
    PathNotAvailable,
    NotFound(String),
    YamlParseError(yaml_serde::Error),
    IoError(std::io::Error),
}

// --- MULTIPLE RULESETS ---

pub(super) fn get_rulesets_and_groups(rulesets: &[String]) -> Vec<Result<Ruleset, PlintIoError>> {
    let mut parsed_rulesets = Vec::new();

    if rulesets.is_empty() {
        match IndexFile::load() {
            Ok(index) => {
                for ruleset_name in index.rulesets.keys() {
                    parsed_rulesets.push(Ruleset::load(ruleset_name));
                }
            }
            Err(err) => {
                parsed_rulesets.push(Err(err));
            }
        }
    } else {
        for name in rulesets {
            if let Ok(group) = Group::load(name) {
                for ruleset_name in &group.rulesets {
                    parsed_rulesets.push(Ruleset::load(ruleset_name));
                }
            } else {
                // Load as a single ruleset if it's not a group
                parsed_rulesets.push(Ruleset::load(name));
            }
        }
    }

    parsed_rulesets
}
