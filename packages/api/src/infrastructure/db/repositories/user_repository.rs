use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    domain::{
        entities::User,
        errors::{DomainError, UserError},
        repositories::UserRepository,
    },
    infrastructure::db::schema::UserRow,
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
    async fn create(&mut self, user: &User) -> Result<User, DomainError> {
        let tx = self.tx.as_mut();
        let result = sqlx::query_as!(
            UserRow,
            r#"
            INSERT INTO "users" (id, name, email, password)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            user.id,
            user.name,
            user.email,
            user.password,
        )
        .fetch_one(tx)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {}", e);
            UserError::Duplicate
        })?;

        Ok(result.into())
    }

    async fn find_by_email(&mut self, email: &str) -> Result<User, DomainError> {
        let tx = self.tx.as_mut();

        sqlx::query_as!(UserRow, r#"SELECT * FROM "users" WHERE email = $1"#, email)
            .fetch_optional(tx)
            .await
            .map_err(|e| {
                tracing::error!("DB error: {}", e);
                UserError::NotFound
            })?
            .map(Into::into)
            .ok_or(UserError::NotFound.into())
    }
}

pub struct UserRepositoryPostgresPool {
    pool: PgPool,
}

impl UserRepositoryPostgresPool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPostgresPool {
    async fn create(&mut self, user: &User) -> Result<User, DomainError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {}", e);
            UserError::Unknown(e.to_string())
        })?;
        let mut repo = UserRepositoryPostgres::new(&mut tx);
        let result = repo.create(user).await?;
        tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {}", e);
            UserError::Unknown(e.to_string())
        })?;
        Ok(result)
    }
    async fn find_by_email(&mut self, email: &str) -> Result<User, DomainError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {}", e);
            UserError::Unknown(e.to_string())
        })?;
        let mut repo = UserRepositoryPostgres::new(&mut tx);
        let result = repo.find_by_email(email).await?;
        tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {}", e);
            UserError::Unknown(e.to_string())
        })?;
        Ok(result)
    }
}
