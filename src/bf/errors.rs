use thiserror::Error;

pub type Result<T> = std::result::Result<T, BFError>;

#[derive(Error, Debug)]
pub enum BFError {
    #[error("args_amount shouldn't be zero")]
    NoArgs,
}
