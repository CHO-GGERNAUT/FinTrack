use thiserror::Error;
#[derive(Debug, Error)]
pub enum MerchantError {
    #[error("duplicate merchant")]
    Duplicate,
    #[error("merchant not found")]
    NotFound,
    #[error("unknown error {0}")]
    Unknown(String),
}
