use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid email or password")]
    InvalidCredentials,

    #[error("Account is Locked")]
    AccountLocked,

    #[error("Hashing error {0}")]
    HashFailed(#[from] bcrypt::BcryptError),
}
