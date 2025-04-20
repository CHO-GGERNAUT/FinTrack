use async_trait::async_trait;

use crate::domain::errors::Result;

#[async_trait]
pub trait UnitOfWork {
    async fn commit(self) -> Result<()>;
    async fn rollback(self) -> Result<()>;
}
