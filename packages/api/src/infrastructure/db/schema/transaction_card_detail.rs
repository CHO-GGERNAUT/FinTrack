use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TransactionCardDetailRow {
    pub transaction_id: Uuid,
    pub merchant_id: Option<Uuid>,
    pub installment_months: Option<i32>,
}
