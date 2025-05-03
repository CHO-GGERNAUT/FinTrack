use thiserror::Error; // Import fmt for the example UserId impl

#[derive(Error, Debug)]
pub enum RepositoryError {
    /// The requested resource could not be found (e.g., find_by_id returned no result).
    #[error("{entity_type} with id '{id}' not found")]
    NotFound {
        entity_type: &'static str, // The type of entity (e.g., "User").
        id: String,                // The ID of the entity that was not found.
    },

    /// A data conflict occurred (e.g., Unique constraint violation).
    #[error("Conflict for {entity_type}: {details}")]
    Conflict {
        entity_type: &'static str,
        details: String, // Description of the conflict.
    },

    /// Concurrency control failure (e.g., Optimistic Locking failure).
    #[error("Optimistic locking failure for {entity_type} with id '{id}'")]
    OptimisticLockingFailure {
        entity_type: &'static str,
        id: String,
    },

    /// Unauthorized data access (less common at the repository level).
    #[error("Unauthorized access: {details}")]
    Unauthorized { details: String },

    /// Specific I/O error (example: using #[from]).
    /// Automatically converts from std::io::Error when it occurs due to the #[from] attribute.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    // --- Unexpected or generic storage errors ---
    // Using #[source] stores the underlying cause and automatically implements
    // the source() method of the std::error::Error trait for error chaining.
    // Using Box<dyn std::error::Error + Send + Sync + 'static> allows wrapping any kind of error.
    #[error("Unexpected error during {operation}")]
    Unexpected {
        operation: &'static str, // The operation during which the error occurred.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    // --- Alternatively, an Unexpected variant with just a simple string description ---
    // Can be used instead of the Box<dyn Error...> variant above if detailed chaining isn't needed.
    /*
    #[error("Unexpected error during {operation}: {description}")]
    UnexpectedSimple {
        operation: &'static str,
        description: String,
    }
    */
}
