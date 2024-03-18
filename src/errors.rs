pub use crate::{semantic::Error as SemanticError, syntax::Error as SyntaxError};

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
