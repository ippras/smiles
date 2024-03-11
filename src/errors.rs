use crate::{lexer::Lexeme, syntax::SyntaxKind};
use thiserror::Error;

/// Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error
#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Syntax(SyntaxError),
    #[error("end of string")]
    EndOfString,
}

#[derive(Clone, Error, Debug)]
#[error("syntax error {{ expected: {expected:?}, found: {found:?} }}")]
pub(crate) struct SyntaxError {
    pub(crate) expected: &'static [SyntaxKind],
    pub(crate) found: Lexeme,
}
