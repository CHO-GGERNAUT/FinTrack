use crate::domain::{entities::Account, errors::Result};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait AccountRepository {
    async fn create(&mut self, account: &Account) -> Result<Account>;
    async fn delete(&mut self, account_id: Uuid) -> Result<()>;

    async fn find_by_id(&mut self, account_id: Uuid) -> Result<Account>;
}
