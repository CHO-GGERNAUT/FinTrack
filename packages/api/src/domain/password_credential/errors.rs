use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PasswordCredentialError {
    #[error("Invalid email or password")]
    InvalidCredentials,

    #[error("Account is Locked")]
    AccountLocked,

    #[error("Hashing error {0}")]
    HashFailed(String),

    #[error("Credential is not supported")]
    InvalidCredentialType,
}
