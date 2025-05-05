use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardBrand {
    Visa,
    Mastercard,
    Amex,
    Jcb,
    Discover,
    Unionpay,
    Etc,
}

#[derive(Debug, Error)]
pub enum CardBrandError {
    #[error("Invalid card network string: {0}")]
    InvalidString(String),
}

impl TryFrom<&str> for CardBrand {
    type Error = CardBrandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "visa" => Ok(CardBrand::Visa),
            "mastercard" => Ok(CardBrand::Mastercard),
            "amex" | "american express" => Ok(CardBrand::Amex),
            "discover" => Ok(CardBrand::Discover),
            "jcb" => Ok(CardBrand::Jcb),
            "unionpay" => Ok(CardBrand::Unionpay),
            "etc" => Ok(CardBrand::Etc),
            _ => Err(CardBrandError::InvalidString(value.to_string())),
        }
    }
}

impl From<CardBrand> for &'static str {
    fn from(value: CardBrand) -> Self {
        match value {
            CardBrand::Visa => "visa",
            CardBrand::Mastercard => "mastercard",
            CardBrand::Amex => "amex",
            CardBrand::Discover => "discover",
            CardBrand::Jcb => "jcb",
            CardBrand::Unionpay => "unionpay",
            CardBrand::Etc => "etc",
        }
    }
}
