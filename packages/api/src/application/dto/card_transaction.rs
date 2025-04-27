use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::enums::TransactionType;

use super::CreateMerchantInput;

#[derive(Debug)]
pub struct CreateCardTransactionInput {
    pub user_id: Uuid,
    pub card_id: Uuid,
    pub category_id: Option<Uuid>,
    pub merchant: MerchantInput,
    pub amount: Decimal,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,
    pub transaction_type: TransactionType,
    pub installment_months: Option<i32>,
}

#[derive(Debug)]
pub struct CreateCardTransactionOutput {
    pub transaction_id: Uuid,
    pub merchant_id: Uuid,
}

#[derive(Debug)]
pub enum MerchantInput {
    ById(Uuid),
    ByInfo(CreateMerchantInput),
}
