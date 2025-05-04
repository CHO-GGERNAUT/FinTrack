use crate::{
    application::errors::RepositoryError,
    domain::{password_credential::entities::PasswordCredential, user::value_objects::UserId},
};

#[async_trait::async_trait]
pub trait PasswordCredentialRepository {
    async fn create(
        &self,
        credential: PasswordCredential,
    ) -> Result<PasswordCredential, RepositoryError>;
    async fn find_by_user_id(&self, user_id: UserId) -> Option<PasswordCredential>;
    async fn update(&self, credential: PasswordCredential) -> Result<(), RepositoryError>;
}
