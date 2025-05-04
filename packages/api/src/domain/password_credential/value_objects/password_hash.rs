use bcrypt::hash;

use crate::domain::password_credential::errors::PasswordCredentialError;

#[derive(Clone)]
pub struct PasswordHash {
    hash: String,
}
impl PasswordHash {
    pub fn new(plane_password: &str) -> Result<Self, PasswordCredentialError> {
        Ok(Self {
            hash: Self::hash(plane_password)?,
        })
    }

    pub fn verify(&self, provided_password: &str) -> Result<(), PasswordCredentialError> {
        bcrypt::verify(provided_password, &self.hash)
            .map_err(|e| PasswordCredentialError::HashFailed(e.to_string()))
            .and_then(|is_valid| {
                if is_valid {
                    Ok(())
                } else {
                    Err(PasswordCredentialError::InvalidCredentials)
                }
            })
    }

    pub fn from_persistent(hash: &str) -> Self {
        Self {
            hash: hash.to_string(),
        }
    }

    pub fn update_hash(&mut self, new_password: &str) -> Result<(), PasswordCredentialError> {
        self.hash = Self::hash(new_password)?;
        Ok(())
    }
}
impl PasswordHash {
    fn hash(password: &str) -> Result<String, PasswordCredentialError> {
        hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| PasswordCredentialError::HashFailed(e.to_string()))
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
