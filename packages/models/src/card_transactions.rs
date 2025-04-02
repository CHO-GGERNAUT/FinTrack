use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default)]
pub struct CardTransaction {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub amount: i32,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,

    pub card_id: Uuid,
    pub merchant_id: Uuid,
    pub category_id: Option<i32>,

    pub transaction_type: String,
    pub installment_months: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TransactionType {
    #[default]
    Refund,
    LumpSum,
    Installment(u8),
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Refund => write!(f, "취소"),
            TransactionType::LumpSum => write!(f, "일시불"),
            TransactionType::Installment(val) => write!(f, "({})개월 할부", val),
        }
    }
}

pub fn from_fields(typ: &str, months: Option<u8>) -> TransactionType {
    match typ {
        "Refund" => TransactionType::Refund,
        "LumpSum" => TransactionType::LumpSum,
        "Installment" => TransactionType::Installment(months.unwrap_or(0)),
        _ => TransactionType::Refund, // fallback
    }
}

pub fn to_fields(ttype: &TransactionType) -> (String, Option<u8>) {
    match ttype {
        TransactionType::Refund => ("Refund".into(), None),
        TransactionType::LumpSum => ("LumpSum".into(), None),
        TransactionType::Installment(months) => ("Installment".into(), Some(*months)),
    }
}
