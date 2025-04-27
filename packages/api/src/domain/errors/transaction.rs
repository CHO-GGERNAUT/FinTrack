use thiserror::Error;
#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("unknown error {0}")]
    Unknown(String),
}
