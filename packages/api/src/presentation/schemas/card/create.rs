use crate::application::dto::CreateCardOutput;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCardRequest {
    pub card_number: String,
    pub issued_at: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,
    pub billing_day: Option<i32>,
    pub brand: CardBrand,
    pub issuer: CardIssuer,
    pub card_type: CardType,
    pub name: Option<String>,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Credit,
    Debit,
}

impl From<CardType> for crate::domain::enums::CardType {
    fn from(value: CardType) -> Self {
        match value {
            CardType::Credit => crate::domain::enums::CardType::Credit,
            CardType::Debit => crate::domain::enums::CardType::Debit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardBrand {
    Visa,
    Mastercard,
    Amex,
    Jcb,
    Unionpay,
    Etc,
}

impl From<CardBrand> for crate::domain::enums::CardBrand {
    fn from(value: CardBrand) -> Self {
        match value {
            CardBrand::Visa => crate::domain::enums::CardBrand::Visa,
            CardBrand::Mastercard => crate::domain::enums::CardBrand::Mastercard,
            CardBrand::Amex => crate::domain::enums::CardBrand::Amex,
            CardBrand::Jcb => crate::domain::enums::CardBrand::Jcb,
            CardBrand::Unionpay => crate::domain::enums::CardBrand::Unionpay,
            CardBrand::Etc => crate::domain::enums::CardBrand::Etc,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

impl From<CardIssuer> for crate::domain::enums::CardIssuer {
    fn from(value: CardIssuer) -> Self {
        match value {
            CardIssuer::Samsung => crate::domain::enums::CardIssuer::Samsung,
            CardIssuer::BC => crate::domain::enums::CardIssuer::BC,
            CardIssuer::Woori => crate::domain::enums::CardIssuer::Woori,
            CardIssuer::Hana => crate::domain::enums::CardIssuer::Hana,
            CardIssuer::Shinhan => crate::domain::enums::CardIssuer::Shinhan,
            CardIssuer::Hyundai => crate::domain::enums::CardIssuer::Hyundai,
            CardIssuer::KB => crate::domain::enums::CardIssuer::KB,
            CardIssuer::Lotte => crate::domain::enums::CardIssuer::Lotte,
            CardIssuer::NH => crate::domain::enums::CardIssuer::NH,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCardResponse {
    pub account_id: Uuid,
}

impl From<CreateCardOutput> for CreateCardResponse {
    fn from(output: CreateCardOutput) -> Self {
        Self {
            account_id: output.account_id,
        }
    }
}
