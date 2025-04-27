use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::domain::{self, entities::Account};

use super::AccountRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CardRow {
    pub account_id: Uuid,

    pub card_number_last4: String,
    pub encrypted_card_number: Vec<u8>,
    pub issued_at: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,
    pub billing_day: Option<i32>,
    pub brand: CardBrandDb,
    pub issuer: CardIssuerDb,
    pub card_type: CardTypeDb,
    pub name: Option<String>,
    pub memo: Option<String>,
}

impl From<(CardRow, AccountRow)> for domain::entities::Card {
    fn from((card_row, account_row): (CardRow, AccountRow)) -> Self {
        Self {
            account: Account {
                id: account_row.id,
                owner_id: account_row.owner_id,
                created_at: account_row.created_at,
                updated_at: account_row.updated_at,
                deleted_at: account_row.deleted_at,
                account_type: domain::enums::AccountType::Card,
            },
            card_number_last4: card_row.card_number_last4,
            encrypted_card_number: card_row.encrypted_card_number,
            issued_at: card_row.issued_at,
            expires_at: card_row.expires_at,
            billing_day: card_row.billing_day,
            brand: domain::enums::CardBrand::from(card_row.brand),
            issuer: domain::enums::CardIssuer::from(card_row.issuer),
            card_type: domain::enums::CardType::from(card_row.card_type),
            name: card_row.name,
            memo: card_row.memo,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(type_name = "card_type", rename_all = "lowercase")]
pub enum CardTypeDb {
    Credit,
    Debit,
    Prepaid,
}

impl From<domain::enums::CardType> for CardTypeDb {
    fn from(value: domain::enums::CardType) -> Self {
        match value {
            domain::enums::CardType::Credit => Self::Credit,
            domain::enums::CardType::Debit => Self::Debit,
            domain::enums::CardType::Prepaid => Self::Prepaid,
        }
    }
}

impl From<CardTypeDb> for domain::enums::CardType {
    fn from(value: CardTypeDb) -> Self {
        match value {
            CardTypeDb::Credit => Self::Credit,
            CardTypeDb::Debit => Self::Debit,
            CardTypeDb::Prepaid => Self::Prepaid,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(type_name = "card_brand", rename_all = "lowercase")]
pub enum CardBrandDb {
    Visa,
    Mastercard,
    Amex,
    Jcb,
    Unionpay,
    Etc,
}

impl From<domain::enums::CardBrand> for CardBrandDb {
    fn from(value: domain::enums::CardBrand) -> Self {
        match value {
            domain::enums::CardBrand::Visa => Self::Visa,
            domain::enums::CardBrand::Mastercard => Self::Mastercard,
            domain::enums::CardBrand::Amex => Self::Amex,
            domain::enums::CardBrand::Jcb => Self::Jcb,
            domain::enums::CardBrand::Unionpay => Self::Unionpay,
            domain::enums::CardBrand::Etc => Self::Etc,
        }
    }
}

impl From<CardBrandDb> for domain::enums::CardBrand {
    fn from(value: CardBrandDb) -> Self {
        match value {
            CardBrandDb::Visa => Self::Visa,
            CardBrandDb::Mastercard => Self::Mastercard,
            CardBrandDb::Amex => Self::Amex,
            CardBrandDb::Jcb => Self::Jcb,
            CardBrandDb::Unionpay => Self::Unionpay,
            CardBrandDb::Etc => Self::Etc,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(type_name = "card_issuer", rename_all = "lowercase")]
pub enum CardIssuerDb {
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

impl From<domain::enums::CardIssuer> for CardIssuerDb {
    fn from(value: domain::enums::CardIssuer) -> Self {
        match value {
            domain::enums::CardIssuer::Samsung => Self::Samsung,
            domain::enums::CardIssuer::BC => Self::BC,
            domain::enums::CardIssuer::Woori => Self::Woori,
            domain::enums::CardIssuer::Hana => Self::Hana,
            domain::enums::CardIssuer::Shinhan => Self::Shinhan,
            domain::enums::CardIssuer::Hyundai => Self::Hyundai,
            domain::enums::CardIssuer::KB => Self::KB,
            domain::enums::CardIssuer::Lotte => Self::Lotte,
            domain::enums::CardIssuer::NH => Self::NH,
        }
    }
}

impl From<CardIssuerDb> for domain::enums::CardIssuer {
    fn from(value: CardIssuerDb) -> Self {
        match value {
            CardIssuerDb::Samsung => Self::Samsung,
            CardIssuerDb::BC => Self::BC,
            CardIssuerDb::Woori => Self::Woori,
            CardIssuerDb::Hana => Self::Hana,
            CardIssuerDb::Shinhan => Self::Shinhan,
            CardIssuerDb::Hyundai => Self::Hyundai,
            CardIssuerDb::KB => Self::KB,
            CardIssuerDb::Lotte => Self::Lotte,
            CardIssuerDb::NH => Self::NH,
        }
    }
}
