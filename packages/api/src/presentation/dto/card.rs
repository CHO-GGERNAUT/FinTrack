use core::str;

use crate::application::dto::{CreateCardOutput, DeleteCardOutput};
use crate::domain::enums::{CardBrand, CardIssuer, CardType};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCardRequest {
    pub card_number: String,
    pub issued_at: Option<NaiveDate>,
    pub expires_at: Option<NaiveDate>,
    pub billing_day: Option<i32>,
    pub brand: CardBrand,
    pub issuer: CardIssuer,
    pub card_type: CardType,
    pub name: Option<String>,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCardResponse {
    pub account_id: Uuid,
}

impl From<CreateCardOutput> for CreateCardResponse {
    fn from(output: CreateCardOutput) -> Self {
        Self {
            account_id: output.account_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCardRequest {
    pub account_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCardResponse {
    pub account_id: Uuid,
}

impl From<DeleteCardOutput> for DeleteCardResponse {
    fn from(output: DeleteCardOutput) -> Self {
        Self {
            account_id: output.account_id,
        }
    }
}
