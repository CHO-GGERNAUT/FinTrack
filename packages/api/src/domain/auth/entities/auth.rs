use super::super::{errors::AuthError, value_objects::AuthProvider};
use crate::domain::user::value_objects::UserId;
use bcrypt::hash;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Auth {
    user_id: UserId,
    provider: AuthProvider,
    password_hash: String,
    last_login_at: Option<DateTime<Utc>>,
    failed_attempt_count: u8,
    is_locked: bool,
    password_changed_at: DateTime<Utc>,
}

// External methods
impl Auth {
    pub fn new(
        user_id: UserId,
        raw_password: &str,
        provider: AuthProvider,
    ) -> Result<Self, AuthError> {
        let password_hash = Self::hash_password(raw_password)?;

        Ok(Auth {
            user_id,
            provider,
            password_hash,
            last_login_at: None,
            failed_attempt_count: 0,
            is_locked: false,
            password_changed_at: Utc::now(),
        })
    }

    pub fn change_password(
        &mut self,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), AuthError> {
        if self.is_locked {
            return Err(AuthError::AccountLocked);
        }
        if !self.verify_password(current_password) {
            self.record_failed_attempt();
            return Err(AuthError::InvalidCredentials);
        }

        self.password_hash = Self::hash_password(new_password);
        self.update_password_changed_at();
        self.reset_failed_attempts();
        Ok(())
    }

    pub fn verify_password(&self, provided_password: &str) -> bool {
        if self.is_locked {
            return false;
        }
        let is_valid =
            bcrypt::verify(provided_password, &self.password_hash).is_ok_and(|is_valid| is_valid);
        if is_valid {
            Ok(())
        } else {
            Err(AuthError::InvalidCredentials)
        }
    }
}

//Internal methods
impl Auth {
    fn record_successful_login(&mut self) {
        if self.is_locked {
            return;
        }
        self.last_login_at = Some(Utc::now());
        self.reset_failed_attempts();
    }

    fn record_failed_attempt(&mut self) {
        if self.is_locked {
            return;
        }
        self.failed_attempt_count += 1;
        if self.failed_attempt_count >= 5 {
            self.lock_account();
        }
    }

    fn lock_account(&mut self) {
        self.is_locked = true;
    }

    fn update_password_changed_at(&mut self) {
        self.password_changed_at = Utc::now();
    }
    fn reset_failed_attempts(&mut self) {
        self.failed_attempt_count = 0;
    }

    fn hash_password(password: &str) -> Result<String, AuthError> {
        hash(password, bcrypt::DEFAULT_COST).map_err(|e| AuthError::HashFailed(e.into()))
    }
}

// Getters
impl Auth {
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn last_login_at(&self) -> Option<DateTime<Utc>> {
        self.last_login_at
    }

    pub fn failed_attempt_count(&self) -> u8 {
        self.failed_attempt_count
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    pub fn password_changed_at(&self) -> DateTime<Utc> {
        self.password_changed_at
    }
}
