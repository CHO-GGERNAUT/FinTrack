use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::enums::TransactionType;

#[derive(Debug)]
pub struct CreateTransactionInput {
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,

    pub amount: Decimal,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,
    pub transaction_type: TransactionType,
}

#[derive(Debug)]
pub struct CreateTransactionOutput {
    pub id: Uuid,
}
