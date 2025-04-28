use bcrypt::{DEFAULT_COST, hash};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::application::{dto::Claims, errors::AuthServiceError, services::AuthService};
#[derive(Clone)]
pub struct AuthServiceImpl {
    pub secret: String,
}

impl AuthServiceImpl {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}

type Result<T> = std::result::Result<T, AuthServiceError>;

impl AuthService for AuthServiceImpl {
    fn hash_password(&self, password: &str) -> Result<String> {
        hash(password, DEFAULT_COST).map_err(|e| AuthServiceError::UnknownError(e.to_string()))
    }

    fn verify_password(&self, password: &str, hashed: &str) -> Result<()> {
        // Implement password verification logic here
        let is_valid = bcrypt::verify(password, hashed).is_ok_and(|is_valid| is_valid);
        if is_valid {
            Ok(())
        } else {
            Err(AuthServiceError::InvalidCredentials)
        }
    }

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
        .map_err(|e| AuthServiceError::JsonWebTokenError(e.to_string()))?;
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
        .map_err(|e| AuthServiceError::JsonWebTokenError(e.to_string()))?;
        Ok(token)
    }

    fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AuthServiceError::JsonWebTokenError(e.to_string()))?;
        Ok(token_data.claims)
    }
}
