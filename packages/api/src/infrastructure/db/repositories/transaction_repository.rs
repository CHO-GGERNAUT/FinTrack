use crate::{
    domain::{
        entities::Transaction,
        errors::{DomainError, TransactionError},
        repositories::TransactionRepository,
    },
    infrastructure::db::schema::{TransactionRow, TransactionTypeDb},
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction as SqlxTransaction};

pub struct TransactionRepositoryPostgres<'a> {
    tx: &'a mut SqlxTransaction<'static, Postgres>,
}

impl<'a> TransactionRepositoryPostgres<'a> {
    pub fn new(tx: &'a mut SqlxTransaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'a> TransactionRepository for TransactionRepositoryPostgres<'a> {
    async fn create(&mut self, transaction: &Transaction) -> Result<Transaction, DomainError> {
        let tx = self.tx.as_mut();

        let row = sqlx::query_as!(
            TransactionRow,
            r#"
              INSERT INTO transaction (
                  id,
                  account_id,
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
                  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
              )
              RETURNING
                  id,
                  account_id,
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

        Ok(row.into())
    }
}
