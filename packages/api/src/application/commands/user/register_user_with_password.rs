use uuid::Uuid;

use crate::{
    application::{errors::ApplicationError, interfaces::unit_of_works::UserUnitOfWork},
    domain::{
        password_credential::{
            entities::PasswordCredential, repository::PasswordCredentialRepository,
        },
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

pub struct RegisterUserPasswordHandler<U: UserUnitOfWork> {
    uow: U,
}

impl<U: UserUnitOfWork> RegisterUserPasswordHandler<U> {
    pub fn new(uow: U) -> Self {
        Self { uow }
    }

    pub async fn execute(
        mut self,
        command: RegisterUserPasswordCommand,
    ) -> Result<RegisterUserPasswordResult, ApplicationError> {
        let email = Email::new(&command.email)?;

        if self
            .uow
            .user_repository()
            .find_by_email(&email)
            .await
            .is_ok()
        {
            return Err(ApplicationError::UserAlreadyExists(command.email));
        }

        let phone_number = PhoneNumber::new(&command.phone_number)?;
        let user = User::register(email, phone_number);
        let user_id = user.id().clone();
        self.uow.user_repository().create(user).await?;

        let credential = PasswordCredential::new(user_id, &command.password)?;
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
