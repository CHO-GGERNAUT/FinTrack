use crate::domain::{entities::Card, errors::Result};

use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CardRepository {
    async fn create(&mut self, card: &Card) -> Result<Card>;
    async fn delete(&mut self, account_id: Uuid) -> Result<()>;
}
