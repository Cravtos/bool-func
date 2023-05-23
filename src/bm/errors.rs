use thiserror::Error;

pub type Result<T> = std::result::Result<T, BMError>;

#[derive(Error, Debug, PartialEq)]
pub enum BMError {
    #[error("dimension can't be zero (got ({0}, {1})")]
    ZeroDim(usize, usize),
    #[error("inconsistent dimentions")]
    InconsistentDim,
    #[error("given char doesn't convert to boolean: {0}")]
    InvalidStr(char),
}
