use crate::domain::Result;

use super::super::entities::Card;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CardRepository {
    async fn save(&self, card: Card) -> Result<Card>;
    async fn find_by_account_id(&self, account_id: Uuid) -> Result<Card>;
    async fn find_by_owner_id(&self, owner_id: Uuid) -> Result<Vec<Card>>;
    async fn delete(&self, account_id: Uuid) -> Result<()>;
}
