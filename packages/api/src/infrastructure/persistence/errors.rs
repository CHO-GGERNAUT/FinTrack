use thiserror::Error;

use crate::domain::shared::errors::DomainValidationRuleError;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error("Failed to reconstitute {0}")]
    ReconstituteFailed(String),
}

impl From<DomainValidationRuleError> for InfraError {
    fn from(e: DomainValidationRuleError) -> Self {
        InfraError::ReconstituteFailed(format!("{}", e))
    }
}
