use crate::domain::errors::DomainError;

pub type Result<T> = std::result::Result<T, ApplicationError>;
pub enum ApplicationError {
    RepositoryError(String),
    ValidationError(String),
    JwtError(String),
    HashError(String),
    DomainError(String),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            ApplicationError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApplicationError::JwtError(msg) => write!(f, "JWT error: {}", msg),
            ApplicationError::HashError(msg) => write!(f, "Password error: {}", msg),
            ApplicationError::DomainError(msg) => write!(f, "Domain error: {}", msg),
        }
    }
}

impl From<DomainError> for ApplicationError {
    fn from(e: DomainError) -> Self {
        ApplicationError::DomainError(e.to_string())
    }
}

impl From<anyhow::Error> for ApplicationError {
    fn from(e: anyhow::Error) -> Self {
        ApplicationError::DomainError(e.to_string())
    }
}
