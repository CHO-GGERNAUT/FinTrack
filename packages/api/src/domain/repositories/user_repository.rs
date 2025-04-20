use crate::domain::{entities::User, errors::DomainError};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create(&mut self, user: &User) -> Result<User, DomainError>;
    async fn find_by_email(&mut self, email: &str) -> Result<User, DomainError>;
}
