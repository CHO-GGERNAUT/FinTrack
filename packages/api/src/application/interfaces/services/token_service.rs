#[derive(Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub trait TokenService {
    fn issue_access_token(&self, user_id: uuid::Uuid) -> Result<String, TokenServiceError>;
    fn issue_refresh_token(&self, user_id: uuid::Uuid) -> Result<String, TokenServiceError>;
    fn verify_token(&self, token: &str) -> Result<Claims, TokenServiceError>;
}

pub enum TokenServiceError {
    TokenCreationError,
    TokenValidationError,
    TokenExpired,
    UnknownError(String),
}

impl std::fmt::Display for TokenServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenServiceError::TokenCreationError => write!(f, "Token creation error"),
            TokenServiceError::TokenValidationError => write!(f, "Token validation error"),
            TokenServiceError::TokenExpired => write!(f, "Token expired"),
            TokenServiceError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}
