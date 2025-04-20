use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::{
    domain::{
        entities::User,
        errors::{DomainError, Result, user::UserError},
        repositories::UserRepository,
    },
    infrastructure::db::{ArcPgPool, schema::UserRow},
};

pub struct UserRepositoryPostgres<'a> {
    tx: &'a mut Transaction<'static, Postgres>,
}
impl<'a> UserRepositoryPostgres<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'a> UserRepository for UserRepositoryPostgres<'a> {
    async fn create(&mut self, user: &User) -> Result<User> {
        let tx = self.tx.as_mut();
        let result = sqlx::query_as!(
            UserRow,
            r#"
            INSERT INTO "user" (id, name, email, password)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            user.id,
            user.name,
            user.email,
            user.password,
        )
        .fetch_one(tx)
        .await;

        match result {
            Ok(row) => Ok(row.into()),
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.constraint() == Some("user_email_key") {
                        return Err(DomainError::UserError(UserError::EmailAlreadyExists));
                    }
                }
                tracing::error!("DB error: {}", e);
                Err(DomainError::UserError(UserError::UserNotCreated))
            }
        }
    }

    async fn find_by_email(&mut self, email: &str) -> Result<User> {
        let tx = self.tx.as_mut();

        sqlx::query_as!(UserRow, r#"SELECT * FROM "user" WHERE email = $1"#, email)
            .fetch_optional(tx)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {}", e);
                DomainError::UserError(UserError::UserNotFound)
            })?
            .map(Into::into)
            .ok_or(DomainError::UserError(UserError::UserNotFound))
    }
}

pub struct UserRepositoryPostgresPool {
    pool: ArcPgPool,
}

impl UserRepositoryPostgresPool {
    pub fn new(pool: ArcPgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPostgresPool {
    async fn create(&mut self, user: &User) -> Result<User> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {}", e);
            DomainError::UserError(UserError::EmailAlreadyExists)
        })?;
        let mut repo = UserRepositoryPostgres::new(&mut tx);
        let result = repo.create(user).await?;
        tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {}", e);
            DomainError::UserError(UserError::EmailAlreadyExists)
        })?;
        Ok(result)
    }
    async fn find_by_email(&mut self, email: &str) -> Result<User> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {}", e);
            DomainError::UserError(UserError::EmailAlreadyExists)
        })?;
        let mut repo = UserRepositoryPostgres::new(&mut tx);
        let result = repo.find_by_email(email).await?;
        tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {}", e);
            DomainError::UserError(UserError::EmailAlreadyExists)
        })?;
        Ok(result)
    }
}
