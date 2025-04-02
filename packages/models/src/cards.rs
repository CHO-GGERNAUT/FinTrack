use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Card {
    pub id: Uuid,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub last_4_digits: u16,
    pub encrypted_card_number: Vec<u8>,
    pub card_type: CardType,
    pub international_network: InternationalNetwork,
    pub network: DomesticNetwork,

    pub issue_date: NaiveDate,
    pub valid_from: NaiveDate,
    pub cvc: u8,

    pub card_owner: Uuid,
    pub meta_data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
pub enum CardType {
    Credit,
    Debit,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
pub enum InternationalNetwork {
    Visa,
    Mastercard,
    AmericanExpress,
    Discover,
    JCB,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
pub enum DomesticNetwork {
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
