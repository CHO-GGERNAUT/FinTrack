use thiserror::Error;
#[derive(Debug, Error)]
pub enum UowError {
    #[error("UnitOfWork commit error {0}")]
    CommitError(String),
    #[error("UnitOfWork rollback error {0}")]
    RollbackError(String),
    #[error("unknown error {0}")]
    Unknown(String),
}
