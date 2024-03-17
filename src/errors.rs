use crate::{lexer::Lexeme, syntax::SyntaxKind};
use std::num::ParseIntError;
use thiserror::Error;

/// Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error
#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Syntax(#[from] SyntaxError),
    #[error(transparent)]
    Semantic(#[from] SemanticError),
}

/// Semantic error
#[derive(Clone, Error, Debug)]
pub enum SemanticError {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    UnknownElement(#[from] UnknownElement),
    #[error("element not found")]
    ElementNotFound,
}

/// Unknown element
#[derive(Clone, Copy, Debug, Default, Error)]
#[error("unknown element")]
pub struct UnknownElement;

#[derive(Clone, Error, Debug)]
#[error("syntax error {{ expected: {expected:?}, found: {found:?} }}")]
pub struct SyntaxError {
    pub expected: &'static [SyntaxKind],
    pub found: Lexeme,
}
