use crate::domain::{entities::Merchant, errors::DomainError};

use async_trait::async_trait;

#[async_trait]
pub trait MerchantRepository {
    async fn create(&mut self, merchant: &Merchant) -> Result<Merchant, DomainError>;
    async fn find_by_biz_number(&mut self, biz_number: &str) -> Result<Merchant, DomainError>;
}
