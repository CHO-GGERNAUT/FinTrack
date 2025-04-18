use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Merchant {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub name: String,
    pub biz_number: String,
    pub category: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateMerchantDto {
    pub name: String,
    pub biz_number: String,
    pub category: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}
pub struct CreateMerchantDtoBuilder {
    pub name: Option<String>,
    pub biz_number: Option<String>,
    pub category: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

impl CreateMerchantDtoBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            biz_number: None,
            category: None,
            address: None,
            phone: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn biz_number(mut self, biz_number: String) -> Self {
        self.biz_number = Some(biz_number);
        self
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }

    pub fn address(mut self, address: String) -> Self {
        self.address = Some(address);
        self
    }

    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }
    pub fn build(self) -> Result<CreateMerchantDto, &'static str> {
        let name = self.name.ok_or("name is required")?;
        let biz_number = self.biz_number.ok_or("biz_number is required")?;
        Ok(CreateMerchantDto {
            name,
            biz_number,
            category: self.category,
            address: self.address,
            phone: self.phone,
        })
    }
}
