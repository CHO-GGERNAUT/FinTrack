use thiserror::Error;
#[derive(Debug, Error)]
pub enum AccountError {
    #[error("duplicate")]
    Duplicate,
    #[error("account not found")]
    NotFound,
    #[error("unknown error {0}")]
    Unknown(String),
}
