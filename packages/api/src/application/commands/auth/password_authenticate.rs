use crate::{
    application::{errors::ApplicationError, interfaces::unit_of_works::UserUnitOfWorkFactory},
    domain::user::value_objects::Email,
};

use super::super::super::interfaces::services::TokenService;
use std::sync::Arc;

pub struct PasswordAuthenticateCommand {
    pub email: String,
    pub password: String,
}

pub struct PasswordAuthenticateResult {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct PasswordAuthenticateHandler {
    user_uow_factory: Arc<dyn UserUnitOfWorkFactory>,
    token_service: Arc<dyn TokenService>,
}

impl PasswordAuthenticateHandler {
    pub fn new(
        user_uow_factory: Arc<dyn UserUnitOfWorkFactory>,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self {
            user_uow_factory,
            token_service,
        }
    }

    pub async fn execute(
        &self,
        command: PasswordAuthenticateCommand,
    ) -> Result<PasswordAuthenticateResult, ApplicationError> {
        let uow = self.user_uow_factory.begin().await?;

        let email = Email::new(&command.email)?;

        let user_repo = uow.user_repository();
        let password_repo = uow.password_credential_repository();

        let user = user_repo.find_user_by_email(&email).await.ok_or_else(|| {
            ApplicationError::AuthenticationFailed(format!("User not found: {email}"))
        })?;
        let user_id = user.id().clone();

        let mut credential = password_repo
            .find_by_user_id(*user.id())
            .await
            .ok_or_else(|| {
                ApplicationError::AuthenticationFailed(format!(
                    "Credential not found for user: {}",
                    user_id
                ))
            })?;

        let verification_result = credential.verify_password(&command.password);
        password_repo.update(credential).await?;

        uow.commit().await?;

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
