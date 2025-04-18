use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub amount: i32,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,

    pub merchant_id: Uuid,
    pub category_id: Option<i32>,

    pub transaction_type: TransactionType,
    pub installment_months: Option<i32>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTransactionDto {
    pub account_id: Uuid,
    pub amount: i32,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,
    pub merchant_id: Uuid,
    pub category_id: Option<i32>,
    pub transaction_type: TransactionType,
    pub installment_months: Option<i32>,
}
pub struct CreateTransactionDtoBuilder {
    account_id: Option<Uuid>,
    amount: Option<i32>,
    approved_at: Option<DateTime<Utc>>,
    memo: Option<String>,
    merchant_id: Option<Uuid>,
    category_id: Option<i32>,
    transaction_type: Option<TransactionType>,
    installment_months: Option<i32>,
}

impl CreateTransactionDtoBuilder {
    pub fn new() -> Self {
        Self {
            account_id: None,
            amount: None,
            approved_at: None,
            memo: None,
            merchant_id: None,
            category_id: None,
            transaction_type: None,
            installment_months: None,
        }
    }

    pub fn account_id(mut self, account_id: Uuid) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn amount(mut self, amount: i32) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn approved_at(mut self, approved_at: DateTime<Utc>) -> Self {
        self.approved_at = Some(approved_at);
        self
    }

    pub fn memo(mut self, memo: Option<String>) -> Self {
        self.memo = memo;
        self
    }

    pub fn merchant_id(mut self, merchant_id: Uuid) -> Self {
        self.merchant_id = Some(merchant_id);
        self
    }

    pub fn category_id(mut self, category_id: Option<i32>) -> Self {
        self.category_id = category_id;
        self
    }

    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }

    pub fn installment_months(mut self, installment_months: Option<i32>) -> Self {
        self.installment_months = installment_months;
        self
    }

    pub fn build(self) -> Result<CreateTransactionDto, &'static str> {
        let account_id = self.account_id.ok_or("account_id is required")?;
        let amount = self.amount.ok_or("amount is required")?;
        let approved_at = self.approved_at.ok_or("approved_at is required")?;
        let merchant_id = self.merchant_id.ok_or("merchant_id is required")?;
        let transaction_type = self
            .transaction_type
            .ok_or("transaction_type is required")?;

        Ok(CreateTransactionDto {
            account_id,
            amount,
            approved_at,
            memo: self.memo,
            merchant_id,
            category_id: self.category_id,
            transaction_type,
            installment_months: self.installment_months,
        })
    }
}
