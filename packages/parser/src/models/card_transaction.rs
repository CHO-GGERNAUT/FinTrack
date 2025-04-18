use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct CardTransaction {
    pub r#type: CardTransactionType,
    pub merchant: String,
    pub business_number: i64,
    pub amount: i64,
    pub approved_at: DateTime<FixedOffset>,
    pub card_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardTransactionType {
    Refund,
    LumpSum,
    Installment(u8),
}
