pub enum DomainError {
    RepositoryError(String),
    ValidationError(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            DomainError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}
pub type Result<T> = std::result::Result<T, DomainError>;
