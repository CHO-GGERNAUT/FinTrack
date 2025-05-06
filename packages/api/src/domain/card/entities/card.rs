use chrono::NaiveDate;

use crate::domain::{shared::value_objects::AuditInfo, user::value_objects::UserId};

use super::super::value_objects::{
    CardBrand, CardId, CardIssuer, CardNumber, CardStatus, CardType, ExpirationDate,
};

use super::super::errors::CardError;

#[derive(Debug, Clone)]
pub struct Card {
    version: u8,
    id: CardId,
    user_id: UserId,

    card_number: CardNumber,
    last_number: String,
    expiration_date: ExpirationDate,
    card_type: CardType,
    card_brand: CardBrand,
    card_issuer: CardIssuer,
    status: CardStatus,
    name: Option<String>,

    issuance_date: NaiveDate,
    audit_info: AuditInfo,
}

impl Card {
    pub fn issue(
        user_id: UserId,
        card_number: CardNumber,
        expiration_date: ExpirationDate,
        issuance_date: NaiveDate,
        card_type: CardType,
        card_brand: CardBrand,
        card_issuer: CardIssuer,
        name: Option<String>,
    ) -> Result<Self, CardError> {
        let last_number = card_number.last_four();

        Ok(Self {
            id: CardId::new(),
            version: 0,
            user_id,
            card_number,
            last_number,
            expiration_date,
            card_type,
            card_brand,
            card_issuer,
            status: CardStatus::Active,
            issuance_date,
            audit_info: AuditInfo::record_creation(),
            name,
        })
    }

    pub fn reconstitute(
        version: u8,
        id: CardId,
        user_id: UserId,
        card_number: CardNumber,
        last_number: String,
        expiration_date: ExpirationDate,
        card_type: CardType,
        card_brand: CardBrand,
        card_issuer: CardIssuer,
        status: CardStatus,
        issuance_date: NaiveDate,
        audit_info: AuditInfo,
        name: Option<String>,
    ) -> Self {
        Self {
            id,
            version,
            user_id,
            card_number,
            last_number,
            expiration_date,
            card_type,
            card_brand,
            card_issuer,
            status,
            issuance_date,
            audit_info,
            name,
        }
    }
    pub fn close(&mut self) -> Result<(), CardError> {
        if self.status == CardStatus::Closed {
            return Ok(());
        }
        self.status = CardStatus::Closed;
        self.audit_info.record_update();
        Ok(())
    }
}

// Getters
impl Card {
    pub fn id(&self) -> &CardId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn card_number(&self) -> &CardNumber {
        &self.card_number
    }

    pub fn expiration_date(&self) -> &ExpirationDate {
        &self.expiration_date
    }

    pub fn card_type(&self) -> &CardType {
        &self.card_type
    }

    pub fn card_brand(&self) -> &CardBrand {
        &self.card_brand
    }

    pub fn card_issuer(&self) -> &CardIssuer {
        &self.card_issuer
    }

    pub fn status(&self) -> CardStatus {
        self.status
    }

    pub fn issuance_date(&self) -> &NaiveDate {
        &self.issuance_date
    }
    pub fn audit_info(&self) -> &AuditInfo {
        &self.audit_info
    }
    pub fn last_number(&self) -> &str {
        &self.last_number
    }
    pub fn version(&self) -> u8 {
        self.version
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
