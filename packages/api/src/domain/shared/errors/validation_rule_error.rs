use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainValidationRuleError {
    #[error("Invalid format for email address: '{0}'")]
    InvalidEmailFormat(String),

    #[error("Invalid format for card number: '{0}'")]
    InvalidCardNumberFormat(String),

    #[error("Invalid format for phone number: '{0}'")]
    InvalidPhoneNumberFormat(String),

    #[error("Invalid format for hash: '{0}'")]
    InvalidHashFormat(String),

    #[error("Card expiration date is in the past: {0}")]
    ExpirationDateInPast(String),

    #[error("Value '{value}' is out of range for field '{field}'")]
    ValueOutOfRange {
        field: &'static str,
        value: String,
        min: Option<String>,
        max: Option<String>,
    },

    #[error("Field '{field}' is empty or missing")]
    FieldEmptyOrMissing { field: &'static str },

    #[error("Invalid status value: '{0}'")]
    InvalidStatusValue(String),
}
