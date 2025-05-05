use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CardError {
    #[error("Card has expired")]
    Expired,

    #[error("Card Validation Error{0}")]
    Validation(String),
}
