use crate::domain::{
    credential::value_objects::Password, shared::value_objects::AuditInfo,
    user::value_objects::UserId,
};

use super::super::{
    errors::CredentialError,
    value_objects::{CredentialDetail, CredentialId},
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Credential {
    id: CredentialId,
    user_id: UserId,
    details: CredentialDetail,

    is_locked: bool,
    audit_info: AuditInfo,
    last_used_at: Option<DateTime<Utc>>,
}

// External methods
impl Credential {
    pub fn new_password_auth(user_id: UserId, raw_password: &str) -> Result<Self, CredentialError> {
        let password = Password::new(raw_password)?;
        Ok(Credential {
            id: CredentialId::new(),
            user_id,
            details: CredentialDetail::Password(password),

            is_locked: false,
            audit_info: AuditInfo::record_creation(),
            last_used_at: None,
        })
    }

    pub fn verify_password(&mut self, password: &str) -> Result<(), CredentialError> {
        match &self.details {
            CredentialDetail::Password(password_vo) => {
                if let Err(err) = password_vo.verify_password(password) {
                    if err == CredentialError::AccountLocked {
                        self.lock_account();
                    }
                    return Err(err);
                } else {
                    self.record_successful_login();
                    Ok(())
                }
            } // _ => Err(CredentialError::InvalidCredentialType),
        }
    }
}

//Internal methods
impl Credential {
    fn record_successful_login(&mut self) {
        if self.is_locked {
            return;
        }
        self.last_used_at = Some(Utc::now());
    }

    #[allow(dead_code)]
    fn lock_account(&mut self) {
        self.is_locked = true;
    }
}

// Getters
impl Credential {
    pub fn id(&self) -> &CredentialId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn details(&self) -> &CredentialDetail {
        &self.details
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    pub fn audit_info(&self) -> &AuditInfo {
        &self.audit_info
    }

    pub fn last_used_at(&self) -> Option<DateTime<Utc>> {
        self.last_used_at
    }
}
