use std::num::ParseIntError;
use thiserror::Error;

/// Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Semantic error
#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error("element not found")]
    ElementNotFound,
    #[error("node not found")]
    NodeNotFound,
    #[error("tree not found")]
    TreeNotFound,
}
