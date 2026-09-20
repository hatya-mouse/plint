#[derive(Debug, Clone)]
pub enum LinterError {
    /// The rule lacks all required arguments.
    MissingArgs,
    /// The rule lacks a specific required argument.
    MissingArg { arg: String },
    /// The rule has an argument that is invalid or cannot be parsed.
    InvalidArg { arg: String, reason: Option<String> },
    /// The rule specifies a checker that does not exist.
    UnknownChecker(String),
    /// The rule lacks a condition, but a checker returns a value that requires a condition to evaluate.
    MissingCondition,
    /// An error happends when executing the code.
    CodeError(String),
}
