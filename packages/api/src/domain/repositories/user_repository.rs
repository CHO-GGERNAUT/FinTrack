use crate::domain::{entities::User, errors::Result};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create(&mut self, user: &User) -> Result<User>;
    async fn find_by_email(&mut self, email: &str) -> Result<User>;
}
