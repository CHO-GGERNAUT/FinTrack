use crate::domain::{shared::errors::RepositoryError, user::value_objects::UserId};

use super::{entities::Credential, value_objects::CredentialId};

pub trait AuthRepository {
    fn create(&self, auth: &Credential) -> Result<Credential, RepositoryError>;
    fn find_by_user_id(&self, user_id: UserId) -> Option<Vec<Credential>>;
    fn find_by_id(&self, id: CredentialId) -> Option<Credential>;
}
