use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    /// The requested resource could not be found.
    #[error("{entity_type} with id '{id}' not found")]
    NotFound {
        entity_type: &'static str,
        id: String,
    },

    /// A conflict occurred, preventing the operation (e.g., unique constraint violation).
    #[error("Conflict for {entity_type}: {details}")]
    Conflict {
        entity_type: &'static str,
        details: String, // Can be the conflicting value or a description
    },

    /// Data read from the database is invalid or cannot be mapped.
    #[error("Invalid data retrieved from storage: {source}")]
    InvalidData {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// An error occurred during a database transaction operation (commit, rollback, begin).
    #[error("Database transaction error: {source}")]
    TransactionError {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// An unexpected error occurred during database interaction (connection, query execution, etc.).
    #[error("Database interaction error: {source}")]
    DatabaseError {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// An unexpected error occurred within the repository layer.
    #[error("Unexpected repository error during {operation}: {source}")]
    Unexpected {
        operation: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

impl RepositoryError {
    /// Helper to create a DatabaseError.
    pub fn db<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        RepositoryError::DatabaseError {
            source: Box::new(err),
        }
    }

    /// Helper to create a TransactionError.
    pub fn transaction<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        RepositoryError::TransactionError {
            source: Box::new(err),
        }
    }

    /// Helper to create an Unexpected repository error.
    pub fn unexpected<E>(operation: &'static str, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        RepositoryError::Unexpected {
            operation,
            source: Box::new(err),
        }
    }

    /// Helper to create an InvalidData error.
    pub fn invalid_data<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        RepositoryError::InvalidData {
            source: Box::new(err),
        }
    }
}
