use thiserror::Error;

pub type Result<T> = std::result::Result<T, BFError>;

#[derive(Error, Debug, PartialEq)]
pub enum BFError {
    #[error("args_amount is zero (should be greater)")]
    NoArgs,
    #[error("string `{0}` contains invalid characters (got expected '0' or '1')")]
    InvalidString(String),
    #[error("string length should be power of two (got: `{0}`)")]
    NotPowTwo(usize),
    #[error("given argument ({given}) is out of bounds ({bounds})")]
    ArgOutOfBounds { given: usize, bounds: usize },
}
