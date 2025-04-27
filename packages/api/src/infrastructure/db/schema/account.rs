use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::domain;

#[derive(Debug, Serialize, Deserialize, FromRow)]

pub struct AccountRow {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub account_type: AccountTypeDb,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]

pub enum AccountTypeDb {
    Card,
    Bank,
}

impl From<AccountTypeDb> for domain::enums::AccountType {
    fn from(value: AccountTypeDb) -> Self {
        match value {
            AccountTypeDb::Card => Self::Card,
            AccountTypeDb::Bank => Self::Bank,
        }
    }
}

impl From<domain::enums::AccountType> for AccountTypeDb {
    fn from(value: domain::enums::AccountType) -> Self {
        match value {
            domain::enums::AccountType::Card => Self::Card,
            domain::enums::AccountType::Bank => Self::Bank,
        }
    }
}

impl From<AccountRow> for domain::entities::Account {
    fn from(row: AccountRow) -> Self {
        Self {
            id: row.id,
            owner_id: row.owner_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
            account_type: domain::enums::AccountType::from(row.account_type),
        }
    }
}
impl From<domain::entities::Account> for AccountRow {
    fn from(value: domain::entities::Account) -> Self {
        Self {
            id: value.id,
            owner_id: value.owner_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
            account_type: AccountTypeDb::from(value.account_type),
        }
    }
}
