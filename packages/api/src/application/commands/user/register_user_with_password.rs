use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{errors::ApplicationError, interfaces::unit_of_works::UserUnitOfWorkFactory},
    domain::{
        password_credential::entities::PasswordCredential,
        user::{
            entities::User,
            value_objects::{Email, PhoneNumber},
        },
    },
};

pub struct RegisterUserPasswordCommand {
    pub email: String,
    pub password: String,
    pub phone_number: String,
    // pub name: Option<String>, // 필요시 추가
}

pub struct RegisterUserPasswordResult {
    pub user_id: Uuid,
}

pub struct RegisterUserPasswordHandler {
    user_uow_factory: Arc<dyn UserUnitOfWorkFactory>,
}

impl RegisterUserPasswordHandler {
    pub fn new(user_uow_factory: Arc<dyn UserUnitOfWorkFactory>) -> Self {
        Self { user_uow_factory }
    }

    pub async fn execute(
        &self,
        command: RegisterUserPasswordCommand,
    ) -> Result<RegisterUserPasswordResult, ApplicationError> {
        let email = Email::new(&command.email)?;

        let uow = self.user_uow_factory.begin().await?;

        let user_repo = uow.user_repository();
        let credential_repo = uow.password_credential_repository();

        if user_repo.find_user_by_email(&email).await.is_some() {
            return Err(ApplicationError::UserAlreadyExists(command.email));
        }

        let phone_number = PhoneNumber::new(&command.phone_number)?;
        let user = User::register(email, phone_number);
        let user_id = user.id().clone();
        user_repo.create(user).await?;

        let credential = PasswordCredential::new(user_id, &command.password)?;
        credential_repo.create(credential).await?;
        uow.commit().await?;

        Ok(RegisterUserPasswordResult {
            user_id: user_id.into(),
        })
    }
}
