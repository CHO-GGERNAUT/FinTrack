use crate::application::errors::RepositoryError;

use super::entities::Card;

#[async_trait::async_trait]
pub trait CardRepository {
    async fn create(&mut self, card: Card) -> Result<Card, RepositoryError>;
}
