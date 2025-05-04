use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Invalid email format: {0}")]
    InvalidEmail(String),
    #[error("Invalid phone number: {0}")]
    InvalidPhoneNumber(String),
    #[error("User Status is invalid. {0}")]
    InvalidUserStatus(String),

    #[error("User({0}) not found.")]
    NotFound(String),

    #[error("User({0}) already exists.")]
    Conflict(String),

    #[error("Unknown error occurred.")]
    Unknown,
}
