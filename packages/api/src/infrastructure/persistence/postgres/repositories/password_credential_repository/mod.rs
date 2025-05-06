pub mod model;
use chrono::Utc;
use model::PasswordCredentialRow;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    application::errors::RepositoryError,
    domain::{
        password_credential::{
            entities::PasswordCredential, repository::PasswordCredentialRepository,
        },
        user::value_objects::UserId,
    },
};

pub static ENTITY_TYPE: &str = "PasswordCredential";
pub struct PasswordCredentialRepositoryPg<'a> {
    tx: &'a mut Transaction<'static, Postgres>,
}
impl<'a> PasswordCredentialRepositoryPg<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl<'a> PasswordCredentialRepository for PasswordCredentialRepositoryPg<'a> {
    async fn create(
        &mut self,
        credential: PasswordCredential,
    ) -> Result<PasswordCredential, RepositoryError> {
        let tx = self.tx.as_mut();

        let result = sqlx::query_as!(
            PasswordCredentialRow,
            r#"
                INSERT INTO password_credentials (id, user_id, password_hash, is_locked, last_used_at, failed_attempts, created_at, updated_at, deleted_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                RETURNING id, user_id, password_hash, is_locked, last_used_at, failed_attempts as "failed_attempts:i32", created_at, updated_at, deleted_at
            "#,
            Into::<Uuid>::into(*credential.id()),
            Into::<Uuid>::into(*credential.user_id()),
            credential.password_hash().as_str(),
            credential.is_locked(),
            credential.last_used_at(),
            credential.failed_attempts() as i32,

            credential.audit_info().created_at(),
            credential.audit_info().updated_at(),
            credential.audit_info().deleted_at(),

        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            RepositoryError::Conflict { entity_type: ENTITY_TYPE, details: e.to_string() }
        })?;
        Ok(result
            .try_into()
            .map_err(|e| RepositoryError::invalid_data(e))?)
    }

    async fn find_by_user_id(
        &mut self,
        user_id: UserId,
    ) -> Result<PasswordCredential, RepositoryError> {
        let tx = self.tx.as_mut();
        let result = sqlx::query_as!(
            PasswordCredentialRow,
            r#"
                SELECT id, user_id, password_hash, is_locked, last_used_at, failed_attempts as "failed_attempts:i32", created_at, updated_at, deleted_at
                FROM password_credentials
                WHERE user_id = $1 AND deleted_at IS NULL
            "#,
            user_id.as_deref(),
        ).fetch_one(tx).await.map_err(|e| {
            tracing::error!("DB error: {}", e);
            RepositoryError::NotFound { entity_type: ENTITY_TYPE, id: user_id.as_deref().to_string() }
        })?;

        Ok(result
            .try_into()
            .map_err(|e| RepositoryError::invalid_data(e))?)
    }

    async fn update(&mut self, credential: PasswordCredential) -> Result<(), RepositoryError> {
        let tx = self.tx.as_mut();

        sqlx::query_as!(
            PasswordCredentialRow,
            r#"
                UPDATE password_credentials
                SET password_hash = $1, is_locked = $2, last_used_at = $3, failed_attempts = $4, updated_at = $5
                WHERE id = $6 AND deleted_at IS NULL
            "#,
            credential.password_hash().as_str(),
            credential.is_locked(),
            credential.last_used_at(),
            credential.failed_attempts() as i32,
            Utc::now(),
            Into::<Uuid>::into(*credential.id()),
        )
        .execute(tx)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            RepositoryError::Conflict { entity_type: ENTITY_TYPE, details: e.to_string() }
        })?;

        Ok(())
    }
}
