use crate::domain::{entities::Account, errors::DomainError};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait AccountRepository {
    async fn create(&mut self, account: &Account) -> Result<Account, DomainError>;
    async fn delete(&mut self, account_id: Uuid) -> Result<(), DomainError>;

    async fn find_by_id(&mut self, account_id: Uuid) -> Result<Account, DomainError>;
}
