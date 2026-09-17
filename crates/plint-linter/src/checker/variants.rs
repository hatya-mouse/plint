use super::std_checker::{
    self, byte_count::ByteCountChecker, char_count::CharCountChecker, line_count::LineCountChecker,
    regex::RegexChecker,
};
use crate::LinterError;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Checker)]
pub(crate) enum CheckerEnum {
    StdByteCount(ByteCountChecker),
    StdCharCount(CharCountChecker),
    StdLineCount(LineCountChecker),
    StdRegex(RegexChecker),
}

pub(crate) fn checker_from(
    check: &str,
    args: Option<&yaml_serde::Mapping>,
) -> Result<CheckerEnum, LinterError> {
    if check.starts_with("std.") {
        std_checker::new_std_checker(check, args)
    } else {
        Err(LinterError::UnknownChecker(check.to_string()))
    }
}
