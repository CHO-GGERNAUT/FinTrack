use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardBrand {
    Visa,
    Mastercard,
    Amex,
    JCB,
    UnionPay,
    Etc,
}

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

impl FromStr for CardBrand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "visa" => Ok(CardBrand::Visa),
            "mastercard" => Ok(CardBrand::Mastercard),
            "amex" => Ok(CardBrand::Amex),
            "jcb" => Ok(CardBrand::JCB),
            "unionpay" => Ok(CardBrand::UnionPay),
            _ => Ok(CardBrand::Etc),
        }
    }
}

impl FromStr for CardIssuer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "samsung" => Ok(CardIssuer::Samsung),
            "bc" => Ok(CardIssuer::BC),
            "woori" => Ok(CardIssuer::Woori),
            "hana" => Ok(CardIssuer::Hana),
            "shinhan" => Ok(CardIssuer::Shinhan),
            "hyundai" => Ok(CardIssuer::Hyundai),
            "kb" => Ok(CardIssuer::KB),
            "lotte" => Ok(CardIssuer::Lotte),
            "nh" => Ok(CardIssuer::NH),
            _ => Err(format!("Unknown card issuer: {}", s)),
        }
    }
}

impl FromStr for CardType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "credit" => Ok(CardType::Credit),
            "debit" => Ok(CardType::Debit),
            "prepaid" => Ok(CardType::Prepaid),
            _ => Err(format!("Unknown card type: {}", s)),
        }
    }
}
