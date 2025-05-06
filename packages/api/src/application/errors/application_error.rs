use thiserror::Error;

use crate::{
    application::interfaces::services::token_service::TokenServiceError,
    domain::{
        password_credential::errors::PasswordCredentialError,
        shared::errors::DomainValidationRuleError, user::errors::UserError,
    },
};

use super::RepositoryError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Validation failed: {0}")]
    Validation(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("Not Found: {0}")]
    NotFound(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("Conflict: {0}")]
    Conflict(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("Authentication failed: {0}")]
    Authentication(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("Internal Server Error: {0}")]
    InternalError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<RepositoryError> for ApplicationError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound { .. } => ApplicationError::not_found(err),
            RepositoryError::Conflict { .. } => ApplicationError::conflict(err),
            e @ RepositoryError::InvalidData { .. }
            | e @ RepositoryError::TransactionError { .. }
            | e @ RepositoryError::DatabaseError { .. }
            | e @ RepositoryError::Unexpected { .. } => ApplicationError::internal(e),
        }
    }
}

impl From<TokenServiceError> for ApplicationError {
    fn from(err: TokenServiceError) -> Self {
        match err {
            TokenServiceError::TokenValidationError { .. } => ApplicationError::authentication(err),
            _ => ApplicationError::internal(err),
        }
    }
}

impl From<DomainValidationRuleError> for ApplicationError {
    fn from(err: DomainValidationRuleError) -> Self {
        ApplicationError::validation(err)
    }
}

impl From<UserError> for ApplicationError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::NotFound { .. } => ApplicationError::not_found(err),
            UserError::Conflict { .. } => ApplicationError::conflict(err),
            _ => ApplicationError::internal(err),
        }
    }
}

impl From<PasswordCredentialError> for ApplicationError {
    fn from(err: PasswordCredentialError) -> Self {
        match err {
            PasswordCredentialError::InvalidCredentials { .. } => ApplicationError::validation(err),
            PasswordCredentialError::AccountLocked { .. } => ApplicationError::validation(err),
            _ => ApplicationError::internal(err),
        }
    }
}
impl ApplicationError {
    pub fn validation<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ApplicationError::Validation(Box::new(err))
    }

    pub fn not_found<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ApplicationError::NotFound(Box::new(err))
    }

    pub fn conflict<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ApplicationError::Conflict(Box::new(err))
    }

    pub fn authentication<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ApplicationError::Authentication(Box::new(err))
    }

    pub fn internal<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ApplicationError::InternalError(Box::new(err))
    }
}
