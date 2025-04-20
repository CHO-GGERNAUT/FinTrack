use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::enums::AccountType;

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub account_type: AccountType,
}
