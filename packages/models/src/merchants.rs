use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Merchant {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub name: String,
    pub business_number: String,
}

impl Merchant {
    pub fn new(name: String, business_number: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: None,
            updated_at: None,
            deleted_at: None,
            name,
            business_number,
        }
    }
}
