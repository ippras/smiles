use crate::{lexer::Lexeme, syntax::SyntaxKind};
use thiserror::Error;

/// Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error
#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Syntax(#[from] SyntaxError),
    // #[error("end of string")]
    // EndOfString,
}

#[derive(Clone, Error, Debug)]
#[error("syntax error {{ expected: {expected:?}, found: {found:?} }}")]
pub struct SyntaxError {
    pub expected: &'static [SyntaxKind],
    pub found: Lexeme,
}
