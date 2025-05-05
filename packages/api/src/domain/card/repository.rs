use crate::application::errors::RepositoryError;

use super::{entities::Card, value_objects::CardId};

#[async_trait::async_trait]
pub trait FinanceAccountRepository {
    async fn create(&mut self, card: Card) -> Result<Card, RepositoryError>;
    async fn find_by_id(&mut self, id: CardId) -> Result<Card, RepositoryError>;
    async fn update(&mut self, card: Card) -> Result<Card, RepositoryError>;
    async fn delete(&mut self, id: CardId) -> Result<bool, RepositoryError>;
}
