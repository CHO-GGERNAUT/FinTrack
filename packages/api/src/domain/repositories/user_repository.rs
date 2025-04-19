use crate::domain::{Result, entities::User};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> Result<User>;
    async fn find_by_email(&self, email: &str) -> Result<User>;
}
