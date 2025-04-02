use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MerchantImportDto {
    pub name: String,
    pub business_number: String,
}

impl MerchantImportDto {
    pub fn new(name: String, business_number: String) -> Self {
        Self {
            name,
            business_number,
        }
    }
}
