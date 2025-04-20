use thiserror::Error;
#[derive(Debug, Error)]
pub enum CardError {
    #[error("invalid card number")]
    InvalidCardNumber,
    #[error("duplicate card")]
    Duplicate,
    #[error("card not found")]
    NotFound,
    #[error("unknown error {0}")]
    Unknown(String),
}
