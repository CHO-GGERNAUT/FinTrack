use crate::domain::{
    password_credential::repository::PasswordCredentialRepository, user::repository::UserRepository,
};

use super::UnitOfWork;

#[async_trait::async_trait]
pub trait UserUnitOfWork: UnitOfWork {
    type UserRepo<'a>: UserRepository
    where
        Self: 'a;

    type PasswordCredentialRepo<'a>: PasswordCredentialRepository
    where
        Self: 'a;

    fn user_repository(&mut self) -> Self::UserRepo<'_>;
    fn password_credential_repository(&mut self) -> Self::PasswordCredentialRepo<'_>;
}
