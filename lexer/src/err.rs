use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexError {
    #[error("unexpected opening parenthesis")]
    MissingOpeningParenthesis,
    #[error("missing closing parenthesis")]
    MissingClosingParenthesis,
    #[error("expected another character")]
    MissingArgument,
    #[error("unknown error")]
    Unknown,
}