use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl From<domain::entities::User> for UserRow {
    fn from(user: domain::entities::User) -> Self {
        Self {
            id: user.id,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: None,
            name: user.name,
            email: user.email,
            password: user.password,
        }
    }
}

impl From<UserRow> for domain::entities::User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            email: row.email,
            password: row.password,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
