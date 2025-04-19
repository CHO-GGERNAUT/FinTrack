pub enum ApplicationError {
    RepositoryError(String),
    ValidationError(String),
    JwtError(String),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            ApplicationError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApplicationError::JwtError(msg) => write!(f, "JWT error: {}", msg),
        }
    }
}
pub type Result<T> = std::result::Result<T, ApplicationError>;
