use crate::domain::shared::errors::RepositoryError;

use super::{
    entities::User,
    value_objects::{Email, UserId},
};

pub trait UserRepository {
    fn create(&self, user: User) -> Result<User, RepositoryError>;
    fn get_user_by_id(&self, id: UserId) -> Option<User>;
    fn get_user_by_email(&self, email: &Email) -> Option<User>;
    fn update(&self, user: User) -> Result<User, RepositoryError>;
    fn delete(&self, id: UserId) -> Result<bool, RepositoryError>;
}
