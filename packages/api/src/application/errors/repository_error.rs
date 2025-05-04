use thiserror::Error; // Import fmt for the example UserId impl

#[derive(Error, Debug)]
pub enum RepositoryError {
    /// The requested resource could not be found (e.g., find_by_id returned no result).
    #[error("{entity_type} with id '{id}' not found")]
    NotFound {
        entity_type: &'static str,
        id: String,
    },

    #[error("Conflict for {entity_type}: {details}")]
    Conflict {
        entity_type: &'static str,
        details: String,
    },

    #[error("Unauthorized access: {details}")]
    Unauthorized { details: String },

    #[error("Unexpected error during {operation}")]
    Unexpected {
        operation: &'static str, // The operation during which the error occurred.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}
