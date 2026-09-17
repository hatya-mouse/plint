pub enum LinterError {
    /// The rule lacks all required arguments.
    MissingArgs,
    /// The rule lacks a specific required argument.
    MissingArg { arg: String },
    /// The rule has an argument that is invalid or cannot be parsed.
    InvalidArg { arg: String, reason: Option<String> },
    /// The rule specifies a checker that is not recognized or supported.
    UnknownChecker(String),
}
