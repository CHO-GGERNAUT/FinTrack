use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    domain::{
        password_credential::{entities::PasswordCredential, value_objects::PasswordHash},
        shared::value_objects::AuditInfo,
    },
    infrastructure::persistence::errors::InfraError,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PasswordCredentialRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub password_hash: String,
    pub is_locked: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub failed_attempts: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<PasswordCredential> for PasswordCredentialRow {
    fn from(credential: PasswordCredential) -> Self {
        Self {
            id: credential.id().as_deref(),
            user_id: credential.user_id().as_deref(),
            password_hash: credential.password_hash().as_str().to_string(),
            is_locked: credential.is_locked(),
            last_used_at: credential.last_used_at(),
            failed_attempts: credential.failed_attempts() as i32,
            created_at: credential.audit_info().created_at(),
            updated_at: credential.audit_info().updated_at(),
            deleted_at: credential.audit_info().deleted_at(),
        }
    }
}

impl TryFrom<PasswordCredentialRow> for PasswordCredential {
    type Error = InfraError;
    fn try_from(row: PasswordCredentialRow) -> Result<Self, Self::Error> {
        let password_hash = PasswordHash::try_from(row.password_hash)
            .map_err(|e| InfraError::ReconstituteFailed(format!("{}", e)))?;
        let is_locked = row.is_locked;
        let last_used_at = row.last_used_at;
        let failed_attempts = row.failed_attempts as u8;
        let audit_info = AuditInfo::new(row.created_at, row.updated_at, row.deleted_at);

        Ok(PasswordCredential::reconstitute(
            row.id.into(),
            row.user_id.into(),
            password_hash,
            is_locked,
            last_used_at,
            failed_attempts,
            audit_info,
        ))
    }
}
