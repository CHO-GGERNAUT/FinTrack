use super::RepositoryError;
use crate::application::interfaces::services::token_service::TokenServiceError;
use crate::domain::password_credential::errors::PasswordCredentialError;
use crate::domain::user::errors::UserError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Validation Error: {0}")]
    Validation(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Authentication Error: {0}")]
    Authentication(String),

    #[error("Authorization Error: {0}")]
    Authorization(String),

    #[error("Internal Server Error")]
    InternalError {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

impl ApplicationError {
    pub fn internal<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ApplicationError::InternalError {
            source: Box::new(err),
        }
    }
}

impl From<RepositoryError> for ApplicationError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound { .. } => ApplicationError::NotFound(err.to_string()),
            RepositoryError::Conflict { .. } => ApplicationError::Conflict(err.to_string()),
            _ => ApplicationError::internal(err),
        }
    }
}

impl From<TokenServiceError> for ApplicationError {
    fn from(err: TokenServiceError) -> Self {
        match err {
            TokenServiceError::TokenValidationError { .. } => {
                ApplicationError::Authentication(err.to_string())
            }
            _ => ApplicationError::internal(err),
        }
    }
}

impl From<UserError> for ApplicationError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::NotFound(_) => ApplicationError::NotFound(err.to_string()),
            UserError::Conflict(_) => ApplicationError::Conflict(err.to_string()),
            UserError::InvalidEmail(_)
            | UserError::InvalidPhoneNumber(_)
            | UserError::InvalidUserStatus(_) => ApplicationError::Validation(err.to_string()),
            _ => ApplicationError::internal(err),
        }
    }
}

impl From<PasswordCredentialError> for ApplicationError {
    fn from(err: PasswordCredentialError) -> Self {
        match err {
            PasswordCredentialError::InvalidCredentials => {
                ApplicationError::Authentication(err.to_string())
            }
            PasswordCredentialError::AccountLocked => {
                ApplicationError::Authorization(err.to_string())
            }
            PasswordCredentialError::HashFailed(_) => ApplicationError::internal(err),
            _ => ApplicationError::internal(err),
        }
    }
}
