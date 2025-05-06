use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::application::interfaces::services::token_service::{
    Claims, TokenService, TokenServiceError,
};
#[derive(Clone)]
pub struct JwtService {
    pub secret: String,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}

type Result<T> = std::result::Result<T, TokenServiceError>;

impl TokenService for JwtService {
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
        .map_err(|e| TokenServiceError::token_creation_error("issue_access_token", e))?;

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
        .map_err(|e| TokenServiceError::token_creation_error("issue_refresh_token", e))?;
        Ok(token)
    }

    fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| TokenServiceError::token_validation_error("veritfy_token", e))?;
        Ok(token_data.claims)
    }
}
