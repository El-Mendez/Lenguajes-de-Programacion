use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexError {
    #[error("missing opening parenthesis for expression `{1}` at position {0}")]
    MissingOpeningParenthesis(usize, String),
    #[error("missing closing parenthesis for expression `{1}` at position {0}")]
    MissingClosingParenthesis(usize, String),
    #[error("expected an argument at position {0} for the expression `{1}`")]
    MissingArgument(usize, String),
}