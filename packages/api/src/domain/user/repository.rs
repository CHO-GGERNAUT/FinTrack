use crate::{
    application::errors::RepositoryError,
    domain::user::{
        entities::User,
        value_objects::{Email, UserId},
    },
};

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create(&self, user: User) -> Result<User, RepositoryError>;
    async fn find_user_by_id(&self, id: UserId) -> Option<User>;
    async fn find_user_by_email(&self, email: &Email) -> Option<User>;
    async fn update(&self, user: User) -> Result<User, RepositoryError>;
    async fn delete(&self, id: UserId) -> Result<bool, RepositoryError>;
}
