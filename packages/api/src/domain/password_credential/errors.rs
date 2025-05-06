use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PasswordCredentialError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Account is Locked")]
    AccountLocked,

    #[error("Hashing error {0}")]
    HashingError(String),
}
