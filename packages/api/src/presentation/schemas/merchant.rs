use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMerchantRequest {
    pub name: String,
    pub biz_number: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}
