use bcrypt::hash;
use chrono::{DateTime, Utc};

use super::super::errors::CredentialError;

#[derive(Debug, Clone)]
pub struct Password {
    password_hash: String,
    failed_attempt_count: u8,
    password_changed_at: DateTime<Utc>,
}

impl Password {
    pub fn new(password: &str) -> Result<Self, CredentialError> {
        let password_hash = Self::hash_password(&password)?;
        Ok(Password {
            password_hash,
            failed_attempt_count: 0,
            password_changed_at: Utc::now(),
        })
    }

    pub fn change_password(
        &mut self,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), CredentialError> {
        if self.verify_password(current_password).is_err() {
            self.record_failed_attempt();
            return Err(CredentialError::InvalidCredentials);
        }

        self.password_hash = Self::hash_password(new_password)?;
        self.update_password_changed_at();
        self.reset_failed_attempts();
        Ok(())
    }

    pub fn verify_password(&self, provided_password: &str) -> Result<(), CredentialError> {
        if self.failed_attempt_count >= 5 {
            return Err(CredentialError::AccountLocked);
        }
        let is_valid =
            bcrypt::verify(provided_password, &self.password_hash).is_ok_and(|is_valid| is_valid);
        if is_valid {
            Ok(())
        } else {
            Err(CredentialError::InvalidCredentials)
        }
    }

    fn record_failed_attempt(&mut self) {
        self.failed_attempt_count += 1;
    }

    fn reset_failed_attempts(&mut self) {
        self.failed_attempt_count = 0;
    }

    fn update_password_changed_at(&mut self) {
        self.password_changed_at = Utc::now();
    }

    fn hash_password(password: &str) -> Result<String, CredentialError> {
        hash(password, bcrypt::DEFAULT_COST).map_err(|e| CredentialError::HashFailed(e.to_string()))
    }
}
