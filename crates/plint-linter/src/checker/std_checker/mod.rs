pub mod byte_count;
pub mod char_count;
pub mod line_count;
pub mod regex;
pub mod word_count;

use crate::{
    LinterError,
    checker::{CheckerEnum, CheckerInit},
};
use byte_count::ByteCountChecker;
use char_count::CharCountChecker;
use line_count::LineCountChecker;
use regex::RegexChecker;

pub(super) fn new_std_checker(
    check: &str,
    args: Option<&yaml_serde::Mapping>,
) -> Result<CheckerEnum, LinterError> {
    match check {
        "std.byte_count" => Ok(CheckerEnum::StdByteCount(ByteCountChecker::new(args)?)),
        "std.char_count" => Ok(CheckerEnum::StdCharCount(CharCountChecker::new(args)?)),
        "std.line_count" => Ok(CheckerEnum::StdLineCount(LineCountChecker::new(args)?)),
        "std.regex" => Ok(CheckerEnum::StdRegex(RegexChecker::new(args)?)),
        _ => Err(LinterError::UnknownChecker(check.to_string())),
    }
}
