use chrono::NaiveDate;

use crate::domain::{
    shared::value_objects::{AuditInfo, CurrencyValue},
    user::value_objects::UserId,
};

use super::super::value_objects::{
    CardBrand, CardId, CardIssuer, CardNumber, CardStatus, CardType, ExpirationDate,
};

use super::super::errors::CardError;

#[derive(Debug, Clone)]
pub struct Card {
    id: CardId,
    version: u64,
    user_id: UserId,

    card_number: CardNumber,
    last_number: String,
    expiration_date: ExpirationDate,
    card_type: CardType,
    card_brand: CardBrand,
    card_issuer: CardIssuer,
    status: CardStatus,

    issuance_date: NaiveDate,
    credit_limit: Option<CurrencyValue>,
    cash_withdrawal_limit: Option<CurrencyValue>,

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
            credit_limit: None,
            cash_withdrawal_limit: None,
            audit_info: AuditInfo::record_creation(),
        })
    }

    pub fn close(&mut self) -> Result<(), CardError> {
        if self.status == CardStatus::Closed {
            return Ok(());
        }
        self.status = CardStatus::Closed;
        self.audit_info.record_update();
        Ok(())
    }

    pub fn set_credit_limit(&mut self, limit: CurrencyValue) -> Result<(), CardError> {
        if self.card_type != CardType::Credit {
            return Err(CardError::Validation(
                "Cannot set credit limit on non-credit card".to_string(),
            ));
        }
        if limit < CurrencyValue::zero(limit.currency()) {
            return Err(CardError::Validation(
                "Credit limit cannot be negative".to_string(),
            ));
        }
        self.credit_limit = Some(limit);
        self.audit_info.record_update();
        Ok(())
    }
}
