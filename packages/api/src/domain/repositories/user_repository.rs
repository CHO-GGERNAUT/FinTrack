use super::super::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> anyhow::Result<()>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
}
