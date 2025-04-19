use std::sync::Arc;

use crate::domain::{entities::user::User, repositories::user_repository::UserRepository};
use models::schema::user::User as UserRow;

use async_trait::async_trait;

#[derive(Clone)]
pub struct UserRepositoryPostgres {
    pub pool: Arc<sqlx::PgPool>,
}

#[async_trait]
impl UserRepository for UserRepositoryPostgres {
    async fn save(&self, user: User) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO \"user\" (id, name, email, password) VALUES ($1, $2, $3, $4)",
            user.id,
            user.name,
            user.email,
            user.password,
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let row = sqlx::query_as!(UserRow, "SELECT * FROM \"user\" WHERE email = $1", email)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(row.map(|r| r.into()))
    }
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            email: row.email,
            password: row.password,
        }
    }
}
