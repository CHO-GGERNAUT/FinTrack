use crate::domain::{shared::value_objects::AuditInfo, user::value_objects::UserId};

use super::super::{
    errors::PasswordCredentialError,
    value_objects::{PasswordCredentialId, PasswordHash},
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PasswordCredential {
    id: PasswordCredentialId,
    user_id: UserId,
    password_hash: PasswordHash,
    is_locked: bool,
    last_used_at: Option<DateTime<Utc>>,
    failed_attempts: u8,
    audit_info: AuditInfo,
}

const MAX_FAILED_ATTEMPTS: u8 = 5;

// External methods
impl PasswordCredential {
    pub fn new(user_id: UserId, plane_password: &str) -> Result<Self, PasswordCredentialError> {
        let password_hash = PasswordHash::new(plane_password)?;
        Ok(Self {
            id: PasswordCredentialId::new(),
            user_id,
            password_hash,

            is_locked: false,
            audit_info: AuditInfo::record_creation(),
            last_used_at: None,
            failed_attempts: 0,
        })
    }
}

impl PasswordCredential {
    pub fn verify_password(&mut self, password: &str) -> Result<(), PasswordCredentialError> {
        if self.is_locked {
            return Err(PasswordCredentialError::AccountLocked);
        }

        if self.password_hash.verify(password).is_ok() {
            self.record_successful_login();
            Ok(())
        } else {
            self.record_failed_attempt();
            Err(PasswordCredentialError::InvalidCredentials)
        }
    }

    pub fn change_password(
        &mut self,
        current_password: &str,
        new_raw_password: &str,
    ) -> Result<(), PasswordCredentialError> {
        if self.verify_password(current_password).is_err() {
            self.record_failed_attempt();
            return Err(PasswordCredentialError::InvalidCredentials);
        }

        self.password_hash.update_hash(new_raw_password)?;
        self.unlock_account();
        self.audit_info.record_update();
        Ok(())
    }
    pub fn reset_password(
        &mut self,
        new_raw_password: &str,
    ) -> Result<(), PasswordCredentialError> {
        self.password_hash = PasswordHash::new(new_raw_password)?;
        self.unlock_account();
        self.audit_info.record_update();
        Ok(())
    }

    pub fn lock_account(&mut self) {
        if !self.is_locked {
            self.is_locked = true;
            self.audit_info.record_update();
        }
    }

    pub fn unlock_account(&mut self) {
        if self.is_locked || self.failed_attempts > 0 {
            self.is_locked = false;
            self.failed_attempts = 0;
            self.audit_info.record_update();
        }
    }

    pub fn from_persistent(
        id: PasswordCredentialId,
        user_id: UserId,
        password_hash: PasswordHash,
        is_locked: bool,
        last_used_at: Option<DateTime<Utc>>,
        failed_attempts: u8,
        audit_info: AuditInfo,
    ) -> Self {
        Self {
            id,
            user_id,
            password_hash,
            is_locked,
            last_used_at,
            failed_attempts,
            audit_info,
        }
    }
}

//Internal methods
impl PasswordCredential {
    fn record_successful_login(&mut self) {
        self.failed_attempts = 0;
        self.last_used_at = Some(Utc::now());
        self.audit_info.record_update();
    }

    fn record_failed_attempt(&mut self) {
        self.failed_attempts += 1;
        if self.failed_attempts >= MAX_FAILED_ATTEMPTS {
            self.lock_account();
        }
        self.audit_info.record_update();
    }
}

// Getters
impl PasswordCredential {
    pub fn id(&self) -> &PasswordCredentialId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
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

    pub fn failed_attempts(&self) -> u8 {
        self.failed_attempts
    }
}
