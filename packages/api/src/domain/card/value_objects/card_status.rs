use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardStatus {
    Active,
    Inactive, // Explicitly deactivated by user/system, not expired or blocked
    Expired,
    Closed,
}

#[derive(Debug, Error)]
pub enum CardStatusError {
    #[error("Invalid card status string: {0}")]
    InvalidString(String),
}

impl TryFrom<&str> for CardStatus {
    type Error = CardStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "active" => Ok(CardStatus::Active),
            "inactive" => Ok(CardStatus::Inactive),
            "expired" => Ok(CardStatus::Expired),
            "closed" => Ok(CardStatus::Closed),
            _ => Err(CardStatusError::InvalidString(value.to_string())),
        }
    }
}

impl From<CardStatus> for &'static str {
    fn from(value: CardStatus) -> Self {
        match value {
            CardStatus::Active => "active",
            CardStatus::Inactive => "inactive",
            CardStatus::Expired => "expired",
            CardStatus::Closed => "closed",
        }
    }
}
