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
    audit_info: AuditInfo,
    last_used_at: Option<DateTime<Utc>>,
    failed_attempts: u8,
}

const MAX_FAILED_ATTEMPTS: u8 = 5; // 예시: 최대 실패 횟수

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
        self.unlock_account(); // 비밀번호 변경 시 잠금 해제 및 실패 횟수 초기화
        self.audit_info.record_update(); // 감사 정보 업데이트
        Ok(())
    }
    pub fn reset_password(
        &mut self,
        new_raw_password: &str,
    ) -> Result<(), PasswordCredentialError> {
        self.password_hash = PasswordHash::new(new_raw_password)?;
        self.unlock_account(); // 비밀번호 재설정 시 잠금 해제 및 실패 횟수 초기화
        self.audit_info.record_update(); // 감사 정보 업데이트
        Ok(())
    }

    pub fn lock_account(&mut self) {
        if !self.is_locked {
            self.is_locked = true;
            self.audit_info.record_update();
        }
    }

    /// 계정을 수동으로 잠금 해제합니다. 실패 횟수도 초기화합니다.
    pub fn unlock_account(&mut self) {
        if self.is_locked || self.failed_attempts > 0 {
            self.is_locked = false;
            self.failed_attempts = 0;
            self.audit_info.record_update();
        }
    }
}

//Internal methods
impl PasswordCredential {
    /// 성공적인 로그인을 기록합니다.
    fn record_successful_login(&mut self) {
        self.failed_attempts = 0; // 실패 횟수 초기화
        self.last_used_at = Some(Utc::now());
        self.audit_info.record_update(); // 마지막 사용 시간 변경도 업데이트로 간주
    }

    /// 로그인 실패를 기록하고, 임계값 초과 시 계정을 잠급니다.
    fn record_failed_attempt(&mut self) {
        self.failed_attempts += 1;
        if self.failed_attempts >= MAX_FAILED_ATTEMPTS {
            self.lock_account(); // 자동 잠금
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

    // password_hash 는 직접 노출하지 않는 것이 좋을 수 있습니다.
    // pub fn password_hash(&self) -> &PasswordHash {
    //     &self.password_hash
    // }

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
