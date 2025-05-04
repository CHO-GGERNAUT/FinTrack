use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AuditInfo {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl AuditInfo {
    pub fn record_creation() -> Self {
        let now = Utc::now();
        AuditInfo {
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn record_update(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn record_deletion(&mut self) {
        self.deleted_at = Some(Utc::now());
    }

    pub fn from_persistent(
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        AuditInfo {
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    pub fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }
}
