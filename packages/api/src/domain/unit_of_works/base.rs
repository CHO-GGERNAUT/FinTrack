use async_trait::async_trait;

use crate::domain::errors::DomainError;

#[async_trait]
pub trait UnitOfWork {
    async fn commit(self) -> Result<(), DomainError>;
    async fn rollback(self) -> Result<(), DomainError>;
}
