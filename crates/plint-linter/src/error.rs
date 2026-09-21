use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LinterError {
    /// An error happends when executing the code.
    CodeError(String),
}

impl Display for LinterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinterError::CodeError(error) => write!(f, "{}", error),
        }
    }
}
