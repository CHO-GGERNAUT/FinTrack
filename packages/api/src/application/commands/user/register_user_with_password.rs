use uuid::Uuid;

use crate::{
    application::{
        errors::{ApplicationError, RepositoryError},
        interfaces::unit_of_works::UserUnitOfWork,
    },
    domain::{
        password_credential::{
            entities::PasswordCredential, repository::PasswordCredentialRepository,
            value_objects::PasswordHash,
        },
        shared::services::Hasher,
        user::{
            entities::User,
            repository::UserRepository,
            value_objects::{Email, PhoneNumber},
        },
    },
};

pub struct RegisterUserPasswordCommand {
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

pub struct RegisterUserPasswordResult {
    pub user_id: Uuid,
}

pub struct RegisterUserPasswordHandler<U: UserUnitOfWork, H: Hasher> {
    uow: U,
    hasher: H,
}

impl<U: UserUnitOfWork, H: Hasher> RegisterUserPasswordHandler<U, H> {
    pub fn new(uow: U, hasher: H) -> Self {
        Self { uow, hasher }
    }

    pub async fn execute(
        mut self,
        command: RegisterUserPasswordCommand,
    ) -> Result<RegisterUserPasswordResult, ApplicationError> {
        let email = Email::try_from(command.email)?;

        if self
            .uow
            .user_repository()
            .find_by_email(&email)
            .await
            .is_ok()
        {
            return Err(RepositoryError::Conflict {
                entity_type: "User",
                details: format!("Email already exists: {}", email.as_str()),
            }
            .into());
        }

        let phone_number = PhoneNumber::try_from(command.phone_number)?;
        let user = User::register(email, phone_number);
        let user_id = user.id().clone();
        self.uow.user_repository().create(user).await?;
        let hash_string = self
            .hasher
            .hash(&command.password)
            .map_err(|e| ApplicationError::internal(e))?;
        let password_hash = PasswordHash::try_from(hash_string)?;

        let credential = PasswordCredential::new(user_id, password_hash)?;
        self.uow
            .password_credential_repository()
            .create(credential)
            .await?;
        self.uow.commit().await?;

        Ok(RegisterUserPasswordResult {
            user_id: user_id.into(),
        })
    }
}
