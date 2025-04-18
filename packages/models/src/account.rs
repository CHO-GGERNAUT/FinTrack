use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]

pub struct Account {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub business_number: Option<String>,
    pub account_type: AccountType,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]

pub enum AccountType {
    Card,
    Bank,
}
