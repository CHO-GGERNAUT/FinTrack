use sha3::{Digest, Sha3_256};

use crate::domain::shared::services::{Hasher, HashingError, Verifier};

pub struct BcryptHashService;

impl Hasher for BcryptHashService {
    fn hash(&self, password: &str) -> Result<String, HashingError> {
        let hash_string = bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| HashingError::HashingFailed(Box::new(e)))?;
        Ok(hash_string)
    }
}

impl Verifier for BcryptHashService {
    fn verify(&self, password: &str, hash: &str) -> bool {
        bcrypt::verify(password, hash).unwrap_or(false)
    }
}

pub struct SHA3HashService;

impl Hasher for SHA3HashService {
    fn hash(&self, password: &str) -> Result<String, HashingError> {
        let mut hasher = Sha3_256::new();
        hasher.update(password.as_bytes());

        let result_bytes = hasher.finalize();

        let fingerprint_hex = hex::encode(result_bytes);
        Ok(fingerprint_hex)
    }
}
