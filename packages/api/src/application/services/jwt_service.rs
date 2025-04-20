use anyhow::Result;
use jsonwebtoken::TokenData;

use crate::application::dto::Claims;

pub trait JwtService: Send + Sync + 'static {
    fn generate(&self, claims: &Claims) -> Result<String>;
    fn verify(&self, token: &str) -> Result<TokenData<Claims>>;
}
