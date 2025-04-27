use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::dto::{CreateTransactionInput, CreateTransactionOutput},
    domain,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,

    pub amount: Decimal,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Income,
    Expense,
}

impl From<TransactionType> for domain::enums::TransactionType {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Income => domain::enums::TransactionType::Income,
            TransactionType::Expense => domain::enums::TransactionType::Expense,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateTransactionResponse {
    pub id: Uuid,
}

impl From<CreateTransactionRequest> for CreateTransactionInput {
    fn from(req: CreateTransactionRequest) -> Self {
        Self {
            account_id: req.account_id,
            category_id: req.category_id,
            amount: req.amount,
            approved_at: req.approved_at,
            memo: req.memo,
            transaction_type: req.transaction_type.into(),
        }
    }
}

impl From<CreateTransactionOutput> for CreateTransactionResponse {
    fn from(res: CreateTransactionOutput) -> Self {
        Self { id: res.id }
    }
}
