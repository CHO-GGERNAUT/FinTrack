use crate::application::errors::RepositoryError;

#[async_trait::async_trait]
pub trait UnitOfWork {
    async fn commit(self) -> Result<(), RepositoryError>;
}
