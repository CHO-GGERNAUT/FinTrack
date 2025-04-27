use crate::domain::{entities::Transaction, errors::DomainError};

use async_trait::async_trait;

#[async_trait]
pub trait TransactionRepository {
    async fn create(&mut self, transaction: Transaction) -> Result<Transaction, DomainError>;
}
