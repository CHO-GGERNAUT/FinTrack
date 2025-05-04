use super::super::errors::UserError;
use super::super::value_objects::{Email, PhoneNumber, UserId, UserStatus};
use crate::domain::shared::value_objects::AuditInfo;

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    email: Email,
    audit_info: AuditInfo,
    phone_number: PhoneNumber,
    status: UserStatus,
}

impl User {
    pub fn register(email: Email, phone_number: PhoneNumber) -> Self {
        let id = UserId::new();
        let audit_info = AuditInfo::record_creation();
        Self {
            id,
            audit_info,
            email,
            phone_number,
            status: UserStatus::PendingActivation,
        }
    }

    pub fn activate(&mut self) -> Result<(), UserError> {
        if matches!(self.status, UserStatus::PendingActivation) {
            self.status = UserStatus::Active;
            self.audit_info.record_update();
            Ok(())
        } else {
            Err(UserError::InvalidUserStatus(
                "Cannot activate user in current status".to_string(),
            ))
        }
    }

    pub fn deactivate(&mut self) {
        self.status = UserStatus::Inactive;
        self.audit_info.record_update();
    }

    pub fn from_persistent(
        id: UserId,
        email: Email,
        phone_number: PhoneNumber,
        status: UserStatus,
        audit_info: AuditInfo,
    ) -> Self {
        Self {
            id,
            email,
            phone_number,
            status,
            audit_info,
        }
    }
}

//getters
impl User {
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn phone_number(&self) -> &PhoneNumber {
        &self.phone_number
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    pub fn audit_info(&self) -> &AuditInfo {
        &self.audit_info
    }
}
