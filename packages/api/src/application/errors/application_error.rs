// src/application/error.rs (예시 경로)
use thiserror::Error;

use super::RepositoryError;
use crate::application::interfaces::services::token_service::TokenServiceError;
use crate::domain::password_credential::errors::PasswordCredentialError;
use crate::domain::user::errors::UserError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("User Already Exists: {0}")]
    UserAlreadyExists(String),

    #[error("Authentication Failed: {0}")]
    AuthenticationFailed(String),

    #[error("Authorization Failed: {0}")]
    AuthorizationFailed(String),

    #[error("Resource Not Found: {0}")]
    NotFound(String),

    #[error("Domain Rule Violation: {source}")]
    DomainError {
        #[from]
        source: DomainErrorWrapper, // 도메인 오류들을 감싸는 래퍼
    },

    #[error("Repository Error: {source}")]
    RepositoryError {
        #[from]
        source: RepositoryError, // RepositoryError 직접 포함
    },

    #[error("Token Generation Error: {0}")]
    TokenError(String),

    #[error("Unexpected Application Error: {0}")]
    Unexpected(String),
    // 다른 필요한 애플리케이션 레벨 오류 추가 가능
}

impl From<TokenServiceError> for ApplicationError {
    fn from(err: TokenServiceError) -> Self {
        ApplicationError::TokenError(err.to_string())
    }
}

#[derive(Error, Debug)]
pub enum DomainErrorWrapper {
    #[error("{0}")]
    User(#[from] UserError),
    #[error("{0}")]
    PasswordCredential(#[from] PasswordCredentialError),
    // 다른 도메인 오류 타입 추가 가능
}

impl From<UserError> for ApplicationError {
    fn from(err: UserError) -> Self {
        ApplicationError::DomainError {
            source: DomainErrorWrapper::User(err),
        }
    }
}
impl From<PasswordCredentialError> for ApplicationError {
    fn from(err: PasswordCredentialError) -> Self {
        ApplicationError::DomainError {
            source: DomainErrorWrapper::PasswordCredential(err),
        }
    }
}
