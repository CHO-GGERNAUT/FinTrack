use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::enums::{CardBrand, CardIssuer, CardType};

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

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
