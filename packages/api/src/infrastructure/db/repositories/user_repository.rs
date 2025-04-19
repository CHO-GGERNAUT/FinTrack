use crate::{
    domain::{DomainError, Result, entities::User, repositories::user_repository::UserRepository},
    infrastructure::db::{ArcPgPool, schema},
};

use async_trait::async_trait;

#[derive(Clone)]
pub struct UserRepositoryPostgres {
    pool: ArcPgPool,
}

impl UserRepositoryPostgres {
    pub fn new(pool: ArcPgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPostgres {
    async fn save(&self, user: User) -> Result<User> {
        let user = sqlx::query_as!(
            schema::User,
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
        .fetch_one(&*self.pool)
        .await;
        match user {
            Ok(user) => Ok(user.into()),
            Err(e) => Err(DomainError::RepositoryError(e.to_string())),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<User> {
        let row = sqlx::query_as!(
            schema::User,
            "SELECT * FROM \"user\" WHERE email = $1",
            email
        )
        .fetch_optional(&*self.pool)
        .await;
        match row {
            Ok(Some(user)) => Ok(user.into()),
            _ => Err(DomainError::RepositoryError(format!(
                "User with email {} not found",
                email
            ))),
        }
    }
}

impl From<schema::User> for User {
    fn from(row: schema::User) -> Self {
        User {
            id: row.id,
            name: row.name,
            email: row.email,
            password: row.password,
        }
    }
}
