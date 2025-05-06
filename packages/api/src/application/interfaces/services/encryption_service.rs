use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Invalid nonce length")]
    InvalidNonceLength,
    #[error("Data format error: {0}")]
    FormatError(String),
}

pub trait EncryptionService {
    fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, EncryptionError>;
    fn decrypt(&self, ciphertext: &[u8]) -> Result<String, EncryptionError>;
}
