use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Card {
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub card_number_last4: i32,
    pub encrypted_card_number: Vec<u8>,

    pub issued_at: NaiveDate,
    pub expires_at: NaiveDate,
    pub billing_day: i32, // Between 1 and 31
    pub credit_limit: Option<f64>,
    pub brand: CardBrand,   // 'visa', 'mastercard', 'amex', etc.
    pub issuer: CardIssuer, // 'samsung', 'hyundai', etc.
    pub card_type: CardType,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(type_name = "card_type")]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(type_name = "card_brand")]
#[sqlx(rename_all = "lowercase")]
pub enum CardBrand {
    Visa,
    Mastercard,
    Amex,
    JCB,
    UnionPay,
    Etc,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(type_name = "card_issuer")]
#[sqlx(rename_all = "lowercase")]
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
