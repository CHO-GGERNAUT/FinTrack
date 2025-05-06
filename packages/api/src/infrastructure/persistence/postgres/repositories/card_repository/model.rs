use chrono::{DateTime, Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::{
    application::interfaces::services::encryption_service::EncryptionService,
    domain::{
        card::{
            entities::Card,
            value_objects::{
                CardBrand, CardIssuer, CardNumber, CardStatus, CardType, ExpirationDate,
            },
        },
        shared::value_objects::AuditInfo,
    },
    infrastructure::persistence::errors::InfraError,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CardRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub version: i64,
    pub card_number: Vec<u8>,
    pub last_four_digits: String,

    pub card_type: CardTypeDb,
    pub card_brand: CardBrandDb,
    pub card_issuer: CardIssuerDb,
    pub card_status: CardStatusDb,

    pub name: Option<String>,
    pub expiration_date: NaiveDate,
    pub issuance_date: NaiveDate,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl CardRow {
    pub fn to_entity(
        self,
        encryption_service: &impl EncryptionService,
    ) -> Result<Card, InfraError> {
        let decrypted_card_number_str = encryption_service
            .decrypt(&self.card_number)
            .map_err(|e| InfraError::ReconstituteFailed(e.to_string()))?;

        let card_number_vo = CardNumber::new(decrypted_card_number_str)?; // Extract year from NaiveDate
        let expiration_date = ExpirationDate::new(
            self.expiration_date.year() as u32,
            (self.expiration_date.month0() + 1) as u8,
        )
        .map_err(|e| InfraError::ReconstituteFailed(e.to_string()))?; // Handle potential VO errors

        let card_type = self.card_type.into();
        let card_brand = self.card_brand.into();
        let card_issuer = self.card_issuer.into();
        let status = self.card_status.into();

        let audit_info = AuditInfo::new(self.created_at, self.updated_at, self.deleted_at);

        Ok(Card::reconstitute(
            self.version as u8,
            self.id.into(),
            self.user_id.into(),
            card_number_vo,
            self.last_four_digits,
            expiration_date,
            card_type,
            card_brand,
            card_issuer,
            status,
            self.issuance_date,
            audit_info,
            self.name,
        ))
    }

    pub fn from_entity(
        card: &Card,
        encryption_service: &impl EncryptionService,
    ) -> Result<Self, InfraError> {
        let encrypted_card_number = encryption_service
            .encrypt(card.card_number().value()) // Get plaintext from VO
            .map_err(|e| InfraError::ReconstituteFailed(e.to_string()))?;

        let expiration = card.expiration_date();
        let expiration_date =
            NaiveDate::from_ymd_opt(expiration.year() as i32, expiration.month() as u32, 1)
                .unwrap();

        Ok(CardRow {
            version: card.version() as i64,
            id: (*card.id()).into(),
            user_id: (*card.user_id()).into(),
            card_number: encrypted_card_number,
            last_four_digits: card.last_number().into(),
            card_type: (*card.card_type()).into(),
            card_brand: (*card.card_brand()).into(),
            card_issuer: (*card.card_issuer()).into(),
            card_status: card.status().into(),
            expiration_date,
            issuance_date: (*card.issuance_date()),
            name: card.name().cloned(),
            created_at: card.audit_info().created_at(),
            updated_at: card.audit_info().updated_at(),
            deleted_at: card.audit_info().deleted_at(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(type_name = "card_type", rename_all = "lowercase")]
pub enum CardTypeDb {
    Credit,
    Debit,
    Prepaid,
}

impl From<CardTypeDb> for CardType {
    fn from(card_type: CardTypeDb) -> Self {
        match card_type {
            CardTypeDb::Credit => CardType::Credit,
            CardTypeDb::Debit => CardType::Debit,
            CardTypeDb::Prepaid => CardType::Prepaid,
        }
    }
}
impl From<CardType> for CardTypeDb {
    fn from(card_type: CardType) -> Self {
        match card_type {
            CardType::Credit => CardTypeDb::Credit,
            CardType::Debit => CardTypeDb::Debit,
            CardType::Prepaid => CardTypeDb::Prepaid,
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
    Discover,
    Etc,
}

impl From<CardBrandDb> for CardBrand {
    fn from(card_brand: CardBrandDb) -> Self {
        match card_brand {
            CardBrandDb::Visa => CardBrand::Visa,
            CardBrandDb::Mastercard => CardBrand::Mastercard,
            CardBrandDb::Amex => CardBrand::Amex,
            CardBrandDb::Jcb => CardBrand::Jcb,
            CardBrandDb::Unionpay => CardBrand::Unionpay,
            CardBrandDb::Discover => CardBrand::Discover,
            CardBrandDb::Etc => CardBrand::Etc,
        }
    }
}
impl From<CardBrand> for CardBrandDb {
    fn from(card_brand: CardBrand) -> Self {
        match card_brand {
            CardBrand::Visa => CardBrandDb::Visa,
            CardBrand::Mastercard => CardBrandDb::Mastercard,
            CardBrand::Amex => CardBrandDb::Amex,
            CardBrand::Jcb => CardBrandDb::Jcb,
            CardBrand::Unionpay => CardBrandDb::Unionpay,
            CardBrand::Discover => CardBrandDb::Discover,
            CardBrand::Etc => CardBrandDb::Etc,
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
impl From<CardIssuerDb> for CardIssuer {
    fn from(card_issuer: CardIssuerDb) -> Self {
        match card_issuer {
            CardIssuerDb::Samsung => CardIssuer::Samsung,
            CardIssuerDb::BC => CardIssuer::BC,
            CardIssuerDb::Woori => CardIssuer::Woori,
            CardIssuerDb::Hana => CardIssuer::Hana,
            CardIssuerDb::Shinhan => CardIssuer::Shinhan,
            CardIssuerDb::Hyundai => CardIssuer::Hyundai,
            CardIssuerDb::KB => CardIssuer::KB,
            CardIssuerDb::Lotte => CardIssuer::Lotte,
            CardIssuerDb::NH => CardIssuer::NH,
        }
    }
}
impl From<CardIssuer> for CardIssuerDb {
    fn from(card_issuer: CardIssuer) -> Self {
        match card_issuer {
            CardIssuer::Samsung => CardIssuerDb::Samsung,
            CardIssuer::BC => CardIssuerDb::BC,
            CardIssuer::Woori => CardIssuerDb::Woori,
            CardIssuer::Hana => CardIssuerDb::Hana,
            CardIssuer::Shinhan => CardIssuerDb::Shinhan,
            CardIssuer::Hyundai => CardIssuerDb::Hyundai,
            CardIssuer::KB => CardIssuerDb::KB,
            CardIssuer::Lotte => CardIssuerDb::Lotte,
            CardIssuer::NH => CardIssuerDb::NH,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(type_name = "card_status", rename_all = "lowercase")]
pub enum CardStatusDb {
    Active,
    Inactive,
    Expired,
    Closed,
}

impl From<CardStatusDb> for CardStatus {
    fn from(card_status: CardStatusDb) -> Self {
        match card_status {
            CardStatusDb::Active => CardStatus::Active,
            CardStatusDb::Inactive => CardStatus::Inactive,
            CardStatusDb::Expired => CardStatus::Expired,
            CardStatusDb::Closed => CardStatus::Closed,
        }
    }
}
impl From<CardStatus> for CardStatusDb {
    fn from(card_status: CardStatus) -> Self {
        match card_status {
            CardStatus::Active => CardStatusDb::Active,
            CardStatus::Inactive => CardStatusDb::Inactive,
            CardStatus::Expired => CardStatusDb::Expired,
            CardStatus::Closed => CardStatusDb::Closed,
        }
    }
}
