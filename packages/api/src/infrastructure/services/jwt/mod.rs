use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};

use crate::application::{
    dto::Claims,
    error::{ApplicationError, Result},
    services::JwtService,
};

#[derive(Clone)]
pub struct JwtServiceImpl {
    pub secret: String,
}

impl JwtServiceImpl {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}

impl JwtService for JwtServiceImpl {
    fn generate(&self, claims: &Claims) -> Result<String> {
        let token = encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| ApplicationError::JwtError(e.to_string()))?;
        Ok(token)
    }

    fn verify(&self, token: &str) -> Result<TokenData<Claims>> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| ApplicationError::JwtError(e.to_string()))?;

        Ok(data)
    }
}
