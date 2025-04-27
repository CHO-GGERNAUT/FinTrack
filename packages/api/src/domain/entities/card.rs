use chrono::NaiveDate;

use crate::domain::enums::{CardBrand, CardIssuer, CardType};

use super::Account;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub account: Account,

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
