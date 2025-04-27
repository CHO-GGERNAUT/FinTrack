use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::dto::{
        CreateCardTransactionInput, CreateCardTransactionOutput, CreateMerchantInput, MerchantInput,
    },
    domain,
};

use super::merchant::CreateMerchantRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCardTransactionRequest {
    pub category_id: Option<Uuid>,

    pub amount: Decimal,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,
    pub transaction_type: TransactionType,
    pub installment_months: Option<i32>,

    pub merchant_id: Option<Uuid>,
    pub merchant: Option<CreateMerchantRequest>,
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

impl TryFrom<(CreateCardTransactionRequest, Uuid, Uuid)> for CreateCardTransactionInput {
    type Error = String;

    fn try_from(
        (req, card_id, user_id): (CreateCardTransactionRequest, Uuid, Uuid),
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            card_id,
            category_id: req.category_id,
            user_id,
            amount: req.amount,
            approved_at: req.approved_at,
            memo: req.memo,
            transaction_type: req.transaction_type.into(),
            installment_months: req.installment_months,
            merchant: if let Some(merchant_id) = req.merchant_id {
                MerchantInput::ById(merchant_id)
            } else if let Some(merchant) = req.merchant {
                MerchantInput::ByInfo(CreateMerchantInput {
                    name: merchant.name,
                    biz_number: merchant.biz_number,
                    address: merchant.address,
                    phone: merchant.phone,
                })
            } else {
                return Err("Either merchant_id or merchant must be provided".to_string());
            },
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CreateCardTransactionResponse {
    pub transaction_id: Uuid,
    pub merchant_id: Uuid,
}

impl From<CreateCardTransactionOutput> for CreateCardTransactionResponse {
    fn from(res: CreateCardTransactionOutput) -> Self {
        Self {
            transaction_id: res.transaction_id,
            merchant_id: res.merchant_id,
        }
    }
}
