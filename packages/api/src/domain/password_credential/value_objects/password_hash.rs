use bcrypt::hash;

use crate::domain::password_credential::errors::PasswordCredentialError;

#[derive(Debug, Clone)]
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
