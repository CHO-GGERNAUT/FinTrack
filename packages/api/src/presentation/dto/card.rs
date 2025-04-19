use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::dto::CreateCardInput;

#[derive(Debug, Deserialize)]
pub struct CreateCardRequest {
    pub account_id: Uuid,
    pub card_number_last4: String,
    pub encrypted_card_number: Vec<u8>,
    pub issued_at: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,
    pub billing_day: Option<i32>,
    pub credit_limit: Option<Decimal>,
    pub brand: String,
    pub issuer: String,
    pub card_type: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCardResponse {
    pub account_id: Uuid,
    pub created_at: String, // ISO 8601
}

#[derive(Debug)]
pub enum CreateCardError {
    InvalidBrand(String),
    InvalidIssuer(String),
    InvalidCardType(String),
}

impl TryFrom<CreateCardRequest> for CreateCardInput {
    type Error = CreateCardError;

    fn try_from(req: CreateCardRequest) -> Result<Self, Self::Error> {
        let brand = req
            .brand
            .parse()
            .map_err(|_| CreateCardError::InvalidBrand(req.brand.clone()))?;
        let issuer = req
            .issuer
            .parse()
            .map_err(|_| CreateCardError::InvalidIssuer(req.issuer.clone()))?;
        let card_type = req
            .card_type
            .parse()
            .map_err(|_| CreateCardError::InvalidCardType(req.card_type.clone()))?;

        Ok(Self {
            account_id: req.account_id,
            card_number_last4: req.card_number_last4,
            encrypted_card_number: req.encrypted_card_number,
            issued_at: req.issued_at,
            expires_at: req.expires_at,
            billing_day: req.billing_day,
            credit_limit: req.credit_limit,
            brand,
            issuer,
            card_type,
        })
    }
}
