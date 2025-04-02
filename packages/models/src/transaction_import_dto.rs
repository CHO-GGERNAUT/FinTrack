use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::card_transactions::{TransactionType, to_fields};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionImportDto {
    pub amount: i32,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,

    pub card_num: String,
    pub business_number: String,

    pub transaction_type: String, // "LumpSum", "Installment", "Refund"
    pub installment_months: Option<u8>, // Installment일 경우만 Some
}

impl TransactionImportDto {
    pub fn new(
        amount: i32,
        approved_at: DateTime<Utc>,
        memo: Option<String>,
        card_num: String,
        business_number: String,
        transaction_type: TransactionType,
    ) -> Self {
        let (transaction_type, installment_months) = to_fields(&transaction_type);
        Self {
            amount,
            approved_at,
            memo,
            card_num,
            business_number,
            transaction_type,
            installment_months,
        }
    }
}
