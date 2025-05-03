use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AuditInfo {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl AuditInfo {
    pub fn record_creation() -> Self {
        let now = Utc::now();
        AuditInfo {
            created_at: now,
            updated_at: now,
        }
    }

    pub fn record_update(self) -> Self {
        AuditInfo {
            created_at: self.created_at,
            updated_at: Utc::now(),
        }
    }

    pub fn from_persistent(created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Self {
        AuditInfo {
            created_at,
            updated_at,
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
