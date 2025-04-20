use thiserror::Error;
#[derive(Debug, Error)]
pub enum InfraError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
