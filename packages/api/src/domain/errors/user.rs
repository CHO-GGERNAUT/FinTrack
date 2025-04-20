use thiserror::Error;
#[derive(Debug, Error)]
pub enum UserError {
    #[error("User already exists")]
    Duplicate,
    #[error("Invalid email")]
    InvalidEmail,
    #[error("User not found")]
    NotFound,
    #[error("unknown error {0}")]
    Unknown(String),
}
