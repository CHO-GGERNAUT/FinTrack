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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTransactionDto {
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,

    pub amount: f32,
    pub approved_at: DateTime<Utc>,
    pub memo: Option<String>,

    pub transaction_type: TransactionType,
}
pub struct CreateTransactionDtoBuilder {
    account_id: Option<Uuid>,
    category_id: Option<Uuid>,

    amount: Option<f32>,
    approved_at: Option<DateTime<Utc>>,
    memo: Option<String>,
    transaction_type: Option<TransactionType>,
}

impl CreateTransactionDtoBuilder {
    pub fn new() -> Self {
        Self {
            account_id: None,
            amount: None,
            approved_at: None,
            memo: None,
            category_id: None,
            transaction_type: None,
        }
    }

    pub fn account_id(mut self, account_id: Uuid) -> Self {
        self.account_id = Some(account_id);
        self
    }
    pub fn category_id(mut self, category_id: Uuid) -> Self {
        self.category_id = Some(category_id);
        self
    }
    pub fn amount(mut self, amount: f32) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn approved_at(mut self, approved_at: DateTime<Utc>) -> Self {
        self.approved_at = Some(approved_at);
        self
    }
    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = Some(transaction_type);
        self
    }

    pub fn build(self) -> Result<CreateTransactionDto, &'static str> {
        Ok(CreateTransactionDto {
            account_id: self.account_id.ok_or("account_id is required")?,
            category_id: self.category_id,
            amount: self.amount.ok_or("amount is required")?,
            approved_at: self.approved_at.ok_or("approved_at is required")?,
            memo: self.memo,
            transaction_type: self
                .transaction_type
                .ok_or("transaction_type is required")?,
        })
    }
}
