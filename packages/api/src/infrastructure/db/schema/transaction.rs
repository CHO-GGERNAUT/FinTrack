use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::domain;

use super::TransactionCardDetailRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TransactionRow {
    pub id: Uuid,
    pub user_id: Uuid,
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

impl From<(TransactionRow, TransactionCardDetailRow)> for domain::entities::CardTransaction {
    fn from((transaction, card_detail): (TransactionRow, TransactionCardDetailRow)) -> Self {
        Self {
            id: transaction.id,
            user_id: transaction.user_id,
            merchant_id: card_detail.merchant_id,
            installment_months: card_detail.installment_months.unwrap_or_default(),
            account_id: transaction.account_id,
            category_id: transaction.category_id,
            created_at: transaction.created_at,
            updated_at: transaction.updated_at,
            deleted_at: transaction.deleted_at,
            amount: transaction.amount,
            approved_at: transaction.approved_at,
            memo: transaction.memo,
            transaction_type: domain::enums::TransactionType::from(transaction.transaction_type),
        }
    }
}
