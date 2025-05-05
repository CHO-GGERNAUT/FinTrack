use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
}

#[derive(Debug, Error)]
pub enum CardTypeError {
    #[error("Invalid card type string: {0}")]
    InvalidString(String),
}

impl TryFrom<&str> for CardType {
    type Error = CardTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "credit" => Ok(CardType::Credit),
            "debit" => Ok(CardType::Debit),
            "prepaid" => Ok(CardType::Prepaid),
            _ => Err(CardTypeError::InvalidString(value.to_string())),
        }
    }
}

impl From<CardType> for &'static str {
    fn from(value: CardType) -> Self {
        match value {
            CardType::Credit => "credit",
            CardType::Debit => "debit",
            CardType::Prepaid => "prepaid",
        }
    }
}
