use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::domain::{
    shared::value_objects::AuditInfo,
    user::{
        entities::User,
        value_objects::{Email, PhoneNumber, UserId, UserStatus},
    },
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub phone_number: String,
    pub status: UserStatusDb,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_status")]
pub enum UserStatusDb {
    Active,
    Inactive,
    PendingActivation,
    Suspended,
}

impl From<User> for UserRow {
    fn from(user: User) -> Self {
        Self {
            id: user.id().as_deref(),
            email: user.email().to_string(),
            phone_number: user.phone_number().as_str().to_string(),
            status: (*user.status()).into(),
            created_at: user.audit_info().created_at(),
            updated_at: user.audit_info().updated_at(),
            deleted_at: user.audit_info().deleted_at(),
        }
    }
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        let id = UserId::from(row.id);
        let email = Email::from_persistent(&row.email);

        let phone_number = PhoneNumber::from_persistent(&row.phone_number);
        let status = UserStatus::from(row.status);
        let audit_info = AuditInfo::from_persistent(row.created_at, row.updated_at, row.deleted_at);
        User::from_persistent(id, email, phone_number, status, audit_info)
    }
}
impl From<UserStatusDb> for UserStatus {
    fn from(status: UserStatusDb) -> Self {
        match status {
            UserStatusDb::Suspended => Self::Suspended,
            UserStatusDb::Active => Self::Active,
            UserStatusDb::Inactive => Self::Inactive,
            UserStatusDb::PendingActivation => Self::PendingActivation,
        }
    }
}
impl From<UserStatus> for UserStatusDb {
    fn from(status: UserStatus) -> Self {
        match status {
            UserStatus::Suspended => Self::Suspended,
            UserStatus::Active => Self::Active,
            UserStatus::Inactive => Self::Inactive,
            UserStatus::PendingActivation => Self::PendingActivation,
        }
    }
}
