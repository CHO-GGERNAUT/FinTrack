use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::application::interfaces::services::{
    TokenService,
    token_service::{Claims, TokenServiceError},
};
#[derive(Clone)]
pub struct TokenServiceImpl {
    pub secret: String,
}

impl TokenServiceImpl {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}

type Result<T> = std::result::Result<T, TokenServiceError>;

impl TokenService for TokenServiceImpl {
    fn issue_access_token(&self, user_id: uuid::Uuid) -> Result<String> {
        let claims = Claims {
            sub: user_id.to_string(),
            exp: jsonwebtoken::get_current_timestamp() as usize + 3600, // 1 hour expiration
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| {
            tracing::error!("Token Create Failed {e}");
            TokenServiceError::TokenCreationError {
                operation: "issue_access_token",
                source: Box::new(e),
            }
        })?;
        Ok(token)
    }

    fn issue_refresh_token(&self, user_id: uuid::Uuid) -> Result<String> {
        let claims = Claims {
            sub: user_id.to_string(),
            exp: jsonwebtoken::get_current_timestamp() as usize + 604800, // 7 days expiration
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| TokenServiceError::TokenCreationError {
            operation: "issue_refresh_token",
            source: Box::new(e),
        })?;
        Ok(token)
    }

    fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| TokenServiceError::TokenValidationError {
            operation: "verify_token",
            source: Box::new(e),
        })?;
        Ok(token_data.claims)
    }
}
