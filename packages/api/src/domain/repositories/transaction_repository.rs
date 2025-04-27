use crate::domain::{entities::CardTransaction, errors::DomainError};

use async_trait::async_trait;

#[async_trait]
pub trait CardTransactionRepository {
    async fn create(
        &mut self,
        transaction: &CardTransaction,
    ) -> Result<CardTransaction, DomainError>;
}
