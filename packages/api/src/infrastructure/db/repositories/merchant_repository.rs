use crate::{
    domain::{
        entities::Merchant,
        errors::{DomainError, MerchantError},
        repositories::MerchantRepository,
    },
    infrastructure::db::schema::MerchantRow,
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

pub struct MerchantRepositoryPostgres<'a> {
    tx: &'a mut Transaction<'static, Postgres>,
}

impl<'a> MerchantRepositoryPostgres<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'a> MerchantRepository for MerchantRepositoryPostgres<'a> {
    async fn create(&mut self, merchant: &Merchant) -> Result<Merchant, DomainError> {
        let tx = self.tx.as_mut();

        let row = sqlx::query_as!(
            MerchantRow,
            r#"
              INSERT INTO merchants (
                  id,
                  name,
                  biz_number,
                  address,
                  phone
              )
              VALUES (
                  $1, $2, $3, $4, $5
              )
              RETURNING
                  id,
                  created_at,
                  updated_at,
                  deleted_at,
                  name,
                  biz_number,
                  address,
                  phone
          "#,
            merchant.id,
            merchant.name,
            merchant.biz_number,
            merchant.address,
            merchant.phone
        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save card: {}", e);
            MerchantError::Duplicate
        })?;

        Ok(row.into())
    }

    async fn find_by_biz_number(&mut self, biz_number: &str) -> Result<Merchant, DomainError> {
        let tx = self.tx.as_mut();

        let row = sqlx::query_as!(
            MerchantRow,
            r#"
              SELECT
                  id,
                  created_at,
                  updated_at,
                  deleted_at,
                  name,
                  biz_number,
                  address,
                  phone
              FROM merchants
              WHERE biz_number = $1
          "#,
            biz_number
        )
        .fetch_optional(tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find merchant by biz_number: {}", e);
            MerchantError::NotFound
        })?;
        if let Some(row) = row {
            Ok(row.into())
        } else {
            Err(MerchantError::NotFound.into())
        }
    }
}
