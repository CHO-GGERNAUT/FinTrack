use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardIssuer {
    Samsung,
    BC,
    Woori,
    Hana,
    Shinhan,
    Hyundai,
    KB,
    Lotte,
    NH,
}

#[derive(Debug, Error)]
pub enum CardIssuerError {
    #[error("Invalid card network string: {0}")]
    InvalidString(String),
}

impl TryFrom<&str> for CardIssuer {
    type Error = CardIssuerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "samsung" => Ok(CardIssuer::Samsung),
            "bc" => Ok(CardIssuer::BC),
            "woori" => Ok(CardIssuer::Woori),
            "hana" => Ok(CardIssuer::Hana),
            "shinhan" => Ok(CardIssuer::Shinhan),
            "hyundai" => Ok(CardIssuer::Hyundai),
            "kb" => Ok(CardIssuer::KB),
            "lotte" => Ok(CardIssuer::Lotte),
            "nh" => Ok(CardIssuer::NH),
            _ => Err(CardIssuerError::InvalidString(value.to_string())),
        }
    }
}

impl From<CardIssuer> for &'static str {
    fn from(value: CardIssuer) -> Self {
        match value {
            CardIssuer::Samsung => "samsung",
            CardIssuer::BC => "bc",
            CardIssuer::Woori => "woori",
            CardIssuer::Hana => "hana",
            CardIssuer::Shinhan => "shinhan",
            CardIssuer::Hyundai => "hyundai",
            CardIssuer::KB => "kb",
            CardIssuer::Lotte => "lotte",
            CardIssuer::NH => "nh",
        }
    }
}
