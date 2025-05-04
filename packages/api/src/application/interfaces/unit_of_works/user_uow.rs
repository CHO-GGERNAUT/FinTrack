use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{
    password_credential::repository::PasswordCredentialRepository, user::repository::UserRepository,
};

use super::super::super::errors::RepositoryError;

#[async_trait]
pub trait UserUnitOfWork {
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    fn password_credential_repository(&self) -> Arc<dyn PasswordCredentialRepository>;
    async fn commit(self: Box<Self>) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait UserUnitOfWorkFactory {
    async fn begin(&self) -> Result<Box<dyn UserUnitOfWork>, RepositoryError>;
}
