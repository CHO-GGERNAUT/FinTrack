use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::commands::card::{IssueCardCommand, IssueCardResult};

#[derive(Debug, Deserialize)]
pub struct IssueCardRequest {
    pub card_number: String,
    pub expiration_date_year: u32,
    pub expiration_date_month: u8,
    pub issuance_date: NaiveDate,
    pub card_type: String,
    pub card_brand: String,
    pub card_issuer: String,
    pub name: Option<String>,
}

impl From<(IssueCardRequest, Uuid)> for IssueCardCommand {
    fn from((req, user_id): (IssueCardRequest, Uuid)) -> Self {
        Self {
            user_id,
            card_number: req.card_number,
            expiration_date: (req.expiration_date_year, req.expiration_date_month),
            issuance_date: req.issuance_date,
            card_type: req.card_type,
            card_brand: req.card_brand,
            card_issuer: req.card_issuer,
            name: req.name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IssueCardResponse {
    pub card_id: String,
    pub created_at: String,
}

impl From<IssueCardResult> for IssueCardResponse {
    fn from(output: IssueCardResult) -> Self {
        Self {
            card_id: output.card_id.to_string(),
            created_at: output.created_at,
        }
    }
}
