use super::SyntaxKind;
use crate::lexer::Lexeme;
use thiserror::Error;

/// Syntax error
#[derive(Clone, Error, Debug)]
#[error("syntax error {{ expected: {expected:?}, found: {found:?} }}")]
pub struct Error {
    pub expected: &'static [SyntaxKind],
    pub found: Lexeme,
}
