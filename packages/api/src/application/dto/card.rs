use crate::domain::enums::{CardBrand, CardIssuer, CardType};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateCardInput {
    pub owner_id: Uuid,
    pub card_number_last4: String,
    pub encrypted_card_number: Vec<u8>,
    pub issued_at: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,
    pub billing_day: Option<i32>,
    pub brand: CardBrand,
    pub issuer: CardIssuer,
    pub card_type: CardType,

    pub name: Option<String>,
    pub memo: Option<String>,
}

#[derive(Debug)]
pub struct CreateCardOutput {
    pub account_id: Uuid,
}

#[derive(Debug)]
pub struct DeleteCardInput {
    pub account_id: Uuid,
    pub owner_id: Uuid,
}

#[derive(Debug)]
pub struct DeleteCardOutput {
    pub account_id: Uuid,
}
