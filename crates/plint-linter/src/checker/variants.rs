use super::{CheckerEnum, doc_checker};
use crate::LinterError;

pub(crate) fn checker_from(
    check: &str,
    args: Option<&yaml_serde::Mapping>,
) -> Result<CheckerEnum, LinterError> {
    if check.starts_with("doc.") {
        doc_checker::new_std_checker(check, args)
    } else {
        Err(LinterError::UnknownChecker(check.to_string()))
    }
}
