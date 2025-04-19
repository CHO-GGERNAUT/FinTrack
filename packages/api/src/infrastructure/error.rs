pub enum InfraError {
    RepositoryError(String),
    NotFound(String),
}

impl std::fmt::Display for InfraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfraError::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            InfraError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}
pub type Result<T> = std::result::Result<T, InfraError>;
