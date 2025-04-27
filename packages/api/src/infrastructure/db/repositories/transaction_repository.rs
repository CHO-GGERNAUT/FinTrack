use crate::{
    domain::{
        entities::CardTransaction,
        errors::{DomainError, TransactionError},
        repositories::CardTransactionRepository,
    },
    infrastructure::db::schema::{TransactionCardDetailRow, TransactionRow, TransactionTypeDb},
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction as SqlxTransaction};

pub struct CardTransactionRepositoryPostgres<'a> {
    tx: &'a mut SqlxTransaction<'static, Postgres>,
}

impl<'a> CardTransactionRepositoryPostgres<'a> {
    pub fn new(tx: &'a mut SqlxTransaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'a> CardTransactionRepository for CardTransactionRepositoryPostgres<'a> {
    async fn create(
        &mut self,
        transaction: &CardTransaction,
    ) -> Result<CardTransaction, DomainError> {
        let tx = self.tx.as_mut();

        let transaction_row = sqlx::query_as!(
            TransactionRow,
            r#"
              INSERT INTO transactions (
                  id,
                  account_id,
                  user_id,
                  category_id,
                  created_at,
                  updated_at,
                  deleted_at,
                  amount,
                  approved_at,
                  memo,
                  transaction_type
              )
              VALUES (
                  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
              )
              RETURNING
                  id,
                  account_id,
                  user_id,
                  category_id,
                  created_at,
                  updated_at,
                  deleted_at,
                  amount,
                  approved_at,
                  memo,
                  transaction_type as "transaction_type:TransactionTypeDb"
          "#,
            transaction.id,
            transaction.account_id,
            transaction.user_id,
            transaction.category_id,
            transaction.created_at,
            transaction.updated_at,
            transaction.deleted_at,
            transaction.amount,
            transaction.approved_at,
            transaction.memo,
            TransactionTypeDb::from(transaction.transaction_type) as TransactionTypeDb,
        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create transaction: {}", e);
            TransactionError::Unknown(e.to_string())
        })?;

        let tx = self.tx.as_mut();

        let card_transaction_detail = sqlx::query_as!(
            TransactionCardDetailRow,
            r#"
            INSERT INTO transaction_card_detail (
                transaction_id,
                merchant_id,
                installment_months
            )
            VALUES (
                $1, $2, $3
            )
            RETURNING
                transaction_id,
                merchant_id,
                installment_months
          "#,
            transaction_row.id,
            transaction.merchant_id,
            transaction.installment_months
        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create transaction card detail: {}", e);
            TransactionError::Unknown(e.to_string())
        })?;

        Ok((transaction_row, card_transaction_detail).into())
    }
}
