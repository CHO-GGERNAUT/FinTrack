use crate::{
    application::{errors::ApplicationError, interfaces::unit_of_works::UserUnitOfWork},
    domain::{
        password_credential::repository::PasswordCredentialRepository,
        user::{repository::UserRepository, value_objects::Email},
    },
};

use super::super::super::interfaces::services::TokenService;
use std::sync::Arc;

#[derive(Debug)]
pub struct PasswordAuthenticateCommand {
    pub email: String,
    pub password: String,
}

pub struct PasswordAuthenticateResult {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct PasswordAuthenticateHandler<U: UserUnitOfWork> {
    uow: U,
    token_service: Arc<dyn TokenService>,
}

impl<U: UserUnitOfWork> PasswordAuthenticateHandler<U> {
    pub fn new(uow: U, token_service: Arc<dyn TokenService>) -> Self {
        Self { uow, token_service }
    }

    pub async fn execute(
        mut self,
        command: PasswordAuthenticateCommand,
    ) -> Result<PasswordAuthenticateResult, ApplicationError> {
        let email = Email::new(&command.email)?;

        let user = self.uow.user_repository().find_by_email(&email).await?;
        let user_id = user.id().clone();

        let mut credential = self
            .uow
            .password_credential_repository()
            .find_by_user_id(*user.id())
            .await?;
        let verification_result = credential.verify_password(&command.password);
        self.uow
            .password_credential_repository()
            .update(credential)
            .await?;

        self.uow.commit().await?;
        match verification_result {
            Ok(_) => {
                let access_token = self.token_service.issue_access_token(user_id.into())?;
                let refresh_token = self.token_service.issue_refresh_token(user_id.into())?;
                Ok(PasswordAuthenticateResult {
                    access_token,
                    refresh_token,
                })
            }
            Err(e) => Err(e.into()),
        }
    }
}
