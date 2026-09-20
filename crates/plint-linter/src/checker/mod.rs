mod doc_checker;
mod variants;

pub(crate) use variants::checker_from;

use crate::{
    Document, LinterError, Match,
    checker::doc_checker::{
        byte_count::ByteCountChecker, char_count::CharCountChecker, contains::ContainsChecker,
        line_count::LineCountChecker, regex::RegexChecker, regex_count::RegexCountChecker,
        sentence_count::SentenceCountChecker, word_count::WordCountChecker,
    },
};
use enum_dispatch::enum_dispatch;

pub(crate) enum CheckResult {
    Matches(Vec<Match>),
    Value(crate::Value),
}

pub(crate) enum CheckResultType {
    Matches,
    Value,
}

pub(crate) trait CheckerInit: Sized {
    fn new(args: Option<&yaml_serde::Mapping>) -> Result<Self, LinterError>;
}

#[enum_dispatch]
pub(crate) trait Checker {
    fn check(&self, doc: &Document) -> CheckResult;

    fn check_type(&self) -> CheckResultType;
}

#[enum_dispatch(Checker)]
pub(crate) enum CheckerEnum {
    DocByteCount(ByteCountChecker),
    DocCharCount(CharCountChecker),
    DocContains(ContainsChecker),
    DocLineCount(LineCountChecker),
    DocRegex(RegexChecker),
    DocRegexCount(RegexCountChecker),
    DocSentenceCount(SentenceCountChecker),
    DocWordCount(WordCountChecker),
}

pub const ALL_CHECKERS: &[&str] = &[
    "doc.byte-count",
    "doc.char-count",
    "doc.contains",
    "doc.line-count",
    "doc.regex",
    "doc.regex-count",
    "doc.sentence-count",
    "doc.word-count",
];
