use anyhow::Result;

use crate::application::{dto::Claims, errors::AuthServiceError};

pub trait AuthService: Send + Sync + 'static {
    fn hash_password(&self, password: &str) -> Result<String, AuthServiceError>;
    fn verify_password(&self, password: &str, hashed: &str) -> Result<(), AuthServiceError>;
    fn issue_access_token(&self, user_id: uuid::Uuid) -> Result<String, AuthServiceError>;
    fn issue_refresh_token(&self, user_id: uuid::Uuid) -> Result<String, AuthServiceError>;
    fn verify_token(&self, token: &str) -> Result<Claims, AuthServiceError>;
}
