use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Invalid email format.")]
    InvalidEmail,
    #[error("Invalid phone number: {0}")]
    InvalidPhoneNumber(String),
    #[error("User not found.")]
    UserNotFound,
    #[error("User Status is invalid. {0}")]
    InvalidUserStatus(String),
}
