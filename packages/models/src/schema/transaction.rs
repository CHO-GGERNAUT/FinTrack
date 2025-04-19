use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub amount: f32,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,

    pub transaction_type: TransactionType,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[sqlx(type_name = "TEXT")]
pub enum TransactionType {
    Income,
    Expense,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Income => write!(f, "income"),
            TransactionType::Expense => write!(f, "expense"),
        }
    }
}
