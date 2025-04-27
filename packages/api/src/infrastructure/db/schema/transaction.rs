use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::domain;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TransactionRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub amount: Decimal, // Ensure all arithmetic operations use Decimal methods
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,

    pub transaction_type: TransactionTypeDb,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum TransactionTypeDb {
    Income,
    Expense,
}

impl std::fmt::Display for TransactionTypeDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionTypeDb::Income => write!(f, "income"),
            TransactionTypeDb::Expense => write!(f, "expense"),
        }
    }
}

impl From<domain::enums::TransactionType> for TransactionTypeDb {
    fn from(value: domain::enums::TransactionType) -> Self {
        match value {
            domain::enums::TransactionType::Income => Self::Income,
            domain::enums::TransactionType::Expense => Self::Expense,
        }
    }
}

impl From<TransactionTypeDb> for domain::enums::TransactionType {
    fn from(value: TransactionTypeDb) -> Self {
        match value {
            TransactionTypeDb::Income => Self::Income,
            TransactionTypeDb::Expense => Self::Expense,
        }
    }
}

impl From<TransactionRow> for domain::entities::Transaction {
    fn from(row: TransactionRow) -> Self {
        Self {
            id: row.id,
            account_id: row.account_id,
            category_id: row.category_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
            amount: row.amount,
            approved_at: row.approved_at,
            memo: row.memo,
            transaction_type: domain::enums::TransactionType::from(row.transaction_type),
        }
    }
}
