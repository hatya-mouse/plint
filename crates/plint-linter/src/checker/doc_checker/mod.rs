pub mod byte_count;
pub mod char_count;
pub mod line_count;
pub mod regex;
pub mod regex_count;
pub mod sentence_count;
pub mod word_count;

use crate::{
    LinterError,
    checker::{CheckerEnum, CheckerInit, doc_checker::regex_count::RegexCountChecker},
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
        "doc.byte-count" => Ok(CheckerEnum::DocByteCount(ByteCountChecker::new(args)?)),
        "doc.char-count" => Ok(CheckerEnum::DocCharCount(CharCountChecker::new(args)?)),
        "doc.line-count" => Ok(CheckerEnum::DocLineCount(LineCountChecker::new(args)?)),
        "doc.regex" => Ok(CheckerEnum::DocRegex(RegexChecker::new(args)?)),
        "doc.regex-count" => Ok(CheckerEnum::DocRegexCount(RegexCountChecker::new(args)?)),
        "doc.sentence-count" => Ok(CheckerEnum::DocSentenceCount(SentenceCountChecker::new(
            args,
        )?)),
        "doc.word-count" => Ok(CheckerEnum::DocWordCount(WordCountChecker::new(args)?)),
        _ => Err(LinterError::UnknownChecker(check.to_string())),
    }
}
