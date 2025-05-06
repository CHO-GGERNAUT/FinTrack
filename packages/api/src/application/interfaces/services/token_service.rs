use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub trait TokenService: Send + Sync + 'static {
    fn issue_access_token(&self, user_id: uuid::Uuid) -> Result<String, TokenServiceError>;
    fn issue_refresh_token(&self, user_id: uuid::Uuid) -> Result<String, TokenServiceError>;
    fn verify_token(&self, token: &str) -> Result<Claims, TokenServiceError>;
}

#[derive(Error, Debug)]
pub enum TokenServiceError {
    #[error("Token creation Error({operation}):  {source}")]
    TokenCreationError {
        operation: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error("Token validation Error({operation}):  {source}")]
    TokenValidationError {
        operation: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl TokenServiceError {
    pub fn token_creation_error<E>(operation: &'static str, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TokenServiceError::TokenCreationError {
            operation,
            source: Box::new(err),
        }
    }
    pub fn token_validation_error<E>(operation: &'static str, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TokenServiceError::TokenValidationError {
            operation,
            source: Box::new(err),
        }
    }
}
