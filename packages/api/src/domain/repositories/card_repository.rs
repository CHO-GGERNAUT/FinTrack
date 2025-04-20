use crate::domain::{entities::Card, errors::DomainError};

use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CardRepository {
    async fn create(&mut self, card: &Card) -> Result<Card, DomainError>;
    async fn delete(&mut self, account_id: Uuid) -> Result<(), DomainError>;
}
