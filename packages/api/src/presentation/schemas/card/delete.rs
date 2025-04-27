use crate::domain::entities::Card;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct FindByIdResponse {
    pub account_id: Uuid,
    pub card_number_last4: String,
    pub issued_at: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,
    pub billing_day: Option<i32>,
    pub brand: String,
    pub issuer: String,
    pub card_type: String,
    pub name: Option<String>,
    pub memo: Option<String>,
}

impl From<Card> for FindByIdResponse {
    fn from(card: Card) -> Self {
        Self {
            account_id: card.account_id,
            card_number_last4: card.card_number_last4,
            issued_at: card.issued_at,
            expires_at: card.expires_at,
            billing_day: card.billing_day,
            brand: format!("{:?}", card.brand),
            issuer: format!("{:?}", card.issuer),
            card_type: format!("{:?}", card.card_type),
            name: card.name,
            memo: card.memo,
        }
    }
}
