pub mod byte_count;
pub mod char_count;
pub mod line_count;
pub mod regex;
pub mod sentence_count;
pub mod word_count;

use crate::{
    LinterError,
    checker::{CheckerEnum, CheckerInit},
};
use byte_count::ByteCountChecker;
use char_count::CharCountChecker;
use line_count::LineCountChecker;
use regex::RegexChecker;
use sentence_count::SentenceCountChecker;
use word_count::WordCountChecker;

pub(super) fn new_std_checker(
    check: &str,
    args: Option<&yaml_serde::Mapping>,
) -> Result<CheckerEnum, LinterError> {
    match check {
        "std.byte-count" => Ok(CheckerEnum::StdByteCount(ByteCountChecker::new(args)?)),
        "std.char-count" => Ok(CheckerEnum::StdCharCount(CharCountChecker::new(args)?)),
        "std.line-count" => Ok(CheckerEnum::StdLineCount(LineCountChecker::new(args)?)),
        "std.regex" => Ok(CheckerEnum::StdRegex(RegexChecker::new(args)?)),
        "std.sentence-count" => Ok(CheckerEnum::StdSentenceCount(SentenceCountChecker::new(
            args,
        )?)),
        "std.word-count" => Ok(CheckerEnum::StdWordCount(WordCountChecker::new(args)?)),
        _ => Err(LinterError::UnknownChecker(check.to_string())),
    }
}
