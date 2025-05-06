use crate::domain::shared::{errors::DomainValidationRuleError, services::Verifier};

#[derive(Clone)]
pub struct PasswordHash {
    hash: String, // Stores the actual hash string
}

impl PasswordHash {
    fn new(hash: String) -> Result<Self, DomainValidationRuleError> {
        if hash.is_empty() {
            return Err(DomainValidationRuleError::InvalidHashFormat(
                "Hash cannot be empty".to_string(),
            ));
        }
        Ok(Self { hash })
    }

    pub fn verify(&self, plain_password: &str, verifier: &impl Verifier) -> bool {
        verifier.verify(plain_password, &self.hash)
    }

    pub fn as_str(&self) -> &str {
        &self.hash
    }
}

impl TryFrom<String> for PasswordHash {
    type Error = DomainValidationRuleError;

    fn try_from(hash: String) -> Result<Self, Self::Error> {
        PasswordHash::new(hash)
    }
}

impl std::fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PasswordHash(*****)")
    }
}

impl std::fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PasswordHash")
            .field("hash", &"<redacted>") // 필드 값을 "<redacted>" 로 표시
            .finish()
    }
}
