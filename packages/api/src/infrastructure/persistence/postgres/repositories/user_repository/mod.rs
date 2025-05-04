pub mod model;
use chrono::Utc;
use model::{UserRow, UserStatusDb};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    application::errors::RepositoryError,
    domain::user::{
        entities::User,
        repository::UserRepository,
        value_objects::{Email, UserId},
    },
};

pub static ENTITY_TYPE: &str = "User";
pub struct UserRepositoryPg<'a> {
    tx: &'a mut Transaction<'static, Postgres>,
}
impl<'a> UserRepositoryPg<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl<'a> UserRepository for UserRepositoryPg<'a> {
    async fn create(&mut self, user: User) -> Result<User, RepositoryError> {
        let tx = self.tx.as_mut();

        let result = sqlx::query_as!(
            UserRow,
            r#"
                INSERT INTO "users" (id, email, phone_number, status, created_at, updated_at, deleted_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, email, phone_number, status as "status:UserStatusDb", created_at, updated_at, deleted_at
            "#,
            Into::<Uuid>::into(*user.id()),
            user.email().as_str(),
            user.phone_number().as_str(),
            Into::<UserStatusDb>::into(*user.status())  as UserStatusDb,
            user.audit_info().created_at(),
            user.audit_info().updated_at(),
            user.audit_info().deleted_at(),
        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            RepositoryError::Conflict { entity_type: ENTITY_TYPE, details: e.to_string() }
        })?;
        Ok(result.into())
    }

    async fn find_by_email(&mut self, email: &Email) -> Result<User, RepositoryError> {
        let tx = self.tx.as_mut();
        let result = sqlx::query_as!(
            UserRow,
            r#"
                SELECT id, email, phone_number, status as "status:UserStatusDb", created_at, updated_at, deleted_at
                FROM "users"
                WHERE email = $1 AND deleted_at IS NULL
            "#,
            email.as_str(),
        ).fetch_optional(tx).await.map_err(|e| {
            tracing::error!("DB error: {}", e);
            RepositoryError::NotFound { entity_type: ENTITY_TYPE, id: email.as_str().to_string() }
        })?;
        if result.is_none() {
            tracing::error!("User not found with email: {}", email.as_str());
            return Err(RepositoryError::NotFound {
                entity_type: ENTITY_TYPE,
                id: email.as_str().to_string(),
            });
        }
        Ok(result.unwrap().into())
    }

    async fn find_by_id(&mut self, id: UserId) -> Result<User, RepositoryError> {
        let tx = self.tx.as_mut();
        let id: Uuid = id.into();

        let result = sqlx::query_as!(
            UserRow,
            r#"
                SELECT id, email, phone_number, status as "status:UserStatusDb", created_at, updated_at, deleted_at
                FROM "users"
                WHERE id = $1 AND deleted_at IS NULL
            "#,
            id,
        ).fetch_one(tx).await.map_err(|e| {
            tracing::error!("DB error: {}", e);
            RepositoryError::NotFound { entity_type: ENTITY_TYPE, id: id.to_string() }
        })?;
        Ok(result.into())
    }
    async fn update(&mut self, user: User) -> Result<User, RepositoryError> {
        let tx = self.tx.as_mut();
        let result = sqlx::query_as!(
            UserRow,
            r#"
                UPDATE "users"
                SET email = $1, phone_number = $2, status = $3, updated_at = $4
                WHERE id = $5 AND deleted_at IS NULL
                RETURNING id, email, phone_number, status as "status:UserStatusDb", created_at, updated_at, deleted_at
            "#,
            user.email().as_str(),
            user.phone_number().as_str(),
            Into::<UserStatusDb>::into(*user.status()) as UserStatusDb,
            user.audit_info().updated_at(),
            Into::<Uuid>::into(*user.id()),
        ).fetch_one(tx).await.map_err(|e| {
            tracing::error!("DB error: {}", e);
            RepositoryError::NotFound { entity_type: ENTITY_TYPE, id: user.id().to_string() }
        })?;
        Ok(result.into())
    }
    async fn delete(&mut self, id: UserId) -> Result<bool, RepositoryError> {
        let tx = self.tx.as_mut();
        let id: Uuid = id.into();
        let now = Utc::now();
        let result = sqlx::query!(
            r#"
                UPDATE "users"
                SET deleted_at = $1
                WHERE id = $2 AND deleted_at IS NULL 
            "#,
            now,
            id,
        )
        .execute(tx)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            RepositoryError::NotFound {
                entity_type: ENTITY_TYPE,
                id: id.to_string(),
            }
        })?;
        Ok(result.rows_affected() > 0)
    }
}
