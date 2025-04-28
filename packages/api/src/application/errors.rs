use crate::domain::errors::DomainError;

pub enum ApplicationError {
    RepositoryError(String),
    ValidationError(String),
    JwtError(String),
    HashError(String),
    DomainError(String),
    AuthServiceError(String),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            ApplicationError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApplicationError::JwtError(msg) => write!(f, "JWT error: {}", msg),
            ApplicationError::HashError(msg) => write!(f, "Password error: {}", msg),
            ApplicationError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            ApplicationError::AuthServiceError(msg) => write!(f, "AuthService error: {}", msg),
        }
    }
}

impl From<AuthServiceError> for ApplicationError {
    fn from(e: AuthServiceError) -> Self {
        ApplicationError::AuthServiceError(e.to_string())
    }
}
impl From<DomainError> for ApplicationError {
    fn from(e: DomainError) -> Self {
        ApplicationError::DomainError(e.to_string())
    }
}

pub enum AuthServiceError {
    InvalidCredentials,
    TokenCreationError,
    TokenValidationError,
    TokenExpired,
    UnknownError(String),
    JsonWebTokenError(String),
}

impl std::fmt::Display for AuthServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthServiceError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthServiceError::TokenCreationError => write!(f, "Token creation error"),
            AuthServiceError::TokenValidationError => write!(f, "Token validation error"),
            AuthServiceError::TokenExpired => write!(f, "Token expired"),
            AuthServiceError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
            AuthServiceError::JsonWebTokenError(msg) => write!(f, "JWT error: {}", msg),
        }
    }
}
