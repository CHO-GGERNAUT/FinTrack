use crate::domain::{shared::errors::RepositoryError, user::value_objects::UserId};

use super::entities::Auth;

pub trait AuthRepository {
    fn create(&self, auth: &Auth) -> Result<Auth, RepositoryError>;
    fn find_by_user_id(&self, user_id: UserId) -> Option<Auth>;
}
