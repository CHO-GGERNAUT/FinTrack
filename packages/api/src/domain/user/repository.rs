use crate::{
    application::errors::RepositoryError,
    domain::user::{
        entities::User,
        value_objects::{Email, UserId},
    },
};

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create(&mut self, user: User) -> Result<User, RepositoryError>;
    async fn find_by_id(&mut self, id: UserId) -> Result<User, RepositoryError>;
    async fn find_by_email(&mut self, email: &Email) -> Result<User, RepositoryError>;
    async fn update(&mut self, user: User) -> Result<User, RepositoryError>;
    async fn delete(&mut self, id: UserId) -> Result<bool, RepositoryError>;
}
