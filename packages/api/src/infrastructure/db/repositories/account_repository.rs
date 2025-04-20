use crate::{
    domain::{
        entities::Account,
        errors::{DomainError, Result, account::AccountError},
        repositories::AccountRepository,
    },
    infrastructure::db::schema::{AccountRow, AccountTypeDb},
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub struct AccountRepositoryPostgres<'a> {
    tx: &'a mut Transaction<'static, Postgres>,
}

impl<'a> AccountRepositoryPostgres<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'a> AccountRepository for AccountRepositoryPostgres<'a> {
    async fn create(&mut self, account: &Account) -> Result<Account> {
        let tx = self.tx.as_mut();

        let row = sqlx::query_as!(
            AccountRow,
            r#"
            INSERT INTO account (id, owner_id, account_type)
            VALUES ($1, $2, $3)
            RETURNING 
                id,
                owner_id,
                created_at,
                updated_at,
                deleted_at,
                account_type as "account_type:AccountTypeDb"
            "#,
            account.id,
            account.owner_id,
            AccountTypeDb::Card as AccountTypeDb,
        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            DomainError::AccountError(AccountError::DuplicateAccount)
        })?;

        Ok(row.into())
    }

    async fn delete(&mut self, account_id: Uuid) -> Result<()> {
        let tx = self.tx.as_mut();

        sqlx::query!(
            r#"
            DELETE FROM account
            WHERE id = $1
            "#,
            account_id
        )
        .execute(tx)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            DomainError::AccountError(AccountError::NotFound)
        })?;

        Ok(())
    }

    async fn find_by_id(&mut self, account_id: Uuid) -> Result<Account> {
        let tx = self.tx.as_mut();

        let row = sqlx::query_as!(
            AccountRow,
            r#"
                SELECT
                    id,
                    owner_id,
                    created_at,
                    updated_at,
                    deleted_at,
                    account_type as "account_type:AccountTypeDb"
                FROM
                    account
                WHERE
                    id = $1
            "#,
            account_id
        )
        .fetch_optional(tx)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            DomainError::AccountError(AccountError::NotFound)
        })?
        .ok_or(DomainError::AccountError(AccountError::NotFound))?;

        Ok(row.into())
    }
}
