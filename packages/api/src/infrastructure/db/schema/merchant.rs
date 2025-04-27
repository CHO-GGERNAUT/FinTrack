use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct MerchantRow {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub name: String,
    pub biz_number: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

impl From<MerchantRow> for crate::domain::entities::Merchant {
    fn from(row: MerchantRow) -> Self {
        Self {
            id: row.id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
            name: row.name,
            biz_number: row.biz_number,
            address: row.address,
            phone: row.phone,
        }
    }
}
