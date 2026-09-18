mod std_checker;
mod variants;

pub(crate) use variants::checker_from;

use crate::checker::std_checker::{
    sentence_count::SentenceCountChecker, word_count::WordCountChecker,
};
use crate::{Document, LinterError, Match};
use enum_dispatch::enum_dispatch;
use std_checker::{
    byte_count::ByteCountChecker, char_count::CharCountChecker, line_count::LineCountChecker,
    regex::RegexChecker,
};

pub(crate) enum CheckResult {
    Matches(Vec<Match>),
    Value(crate::Value),
}

pub(crate) trait CheckerInit: Sized {
    fn new(args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError>;
}

#[enum_dispatch]
pub(crate) trait Checker {
    fn check(&self, doc: &Document) -> CheckResult;
}

#[enum_dispatch(Checker)]
pub(crate) enum CheckerEnum {
    StdByteCount(ByteCountChecker),
    StdCharCount(CharCountChecker),
    StdLineCount(LineCountChecker),
    StdRegex(RegexChecker),
    StdSentenceCount(SentenceCountChecker),
    StdWordCount(WordCountChecker),
}

pub const ALL_CHECKERS: &[&str] = &[
    "std.byte-count",
    "std.char-count",
    "std.line-count",
    "std.regex",
    "std.sentence-count",
    "std.word-count",
];
