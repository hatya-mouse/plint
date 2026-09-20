pub mod document;
pub mod error;
pub mod ext_funcs;
pub mod ruleset;

pub use document::Document;
pub use error::LinterError;
pub use ruleset::{LintEntry, LintResult, Rule, Ruleset};
