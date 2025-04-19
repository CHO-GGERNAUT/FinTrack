use crate::domain::{
    entities,
    enums::{CardBrand, CardIssuer, CardType},
};
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateCardInput {
    pub account_id: Uuid,
    pub card_number_last4: String,
    pub encrypted_card_number: Vec<u8>,
    pub issued_at: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,
    pub billing_day: Option<i32>,
    pub credit_limit: Option<Decimal>,
    pub brand: CardBrand,
    pub issuer: CardIssuer,
    pub card_type: CardType,
}

#[derive(Debug)]
pub struct CreateCardOutput {
    pub card_id: Uuid,
    pub created_at: String, // ISO 8601
}

impl From<CreateCardInput> for entities::Card {
    fn from(input: CreateCardInput) -> Self {
        entities::Card {
            account_id: input.account_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            card_number_last4: input.card_number_last4,
            encrypted_card_number: input.encrypted_card_number,
            issued_at: input.issued_at,
            expires_at: input.expires_at,
            billing_day: input.billing_day,
            credit_limit: input.credit_limit,
            brand: input.brand,
            issuer: input.issuer,
            card_type: input.card_type,
        }
    }
}
